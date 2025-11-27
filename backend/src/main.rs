#![allow(dead_code, unused_imports, unused_variables, unused_mut)]
use axum::{extract::FromRef, response::IntoResponse, Router};
use bevy::prelude::{App as BevyApp, MinimalPlugins, Name, Update};
use leptos::config::get_configuration;
use leptos::prelude::*;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;
use std::sync::{Arc, RwLock};
use std::thread;
use tower_http::cors::{Any, CorsLayer};

mod ai;
mod core; // Plugin system traits
mod domain;
mod error;
mod game;
mod handlers;
mod plugins; // Plugin registry and implementations
mod routes;
mod services; // Model Manager and Pete AI
mod static_assets;

use domain::player::get_simulated_character;
pub use error::{AppError, Result};
use routes::ai_mirror::ai_mirror_routes;
use routes::expert::expert_routes;
use routes::persona::persona_routes;
use routes::player::player_routes;
use routes::research::research_routes;
// use routes::weigh_station_routes::weigh_station_routes; // [NEW] - Disabled until Llama model is available
use static_assets::Assets; // [NEW]

// use crate::ai::llm::llama_engine::{Llama3Model, ModelConfig}; // Disabled
// use crate::handlers::weigh_station::WeighStation; // [NEW] // [NEW] - Disabled

use crate::game::components::*;
use crate::game::systems::*;

use bevy_yarnspinner::prelude::*;

use crate::ai::{conversation_memory::ConversationMemory, socratic_engine::SocraticEngine};

// Define a shared application state
#[derive(Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub pool: Option<PgPool>,
    pub shared_research_log: Arc<RwLock<ResearchLog>>,
    pub shared_virtues: Arc<RwLock<VirtueTopology>>,
    pub gemma_server: Arc<crate::ai::llm::gemma_server::Gemma27BServer>,
    pub conversation_memory: Arc<ConversationMemory>,
    pub socratic_engine: Arc<tokio::sync::RwLock<SocraticEngine>>,
    pub model_manager: Arc<tokio::sync::Mutex<crate::services::model_manager::ModelManager>>,
    pub pete_assistant: Arc<crate::services::pete::PeteAssistant>,
    // pub weigh_station: Arc<tokio::sync::Mutex<WeighStation>>, // [NEW] - Disabled
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

