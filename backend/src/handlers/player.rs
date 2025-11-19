use axum::{
    response::IntoResponse,
    http::StatusCode,
    Json,
    extract::State,
    Extension,
};
use leptos::LeptosOptions;
use std::collections::{HashMap};
use tokio::sync::{mpsc, oneshot};
use sqlx::PgPool;

// Import from our project
use crate::AppError;
use crate::domain::player::get_simulated_character;

// Import our shared data structures
use common::{
    PlayerCharacter,
    CHARACTER_TEMPLATES, // Our static list of premade characters
    JournalData, VocabEntry,
    ProfileData, CharacterSummary,
    // (IMPROVEMENT) New structs for interactivity
    PlayerCommand, GameTurn,
    PlayerProfile,
};


// --- (IMPROVEMENT) New Handler for Submitting Commands ---
/// This function is the new API endpoint for the game loop.
/// It receives a command from the frontend, processes it (simulated),
/// and returns the new game state.
pub async fn handle_submit_command(
    State(_options): State<LeptosOptions>,
    Extension(tx): Extension<mpsc::Sender<(String, oneshot::Sender<PlayerCharacter>)>>,
    Json(payload): Json<PlayerCommand>,
) -> impl IntoResponse {
    let (one_tx, one_rx) = oneshot::channel();
    let command = payload.command_text.clone();
    tx.send((command, one_tx)).await.unwrap();

    let updated_character = one_rx.await.unwrap();

    let (ai_narrative, system_message) = if payload.current_character.current_step_id != updated_character.current_step_id {
        (
            updated_character.current_step_description.clone(),
            Some("Quest step advanced.".to_string()),
        )
    } else {
        (
            "Your command has been processed, but nothing seems to have changed.".to_string(),
            None,
        )
    };

    let game_turn = GameTurn {
        player_command: payload.command_text,
        ai_narrative,
        system_message,
        updated_character,
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

// Define a custom Result alias for cleaner signatures
use crate::Result;

// This saves us from typing Result<T, AppError> everywhere
/// Get Player Profile
/// Notice the clean return type: Result<Json<PlayerProfile>>
pub async fn get_player_profile(
    State(pool): State<PgPool>,
    // In the future, we will add Auth extractor here
) -> Result<Json<PlayerProfile>> {
    // Simulation of a DB call
    // If sqlx fails, the '?' operator automatically converts it to AppError::DatabaseError
    // which automatically logs it and returns a safe 500 to the user.
    /* let player = sqlx::query_as!(PlayerProfile, "SELECT * FROM players WHERE id = $1", 1)
        .fetch_optional(&pool)
        .await?
        .ok_or(AppError::NotFound)?;
    */

    // For now, returning a mock to prove the type system works
    Ok(Json(PlayerProfile {
        id: 1,
        username: "Daydreamer".to_string(),
        archetype: "The Sage".to_string(),
    }))
}
