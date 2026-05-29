use axum::{extract::FromRef, response::IntoResponse, Extension, Router};
use bevy::prelude::{App as BevyApp, MinimalPlugins, Name, Update};
use common::PlayerCharacter;
// use frontend::app::App; // Unused in SPA mode
use leptos::config::get_configuration;
use leptos::prelude::*;
// use leptos_axum::{generate_route_list, LeptosRoutes}; // Unused in SPA mode
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;
use std::sync::{Arc, RwLock};
use std::thread;
use tokio::sync::oneshot;
use tower_http::cors::{Any, CorsLayer};

mod ai;
mod domain;
mod error;
mod game;
mod handlers;
mod narrative; // [NEW] YouTube video generation pipeline
mod routes;
mod static_assets; // [NEW]

use domain::player::get_simulated_character;
pub use error::{AppError, Result};
use routes::ai_mirror::ai_mirror_routes;
use routes::expert::expert_routes;
use routes::persona::persona_routes;
use routes::player::player_routes;
use routes::research::research_routes;
use static_assets::static_handler;

use crate::game::components::*;
use crate::game::systems::*;

use bevy_yarnspinner::prelude::*;

// Define a shared application state
#[derive(Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub pool: Option<PgPool>,
    pub shared_research_log: Arc<RwLock<ResearchLog>>,
    pub shared_virtues: Arc<RwLock<VirtueTopology>>,
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

fn run_bevy_app(shared_log: Arc<RwLock<ResearchLog>>, shared_virtues: Arc<RwLock<VirtueTopology>>) {
    let mut app = BevyApp::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(bevy::asset::AssetPlugin::default());
    app.add_plugins(YarnSpinnerPlugin::new());

    // Insert Shared Resources
    app.insert_resource(SharedResearchLogResource(shared_log));
    app.insert_resource(SharedVirtuesResource(shared_virtues));

    // Register Systems
    app.add_systems(
        Update,
        (
            update_virtue_topology,
            monitor_cognitive_load,
            log_research_events,
            sync_yarn_to_story_progress,
            sync_ecs_to_shared,
        ),
    );

    let simulated_player = get_simulated_character();

    // Spawn StudentBundle
    app.world_mut().spawn(StudentBundle {
        name: Name::new(simulated_player.name),
        persona: Persona {
            archetype: Archetype::Novice,
            shadow_trait: "None".to_string(),
            projective_dissonance: 0.0,
        },
        virtue_topology: VirtueTopology::default(),
        cognitive_load: CognitiveLoad::default(),
        story_progress: StoryProgress {
            current_quest_id: simulated_player.current_quest_id,
            current_step_id: simulated_player.current_step_id,
            current_step_description: simulated_player.current_step_description,
            history: Vec::new(),
            inventory: simulated_player.inventory,
            quest_flags: simulated_player.quest_flags,
            learned_vocab: simulated_player.learned_vocab,
        },
        research_log: ResearchLog::default(),
        level: Level(1),
        xp: Experience(0),
    });

    app.run();
}


#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("Starting Daydream Backend Server...");

    let shared_research_log = Arc::new(RwLock::new(ResearchLog::default()));
    let shared_virtues = Arc::new(RwLock::new(VirtueTopology::default()));

    let log_clone = shared_research_log.clone();
    let virtues_clone = shared_virtues.clone();

    thread::spawn(move || run_bevy_app(log_clone, virtues_clone));

    let conf = get_configuration(None)
        .map_err(|e| format!("Failed to load Leptos configuration: {}", e))?;
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    // let routes = generate_route_list(App); // Not used in SPA mode

    let pool = match env::var("DATABASE_URL") {
        Ok(database_url) => {
            println!("DATABASE_URL found, connecting to the database...");
            Some(
                PgPoolOptions::new()
                    .max_connections(5)
                    .connect(&database_url)
                    .await
                    .expect("Failed to create database pool"),
            )
        }
        Err(_) => {
            println!("WARN: DATABASE_URL not found. Running in SIMULATION MODE - No Database.");
            None
        }
    };

    // Create the application state
    let app_state = AppState {
        leptos_options,
        pool,
        shared_research_log,
        shared_virtues,
    };

    // SECURITY: Restrict CORS to known origins. Use env var for production override.
    let allowed_origin = env::var("CORS_ORIGIN")
        .unwrap_or_else(|_| "http://localhost:3000".to_string());
    let cors = CorsLayer::new()
        .allow_origin(allowed_origin.parse::<axum::http::HeaderValue>()
            .unwrap_or_else(|_| "http://localhost:3000".parse().expect("static origin")))
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .merge(player_routes(&app_state))
        .merge(persona_routes(&app_state))
        .merge(expert_routes(&app_state))
        .merge(research_routes(&app_state))
        .nest("/api/ai-mirror", ai_mirror_routes())
        // [NEW] Serve static assets for all other routes
        .fallback(static_handler)
        .layer(cors)
        .with_state(app_state);

    println!("Backend listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await
        .map_err(|e| format!("Failed to bind to {}: {}", addr, e))?;
    axum::serve(listener, app).await
        .map_err(|e| format!("Server error: {}", e))?;

    Ok(())
}
