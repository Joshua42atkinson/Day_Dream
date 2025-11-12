use axum::{
    routing::{get, post}, // Added `post`
    Router,
    response::IntoResponse,
    http::StatusCode,
    Json,
};
use std::collections::{HashMap, HashSet};

// --- (IMPROVEMENT) Import Leptos routes ---
// These helpers will allow us to host server functions
use leptos::{get_configuration, logging};
use leptos_axum::{generate_route_list, LeptosRoutes};

// --- (IMPROVEMENT) Import frontend components ---
// This allows the backend to know about the server functions
// defined in the frontend crate.
use frontend::app::App; 

// Import our shared data structures
use common::{
    PlayerCharacter, ReportSummary,
    CHARACTER_TEMPLATES, // Our static list of premade characters
    QUEST_DATA,          // Our static map of quest data
    RACE_DATA_MAP,       // Our static map of race data
    JournalData, VocabEntry,
    ProfileData, CharacterSummary,
    // (IMPROVEMENT) New structs for interactivity
    PlayerCommand, GameTurn,
};

// --- (IMPROVEMENT) New Handler for Submitting Commands ---
/// This function is the new API endpoint for the game loop.
/// It receives a command from the frontend, processes it (simulated),
/// and returns the new game state.
async fn handle_submit_command(
    // Axum deserializes the request JSON into our `PlayerCommand` struct
    Json(payload): Json<PlayerCommand>
) -> impl IntoResponse {
    
    // Get the character state that the frontend sent us
    let mut character = payload.current_character;
    
    // --- Simulation of Game Logic ---
    // (Here we would port all your `app.py` logic for
    // command parsing, quest triggers, AI calls, etc.)
    
    // For now, let's just simulate a simple response
    let ai_narrative = format!(
        "You attempt to '{}'. The world shifts around you. The air smells of ozone and forgotten memories.", 
        payload.command_text
    );
    
    // Simulate finding an item
    character.inventory.push("Strange Cog".to_string());
    // Simulate spending a fate point
    character.fate_points -= 1;
    
    let system_message = Some("You found a 'Strange Cog'!".to_string());
    // --- End Simulation ---

    // Create the response object
    let game_turn = GameTurn {
        player_command: payload.command_text,
        ai_narrative,
        system_message,
        updated_character: character, // Send the modified character back
    };

    (StatusCode::OK, Json(game_turn))
}

// --- API Handlers for GETting page data ---

/// Handler for the main game view and character/quest journal
async fn get_player_character() -> impl IntoResponse {
    let character = get_simulated_character();
    (StatusCode::OK, Json(character))
}

/// Handler for the vocab/report journal
async fn get_journal_data() -> impl IntoResponse {
    // (This function is unchanged from the previous version)
    let character = get_simulated_character(); 
    let mut awl_words = Vec::new();
    awl_words.push(VocabEntry { word: "analyse".to_string(), definition: "To examine something methodically...".to_string() });
    awl_words.push(VocabEntry { word: "approach".to_string(), definition: "A way of dealing with something...".to_string() });

    let mut ai_lists = HashMap::new();
    ai_lists.insert(
        "'Chaos' Context".to_string(),
        vec![VocabEntry { word: "entropy".to_string(), definition: "Lack of order or predictability...".to_string() }]
    );
    
    let data = JournalData {
        awl_words: awl_words,
        ai_word_lists: ai_lists,
        report_summaries: character.report_summaries,
    };
    (StatusCode::OK, Json(data))
}

/// Handler for the profile page
async fn get_profile_data() -> impl IntoResponse {
    // (This function is unchanged from the previous version)
    let characters = vec![
        CharacterSummary {
            id: "char_sim_totem_001".to_string(),
            name: "Totem".to_string(),
            race: "Sasquatch".to_string(),
            class_name: "Soldier".to_string(),
        },
        CharacterSummary {
            id: "char_sim_bolt_002".to_string(),
            name: "Bolt".to_string(),
            race: "Android".to_string(),
            class_name: "Inventor".to_string(),
        }
    ];

    let data = ProfileData {
        email: "player@daydream.com".to_string(),
        has_premium: true, // Simulate premium access
        characters: characters,
        premade_characters: CHARACTER_TEMPLATES.to_vec(),
    };
    (StatusCode::OK, Json(data))
}


