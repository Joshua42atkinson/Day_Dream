use axum::{Router, extract::FromRef};
use bevy::prelude::{App as BevyApp, MinimalPlugins, Update, World};
use common::PlayerCharacter;
use leptos::{get_configuration, logging, LeptosOptions};
use leptos_axum::{generate_route_list, LeptosRoutes};
use std::thread;
use tokio::sync::{mpsc, oneshot};
use frontend::app::App;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;
use tower_http::cors::{Any, CorsLayer};

mod handlers;
mod routes;
mod domain;
mod error;

pub use error::{AppError, Result};
use domain::game_logic::process_command;
use domain::player::get_simulated_character;
use routes::player::player_routes;
use routes::persona::persona_routes;
use routes::vaam::vaam_routes;
use tokio::sync::mpsc::Receiver;

// Define a shared application state
#[derive(Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub pool: Option<PgPool>,
    pub tx: mpsc::Sender<(String, oneshot::Sender<PlayerCharacter>)>,
}

// Implement FromRef<AppState> for LeptosOptions
impl FromRef<AppState> for LeptosOptions {
    fn from_ref(state: &AppState) -> Self {
        state.leptos_options.clone()
    }
}

fn run_bevy_app(
    mut rx: Receiver<(String, oneshot::Sender<PlayerCharacter>)>,
) {
    let mut app = BevyApp::new();
    app.add_plugins(MinimalPlugins)
        .add_systems(Update, move |world: &mut World| {
            process_command(world, &mut rx);
        });

    let simulated_player = get_simulated_character();
    app.world_mut().spawn(simulated_player);

    app.run();
}

#[tokio::main]
async fn main() {
    logging::log!("Starting Daydream Backend Server...");

    let (tx, rx) =
        mpsc::channel::<(String, oneshot::Sender<PlayerCharacter>)>(100);

    thread::spawn(move || run_bevy_app(rx));

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr.clone();
    let routes = generate_route_list(App);

    let pool = match env::var("DATABASE_URL") {
        Ok(database_url) => {
            logging::log!("DATABASE_URL found, connecting to the database...");
            Some(
                PgPoolOptions::new()
                    .max_connections(5)
                    .connect(&database_url)
                    .await
                    .expect("Failed to create database pool"),
            )
        }
        Err(_) => {
            logging::log!("WARN: DATABASE_URL not found. Running in SIMULATION MODE - No Database.");
            None
        }
    };

    // Create the application state
    let app_state = AppState {
        leptos_options,
        pool,
        tx,
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .merge(player_routes(&app_state))
        .merge(persona_routes(&app_state))
        .merge(vaam_routes(&app_state))
        .leptos_routes(&app_state, routes, App)
        .layer(cors)
        .with_state(app_state);

    logging::log!("Backend listening on http://{}", &addr);
    axum::serve(tokio::net::TcpListener::bind(&addr).await.unwrap(), app)
        .await
        .unwrap();
}
