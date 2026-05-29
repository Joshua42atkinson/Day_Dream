use crate::{AppError, AppState, Result};
use axum::{extract::State, Json};
use common::expert::StoryGraph;
use sqlx::Row;

pub async fn get_graph(State(app_state): State<AppState>) -> Result<Json<StoryGraph>> {
    // Check if we are in simulation mode (no DB)
    let pool = match app_state.pool {
        Some(pool) => pool,
        None => {
            // Return a default graph in simulation mode
            let default_graph = StoryGraph {
                id: "demo_graph".to_string(),
                title: "Simulation Story".to_string(),
                nodes: vec![],
                connections: vec![],
            };
            return Ok(Json(default_graph));
        }
    };

    // Fetch the graph (hardcoded ID for now, similar to mock)
    let row = sqlx::query("SELECT nodes, connections, title FROM story_graphs WHERE id = $1")
        .bind("demo_graph")
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Database error: {:?}", e);
            AppError::InternalServerError
        })?;

    if let Some(row) = row {
        let nodes: serde_json::Value = row.get("nodes");
        let connections: serde_json::Value = row.get("connections");
        let title: String = row.get("title");

        let graph = StoryGraph {
            id: "demo_graph".to_string(),
            title,
            nodes: serde_json::from_value(nodes).unwrap_or_default(),
            connections: serde_json::from_value(connections).unwrap_or_default(),
        };
        Ok(Json(graph))
    } else {
        // Return a default empty graph if not found (auto-create logic could go here)
        let default_graph = StoryGraph {
            id: "demo_graph".to_string(),
            title: "New Story".to_string(),
            nodes: vec![],
            connections: vec![],
        };
        Ok(Json(default_graph))
    }
}

pub async fn save_graph(
    State(app_state): State<AppState>,
    Json(payload): Json<StoryGraph>,
) -> Result<Json<StoryGraph>> {
    let pool = match app_state.pool {
        Some(pool) => pool,
        None => {
            // Mock save in simulation mode
            return Ok(Json(payload));
        }
    };

    let nodes_json = serde_json::to_value(&payload.nodes)
        .map_err(|_| AppError::InternalServerError)?;
    let connections_json = serde_json::to_value(&payload.connections)
        .map_err(|_| AppError::InternalServerError)?;

    sqlx::query(
        r#"
        INSERT INTO story_graphs (id, title, nodes, connections, updated_at)
        VALUES ($1, $2, $3, $4, NOW())
        ON CONFLICT (id) DO UPDATE
        SET title = EXCLUDED.title,
            nodes = EXCLUDED.nodes,
            connections = EXCLUDED.connections,
            updated_at = NOW()
        "#,
    )
    .bind(&payload.id)
    .bind(&payload.title)
    .bind(nodes_json)
    .bind(connections_json)
    .execute(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {:?}", e);
        AppError::InternalServerError
    })?;

    Ok(Json(payload))
}

/// Process a student's branching choice and update virtue topology.
///
/// This handler bridges the frontend's choice events into the Bevy ECS
/// state engine. Each VAAM subject word maps to specific virtue adjustments,
/// making the "topological choice mapping" from the Daydream Bible quantitative.
pub async fn submit_choice(
    State(app_state): State<AppState>,
    Json(payload): Json<common::expert::ChoiceAction>,
) -> Result<Json<common::expert::VirtueSnapshot>> {
    // Map the subject word to virtue adjustments
    // This is the pedagogical core: different concepts reinforce different virtues
    let virtues = app_state.shared_virtues.clone();

    {
        let mut v = virtues.write().map_err(|_| AppError::InternalServerError)?;

        match payload.subject_word.to_lowercase().as_str() {
            "presence" => {
                v.spirituality += 0.05;  // Grounding
                v.self_efficacy += 0.02; // Willingness to engage
            }
            "bias" => {
                v.competence += 0.05;    // Inquiry / curiosity
                v.self_esteem += 0.03;   // Facing uncomfortable truths
            }
            "growth" | "resilience" => {
                v.honor += 0.05;         // Persistence
                v.self_efficacy += 0.04; // Agency
                v.self_esteem += 0.03;   // Self-worth
            }
            "self-reflection" | "withdrawal" => {
                v.compassion += 0.04;    // Self-compassion
                v.interdependence += 0.03;
            }
            "conflict" => {
                v.valor += 0.04;
                v.self_efficacy += 0.03;
            }
            _ => {
                // Unknown subject words still reward presence
                v.self_efficacy += 0.01;
            }
        }
    }

    // Read back the updated snapshot
    let snapshot = {
        let v = virtues.read().map_err(|_| AppError::InternalServerError)?;
        common::expert::VirtueSnapshot {
            self_efficacy: v.self_efficacy,
            self_esteem: v.self_esteem,
            interdependence: v.interdependence,
            compassion: v.compassion,
            valor: v.valor,
            inquiry: v.competence,          // Map competence → inquiry
            resilience: v.honor,            // Map honor → resilience
            presence: v.spirituality,       // Map spirituality → presence
            total_choices: 0,               // TODO: add counter to VirtueTopology
        }
    };

    tracing::info!(
        "Choice processed: node={}, word={}, choice={}",
        payload.node_id,
        payload.subject_word,
        payload.choice_id
    );

    Ok(Json(snapshot))
}