// --- Main Server Function ---
#[tokio::main]
async fn main() {
    logging::log!("Starting Daydream Backend Server...");

    // --- (IMPROVEMENT) Load Leptos Config ---
    // This loads the `frontend/Cargo.toml` (which contains a [package.metadata.leptos] section)
    // to configure server-side rendering, etc.
    let conf = get_configuration(Some("frontend/Cargo.toml")).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App); // Get all routes from the <App/> component

    // --- (IMPROVEMENT) CORS Layer ---
    // This allows requests from our frontend (running on :3000)
    // to our backend (running on :3001)
    use tower_http::cors::{CorsLayer, Any};
    let cors = CorsLayer::new()
        .allow_origin(Any) // Allow any origin for development
        .allow_methods(Any)
        .allow_headers(Any);

    // Build our application router
    let app = Router::new()
        .route("/api/profile_data", get(get_profile_data))
        .route("/api/player_character", get(get_player_character))
        .route("/api/journal_data", get(get_journal_data))
        // --- (IMPROVEMENT) New route for game logic ---
        .route("/api/submit_command", post(handle_submit_command))
        
        // --- (IMPROVEMENT) Leptos Routes ---
        // This line adds all the routes needed for Leptos server functions.
        // It's separate from the file serving, which `cargo-leptos` handles.
        .leptos_routes(&leptos_options, routes, App)
        .layer(cors) // Apply the CORS middleware
        .with_state(leptos_options);

    // Run the server
    logging::log!("Backend listening on http://{}", &addr);
    axum::serve(tokio::net::TcpListener::bind(&addr).await.unwrap(), app)
        .await
        .unwrap();
}


// --- Simulated Database Function ---
// This creates a "Totem" character on-the-fly for testing.
// (Unchanged from previous version)
fn get_simulated_character() -> PlayerCharacter {
    let template = CHARACTER_TEMPLATES.get(0).cloned().unwrap();
    let base_fate_points = 1;
    let race_data = RACE_DATA_MAP.get(&template.race_name).unwrap();
    let mut quest_title = "Starting Quest".to_string();
    let mut step_desc = "Begin your journey.".to_string();
    let start_quest = QUEST_DATA.get(&template.starting_quest_id);
    let (start_step_id, start_quest_id) = if let Some(quest) = start_quest {
        quest_title = quest.title.clone();
        if let Some(step) = quest.steps.get(&quest.starting_step) {
            step_desc = step.description.clone();
        }
        (Some(quest.starting_step.clone()), Some(template.starting_quest_id.clone()))
    } else {
        (None, None)
    };
    let character = PlayerCharacter {
        id: "char_sim_totem_001".to_string(),
        user_id: "user_sim_001".to_string(),
        name: template.name,
        race_name: template.race_name,
        class_name: template.class_name,
        philosophy_name: template.philosophy_name,
        boon: template.boon,
        backstory: template.backstory,
        abilities: race_data.abilities.clone(),
        aspects: vec!["Weapon of Choice".to_string()],
        inventory: vec!["Rations".to_string(), "Trembling Porcupine".to_string()],
        quest_flags: HashMap::new(),
        current_location: "Thetopia - Town Square".to_string(),
        current_quest_id: start_quest_id,
        current_step_id: start_step_id,
        current_quest_title: quest_title,
        current_step_description: step_desc,
        fate_points: base_fate_points + race_data.fate_point_mod,
        learned_vocab: HashSet::new(),
        report_summaries: vec![
            ReportSummary {
                chapter: 1,
                summary: "**Chapter Complete!**\n* Comprehension Score: 8.5 / 10\n* Player XP Gained: +75".to_string(),
                comprehension_score: 8.5,
                player_xp_gained: 75,
            }
        ],
    };
    character
}