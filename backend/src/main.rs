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
mod antigravity;
mod core; // Plugin system traits
mod domain;
mod error;
mod game;
mod handlers;
mod plugins; // Plugin registry and implementations
mod routes;
mod services; // Model Manager and Pete AI
mod state;
mod static_assets;

use crate::state::AppState;
use domain::player::get_simulated_character;
pub use error::{AppError, Result};
use routes::ai_mirror::ai_mirror_routes;
use routes::expert::expert_routes;
use routes::persona::persona_routes;
use routes::player::player_routes;
use routes::research::research_routes;
use routes::weigh_station_routes::weigh_station_routes; // [NEW] - Enabled
use static_assets::Assets; // [NEW]

use crate::ai::llm::gemma_engine::{GemmaConfigWrapper, GemmaModel}; // Enabled
use crate::handlers::weigh_station::WeighStation; // [NEW] // [NEW] - Enabled

use crate::game::components::*;
use crate::game::systems::*;

use bevy_yarnspinner::prelude::*;

use crate::ai::{conversation_memory::ConversationMemory, socratic_engine::SocraticEngine};

fn run_bevy_app(
    shared_log: Arc<RwLock<ResearchLog>>,
    shared_virtues: Arc<RwLock<VirtueTopology>>,
    shared_physics: SharedPhysicsResource, // [NEW]
    download_inbox: DownloadCommandInbox,
    download_state: SharedDownloadStateResource,
    pete_command_inbox: PeteCommandInbox,
    pete_response_outbox: PeteResponseOutbox,
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
    app.add_event::<AskPeteEvent>(); // [NEW]
    app.add_event::<PeteResponseEvent>(); // [NEW]

    // Insert Shared Resources
    app.insert_resource(SharedResearchLogResource(shared_log));
    app.insert_resource(SharedVirtuesResource(shared_virtues));
    app.insert_resource(shared_physics); // [NEW]
    app.insert_resource(download_inbox);
    app.insert_resource(download_state);
    app.insert_resource(pete_command_inbox.clone());
    app.insert_resource(pete_response_outbox.clone());

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
            sync_inbox_to_events,
            calculate_train_velocity,
            sync_pete_bridge,
            track_student_miles,
            sync_physics_to_shared, // [NEW]
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
        mass: Mass(10.0),                 // Default mass (Cargo Weight)
        engine_power: EnginePower(100.0), // Default power (Willpower)
        velocity: TrainVelocity(0.0),     // Starts stationary
        miles: StudentMiles::default(),   // [NEW]
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

    // Initialize Pete Resources
    let pete_command_inbox = PeteCommandInbox(Arc::new(RwLock::new(Vec::new())));
    let pete_response_outbox = PeteResponseOutbox(Arc::new(RwLock::new(Vec::new())));

    // Initialize Shared Physics Resource
    let shared_physics = SharedPhysicsResource(Arc::new(RwLock::new(PhysicsState::default())));

    let log_clone = shared_research_log.clone();
    let virtues_clone = shared_virtues.clone();
    let inbox_clone = download_inbox.clone();
    let state_clone = download_state.clone();
    let pete_inbox_clone = pete_command_inbox.clone();
    let pete_outbox_clone = pete_response_outbox.clone();
    let physics_clone = shared_physics.clone(); // [NEW]

    thread::spawn(move || {
        run_bevy_app(
            log_clone,
            virtues_clone,
            physics_clone, // [NEW]
            inbox_clone,
            state_clone,
            pete_inbox_clone,
            pete_outbox_clone,
        )
    });

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    // Cloud Run Support: Override site_addr if PORT env var is set
    let addr = if let Ok(port) = env::var("PORT") {
        format!("0.0.0.0:{}", port).parse().unwrap()
    } else {
        leptos_options.site_addr.clone()
    };

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

    // Initialize Gemini 3 Ultra Client
    let gemini_config = crate::ai::llm::gemini_client::GeminiConfig::default();
    let gemini_client = crate::ai::llm::gemini_client::GeminiClient::new(gemini_config);

    // Initialize AI Mirror components
    let conversation_memory = Arc::new(match pool.as_ref() {
        Some(p) => ConversationMemory::new(p.clone(), 100),
        None => {
            println!("Using in-memory conversation storage for AI Mirror");
            ConversationMemory::new_in_memory(100)
        }
    });

    let mut socratic_engine_instance = SocraticEngine::new(conversation_memory.clone());
    socratic_engine_instance.set_gemini_client(gemini_client);

    // Initialize Antigravity Client (Enterprise Bridge)
    let antigravity_client = crate::antigravity::AntigravityClient::new();
    socratic_engine_instance.set_antigravity_client(antigravity_client);

    let socratic_engine = Arc::new(tokio::sync::RwLock::new(socratic_engine_instance));

    println!("AI Mirror Socratic Engine initialized and connected to Gemini 3 Ultra");

    let model_manager = Arc::new(tokio::sync::Mutex::new(
        crate::services::model_manager::ModelManager::new()
            .expect("Failed to initialize ModelManager"),
    ));

    // [NEW] Auto-download "pete" model if missing
    {
        let mut manager = model_manager.lock().await;
        if !manager.has_model("pete") {
            println!("'pete' model not found. Starting automatic download...");
            let models = crate::services::model_manager::ModelManager::list_available_models();
            if let Some(pete_def) = models.iter().find(|m| m.alias == "pete") {
                match manager.download_model(pete_def).await {
                    Ok(path) => println!("Successfully downloaded 'pete' model to {:?}", path),
                    Err(e) => eprintln!("Failed to download 'pete' model: {}", e),
                }
            }
        } else {
            println!("'pete' model found. Ready for inference.");
        }
    }

    let pete_assistant = Arc::new(
        crate::services::pete::PeteAssistant::new().expect("Failed to initialize PeteAssistant"),
    );

    // Initialize Weigh Station
    println!("Loading Gemma 3 Model for Weigh Station...");
    let gemma_config = GemmaConfigWrapper::default();
    let weigh_station = match GemmaModel::load(gemma_config) {
        Ok(gemma_model) => {
            println!("✅ Gemma 3 Model Loaded Successfully.");
            if let Some(db_pool) = pool.clone() {
                Some(Arc::new(tokio::sync::Mutex::new(WeighStation::new(
                    db_pool,
                    gemma_model,
                ))))
            } else {
                println!("⚠️ Database not available, Weigh Station disabled.");
                None
            }
        }
        Err(e) => {
            println!(
                "⚠️ Failed to load Gemma 3 model: {}. Weigh Station disabled.",
                e
            );
            None
        }
    };

    // Create the application state
    let app_state = AppState {
        leptos_options,
        pool,
        shared_research_log,
        shared_virtues,
        // gemma_server,
        conversation_memory,
        socratic_engine,
        model_manager,
        pete_assistant,
        pete_command_inbox,   // [NEW]
        pete_response_outbox, // [NEW]
        shared_physics,       // [NEW]
        weigh_station,        // Enabled
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
        .merge(crate::routes::recharge::recharge_routes(&app_state))
        .merge(crate::routes::simulation::simulation_routes())
        .merge(ai_mirror_routes())
        .nest(
            "/api/models",
            crate::routes::model_routes::model_routes().with_state(model_app_state),
        )
        // .merge(crate::routes::debug::debug_routes())
        .merge(crate::routes::knowledge::knowledge_routes())
        .nest("/api/weigh_station", weigh_station_routes()) // [NEW] - Enabled
        .layer(cors)
        .with_state(app_state) // Apply state BEFORE fallback
        .fallback(static_handler); // Fallback LAST so it doesn't catch API routes

    println!("Backend listening on http://{}", &addr);
    axum::serve(tokio::net::TcpListener::bind(&addr).await.unwrap(), app)
        .await
        .unwrap();
}
