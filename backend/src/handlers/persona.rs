use axum::{http::StatusCode, Json, Extension};
use sqlx::{PgPool, Row};
use common::{Dilemma, Archetype, DilemmaChoice};
use std::collections::HashMap;

pub async fn get_dilemmas(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<Dilemma>>, (StatusCode, String)> {
    let dilemma_rows = match sqlx::query("SELECT id, title, dilemma_text FROM dilemmas")
        .fetch_all(&pool)
        .await
    {
        Ok(rows) => rows,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to fetch dilemmas: {}", e),
            ));
        }
    };

    let choice_rows = match sqlx::query("SELECT id, dilemma_id, choice_text FROM dilemma_choices")
        .fetch_all(&pool)
        .await
    {
        Ok(rows) => rows,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to fetch dilemma choices: {}", e),
            ));
        }
    };

    let mut choices_map: HashMap<i32, Vec<DilemmaChoice>> = HashMap::new();
    for row in choice_rows {
        let choice = DilemmaChoice {
            id: row.get("id"),
            dilemma_id: row.get("dilemma_id"),
            choice_text: row.get("choice_text"),
        };
        choices_map.entry(choice.dilemma_id).or_default().push(choice);
    }

    let dilemmas = dilemma_rows.into_iter().map(|row| {
        let dilemma_id: i32 = row.get("id");
        Dilemma {
            id: dilemma_id,
            title: row.get("title"),
            dilemma_text: row.get("dilemma_text"),
            choices: choices_map.remove(&dilemma_id).unwrap_or_default(),
        }
    }).collect();

    Ok(Json(dilemmas))
}

pub async fn get_archetypes(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<Archetype>>, (StatusCode, String)> {
    let rows = match sqlx::query("SELECT id, name, description FROM archetypes")
        .fetch_all(&pool)
        .await {
            Ok(rows) => rows,
            Err(e) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to fetch archetypes: {}", e),
                ));
            }
        };

    let archetypes = rows.into_iter().map(|row| {
        Archetype {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
        }
    }).collect();

    Ok(Json(archetypes))
}

use common::{QuizSubmission, PlayerCharacter, QuizResult, Command};
use crate::domain::persona_logic::calculate_archetype;
use tokio::sync::oneshot;

pub async fn submit_quiz(
    Extension(pool): Extension<PgPool>,
    Extension(tx): Extension<tokio::sync::mpsc::Sender<(Command, oneshot::Sender<PlayerCharacter>)>>,
    Json(submission): Json<QuizSubmission>,
) -> Result<Json<QuizResult>, (StatusCode, String)> {
    let calc_result = calculate_archetype(&pool, &submission).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    let (one_tx, one_rx) = oneshot::channel();

    let command = Command::SetArchetype {
        archetype_id: calc_result.primary_archetype.id,
        stats: calc_result.stats.clone(),
    };

    tx.send((command, one_tx)).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let updated_player = one_rx.await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let quiz_result = QuizResult {
        player: updated_player,
        archetype: calc_result.primary_archetype,
    };

    Ok(Json(quiz_result))
}
