use axum::{extract::FromRef, Extension, Router};
use bevy::prelude::{App as BevyApp, MinimalPlugins, Update, World};
use bevy_defer::{AsyncPlugin, AsyncWorld};
use common::PlayerCharacter;
use frontend::app::App;
use leptos::{get_configuration, logging, LeptosOptions};
use leptos_axum::{generate_route_list, LeptosRoutes};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;
use std::thread;
use tokio::sync::oneshot;
use tower_http::cors::{Any, CorsLayer};

mod domain;
mod error;
mod game;
mod handlers;
mod routes;

pub use error::{AppError, Result};
// use domain::game_logic::process_command;
use domain::player::get_simulated_character;
// use routes::ai::ai_routes;
use routes::expert::expert_routes;
use routes::persona::persona_routes;
use routes::player::player_routes;
// use routes::vaam::vaam_routes;

// Define a shared application state
#[derive(Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub pool: Option<PgPool>,
    pub async_world: AsyncWorld,
}

// Implement FromRef<AppState> for LeptosOptions
impl FromRef<AppState> for LeptosOptions {
    fn from_ref(state: &AppState) -> Self {
        state.leptos_options.clone()
    }
}

// Implement FromRef<AppState> for PgPool
impl FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> Self {
        state.pool.clone().expect(
            "Database pool not available. This handler should not be reachable in simulation mode.",
        )
    }
}

fn run_bevy_app(tx: oneshot::Sender<AsyncWorld>) {
    let mut app = BevyApp::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(AsyncPlugin::default_settings());

    let simulated_player = get_simulated_character();
    app.world_mut().spawn(simulated_player);

    let async_world = app.world().resource::<AsyncWorld>().clone();
    let _ = tx.send(async_world);

    app.run();
}

#[tokio::main]
async fn main() {
    logging::log!("Starting Daydream Backend Server...");

    let (tx, rx) = oneshot::channel();

    thread::spawn(move || run_bevy_app(tx));

    let async_world = rx.await.expect("Failed to get AsyncWorld from Bevy");

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
            logging::log!(
                "WARN: DATABASE_URL not found. Running in SIMULATION MODE - No Database."
            );
            None
        }
    };

    // Create the application state
    let app_state = AppState {
        leptos_options,
        pool,
        async_world,
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .merge(player_routes(&app_state))
        .merge(persona_routes(&app_state))
        .merge(vaam_routes(&app_state))
        .merge(ai_routes(&app_state))
        .merge(expert_routes(&app_state))
        .leptos_routes(&app_state, routes, App)
        .layer(cors)
        .with_state(app_state);

    logging::log!("Backend listening on http://{}", &addr);
    axum::serve(tokio::net::TcpListener::bind(&addr).await.unwrap(), app)
        .await
        .unwrap();
}
