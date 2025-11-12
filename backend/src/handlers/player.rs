use axum::{
    response::IntoResponse,
    http::StatusCode,
    Json,
    extract::State,
};
use leptos::LeptosOptions;
use std::collections::{HashMap};

// Import our shared data structures
use common::{
    CHARACTER_TEMPLATES, // Our static list of premade characters
    JournalData, VocabEntry,
    ProfileData, CharacterSummary,
    // (IMPROVEMENT) New structs for interactivity
    PlayerCommand, GameTurn,
};

use crate::domain::player::get_simulated_character;

// --- (IMPROVEMENT) New Handler for Submitting Commands ---
/// This function is the new API endpoint for the game loop.
/// It receives a command from the frontend, processes it (simulated),
/// and returns the new game state.
pub async fn handle_submit_command(
    State(_options): State<LeptosOptions>,
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
pub async fn get_player_character(State(_options): State<LeptosOptions>) -> impl IntoResponse {
    let character = get_simulated_character();
    (StatusCode::OK, Json(character))
}

/// Handler for the vocab/report journal
pub async fn get_journal_data(State(_options): State<LeptosOptions>) -> impl IntoResponse {
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
pub async fn get_profile_data(State(_options): State<LeptosOptions>) -> impl IntoResponse {
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
