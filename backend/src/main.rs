use axum::{Extension, Router};
use bevy::prelude::{App as BevyApp, MinimalPlugins, Update, World, Resource};
use common::PlayerCharacter;
use leptos::{get_configuration, logging};
use leptos_axum::{generate_route_list, LeptosRoutes};
use std::thread;
use tokio::sync::{mpsc, oneshot};
use frontend::app::App;
use common::Command;

mod handlers;
mod routes;
mod domain;

use domain::game_logic::process_command;
use domain::player::get_simulated_character;
use domain::content_loader::{load_game_data, GameData};
use routes::player::player_routes;
use routes::persona::persona_routes;
use tokio::sync::mpsc::Receiver;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[derive(Resource, Clone)]
pub struct AppState {
    pub game_data: GameData,
}

fn run_bevy_app(
    mut rx: Receiver<(Command, oneshot::Sender<PlayerCharacter>)>,
    game_data: GameData,
) {
    let mut app = BevyApp::new();
    app.add_plugins(MinimalPlugins)
        .insert_resource(AppState { game_data: game_data.clone() })
        .add_systems(Update, move |world: &mut World| {
            process_command(world, &mut rx);
        });

    // Spawn a simulated player entity for testing
    let simulated_player = get_simulated_character(&game_data);
    app.world_mut().spawn(simulated_player);

    app.run();
}

#[tokio::main]
async fn main() {
    logging::log!("Starting Daydream Backend Server...");

    let game_data = load_game_data().expect("Failed to load game data");

    // Create a channel for sending commands from Axum to Bevy
    let (tx, rx) =
        mpsc::channel::<(Command, oneshot::Sender<PlayerCharacter>)>(100);

    // Spawn the Bevy app in a separate thread
    thread::spawn(move || run_bevy_app(rx, game_data));

    let conf = get_configuration(Some("frontend/Cargo.toml")).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    use tower_http::cors::{Any, CorsLayer};
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create database pool");

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        .merge(player_routes(&leptos_options))
        .merge(persona_routes(&leptos_options))
        .layer(cors)
        .layer(Extension(tx))
        .layer(Extension(pool))
        .with_state(leptos_options);

    logging::log!("Backend listening on http://{}", &addr);
    axum::serve(tokio::net::TcpListener::bind(&addr).await.unwrap(), app)
        .await
        .unwrap();
}
