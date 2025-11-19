use axum::{
    extract::State,
    Json,
};
use std::collections::HashMap;
use tokio::sync::{oneshot};
use crate::{AppState, AppError, Result};
use crate::domain::player::get_simulated_character;
use common::{
    PlayerCharacter,
    CHARACTER_TEMPLATES,
    JournalData, VocabEntry,
    ProfileData, CharacterSummary,
    PlayerCommand, GameTurn,
    PlayerProfile,
};

pub async fn handle_submit_command(
    State(app_state): State<AppState>,
    Json(payload): Json<PlayerCommand>,
) -> Result<Json<GameTurn>> {
    let (one_tx, one_rx) = oneshot::channel();
    let command = payload.command_text.clone();
    app_state.tx.send((command, one_tx)).await.map_err(|_| AppError::InternalServerError)?;

    let updated_character = one_rx.await.map_err(|_| AppError::InternalServerError)?;

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

    Ok(Json(game_turn))
}

pub async fn get_player_character(State(_app_state): State<AppState>) -> Result<Json<PlayerCharacter>> {
    let character = get_simulated_character();
    Ok(Json(character))
}

pub async fn get_journal_data(State(_app_state): State<AppState>) -> Result<Json<JournalData>> {
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
    Ok(Json(data))
}

pub async fn get_profile_data(State(_app_state): State<AppState>) -> Result<Json<ProfileData>> {
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
        has_premium: true,
        characters: characters,
        premade_characters: CHARACTER_TEMPLATES.to_vec(),
    };
    Ok(Json(data))
}

pub async fn get_player_profile(
    State(_app_state): State<AppState>,
) -> Result<Json<PlayerProfile>> {
    // let player = sqlx::query_as!(PlayerProfile, "SELECT * FROM players WHERE id = $1", 1)
    //     .fetch_optional(&app_state.pool)
    //     .await?
    //     .ok_or(AppError::NotFound)?;

    Ok(Json(PlayerProfile {
        id: 1,
        username: "Daydreamer".to_string(),
        archetype: "The Sage".to_string(),
    }))
}