fn run_bevy_app(
    shared_log: Arc<RwLock<ResearchLog>>,
    shared_virtues: Arc<RwLock<VirtueTopology>>,
    download_inbox: DownloadCommandInbox,
    download_state: SharedDownloadStateResource,
) {
    let mut app = BevyApp::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(bevy::asset::AssetPlugin::default());
    app.add_plugins(bevy::audio::AudioPlugin::default());
    app.add_plugins(YarnSpinnerPlugin::new());

    // Register Events
    app.add_event::<PlayWhistleEvent>();
    app.add_event::<StartDownloadEvent>();
    app.add_event::<DownloadProgressEvent>();

    // Insert Shared Resources
    app.insert_resource(SharedResearchLogResource(shared_log));
    app.insert_resource(SharedVirtuesResource(shared_virtues));
    app.insert_resource(download_inbox);
    app.insert_resource(download_state);

    // Register Systems
    app.add_systems(
        Update,
        (
            update_virtue_topology,
            monitor_cognitive_load,
            log_research_events,
            sync_yarn_to_story_progress,
            sync_ecs_to_shared,
            whistle_system,
            download_manager_system,
            progress_update_system,
            sync_inbox_to_events, // [NEW]
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

// [NEW] Handler for static assets
async fn static_handler(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    let mut path = uri.path().trim_start_matches('/').to_string();

    if path.is_empty() {
        path = "index.html".to_string();
    }

    match Assets::get(&path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            (
                [(axum::http::header::CONTENT_TYPE, mime.as_ref())],
                content.data,
            )
                .into_response()
        }
        None => {
            if path.contains('.') {
                return axum::http::StatusCode::NOT_FOUND.into_response();
            }
            // Fallback to index.html for SPA routing
            match Assets::get("index.html") {
                Some(content) => {
                    let mime = mime_guess::from_path("index.html").first_or_octet_stream();
                    (
                        [(axum::http::header::CONTENT_TYPE, mime.as_ref())],
                        content.data,
                    )
                        .into_response()
                }
                None => axum::http::StatusCode::NOT_FOUND.into_response(),
            }
        }
    }
}

#[tokio::main]
async fn main() {
    println!("Starting Ask Pete Backend Server...");

    let shared_research_log = Arc::new(RwLock::new(ResearchLog::default()));
    let shared_virtues = Arc::new(RwLock::new(VirtueTopology::default()));

    // Initialize Download Resources
    let download_inbox = DownloadCommandInbox(Arc::new(RwLock::new(Vec::new())));
    let download_state = SharedDownloadStateResource(Arc::new(RwLock::new(None)));

    let log_clone = shared_research_log.clone();
    let virtues_clone = shared_virtues.clone();
    let inbox_clone = download_inbox.clone();
    let state_clone = download_state.clone();

    thread::spawn(move || run_bevy_app(log_clone, virtues_clone, inbox_clone, state_clone));

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr.clone();

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

    let gemma_server = Arc::new(crate::ai::llm::gemma_server::Gemma27BServer::new(
        std::path::PathBuf::from("frontend/public/models/gemma-3-27B-it-QAT-Q4_0.gguf"),
    ));

    // Initialize AI Mirror components
    let conversation_memory = Arc::new(match pool.as_ref() {
        Some(p) => ConversationMemory::new(p.clone(), 100),
        None => {
            println!("Using in-memory conversation storage for AI Mirror");
            ConversationMemory::new_in_memory(100)
        }
    });

    let mut socratic_engine_instance = SocraticEngine::new(conversation_memory.clone());
    socratic_engine_instance.set_gemma_server(gemma_server.clone());

    let socratic_engine = Arc::new(tokio::sync::RwLock::new(socratic_engine_instance));

    println!("AI Mirror Socratic Engine initialized and connected to Gemma 27B");

    let model_manager = Arc::new(tokio::sync::Mutex::new(
        crate::services::model_manager::ModelManager::new()
            .expect("Failed to initialize ModelManager"),
    ));

    let pete_assistant = Arc::new(
        crate::services::pete::PeteAssistant::new().expect("Failed to initialize PeteAssistant"),
    );

    // Initialize Weigh Station - DISABLED until Llama model is downloaded
    // println!("Loading Llama 3.2 Model for Weigh Station...");
    // let llama_config = ModelConfig::default();
    // let llama_model = Llama3Model::load(llama_config).expect("Failed to load Llama 3.2 model");

    // let weigh_station = Arc::new(tokio::sync::Mutex::new(WeighStation::new(
    //     pool.clone().expect("Database required for Weigh Station"),
    //     llama_model,
    // )));

    // Create the application state
    let app_state = AppState {
        leptos_options,
        pool,
        shared_research_log,
        shared_virtues,
        gemma_server,
        conversation_memory,
        socratic_engine,
        model_manager,
        pete_assistant,
        // weigh_station, // Disabled
    };

    // Create Model App State
    let model_app_state = crate::routes::model_routes::ModelAppState {
        download_inbox,
        download_state,
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .merge(player_routes(&app_state))
        .merge(persona_routes(&app_state))
        .merge(expert_routes(&app_state))
        .merge(research_routes(&app_state))
        .merge(crate::routes::pete::pete_routes(&app_state))
        .nest("/api/ai-mirror", ai_mirror_routes())
        .nest(
            "/api/models",
            crate::routes::model_routes::model_routes().with_state(model_app_state),
        ) // [NEW]
        // .nest("/api/weigh_station", weigh_station_routes()) // [NEW] - Disabled
        .layer(cors)
        .with_state(app_state) // Apply state BEFORE fallback
        .fallback(static_handler); // Fallback LAST so it doesn't catch API routes

    println!("Backend listening on http://{}", &addr);
    axum::serve(tokio::net::TcpListener::bind(&addr).await.unwrap(), app)
        .await
        .unwrap();
}
