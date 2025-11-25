mod ai;
mod db;
mod sync;

use crate::ai::SocraticAgent;
use crate::db::JournalStore;
use crate::sync::SyncService;
use std::sync::Mutex;
use tauri::{Manager, State};

struct AppState {
    agent: Mutex<Option<SocraticAgent>>,
    store: Mutex<JournalStore>,
    sync: SyncService,
}

#[tauri::command]
async fn submit_journal(state: State<'_, AppState>, content: String) -> Result<String, String> {
    // 1. Save to DB
    let mut store = state.store.lock().map_err(|e| e.to_string())?;
    let entry_id = store.add_entry(&content, "").map_err(|e| e.to_string())?;

    // 2. Generate AI Response
    let response = {
        let mut agent_guard = state.agent.lock().map_err(|e| e.to_string())?;
        if let Some(agent) = agent_guard.as_mut() {
            agent
                .generate_response(&content)
                .map_err(|e| e.to_string())?
        } else {
            "AI Agent not initialized. Please download the model.".to_string()
        }
    };

    // 3. Update DB with reflection (if we wanted to store it, currently add_entry takes reflection)
    // For MVP, we just return it. In a real app, we'd update the row.

    // 4. Award XP (Mock logic)
    let _ = state.sync.push_xp(50, "Journal Entry").await;

    Ok(response)
}

#[tauri::command]
async fn sync_xp(state: State<'_, AppState>) -> Result<String, String> {
    state
        .sync
        .push_xp(0, "Manual Sync")
        .await
        .map_err(|e| e.to_string())?;
    Ok("Synced".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default().build())
        .manage(AppState {
            agent: Mutex::new(None), // Initialize as None until model is loaded
            store: Mutex::new(
                JournalStore::init("journal.db", "secret_key").expect("Failed to init DB"),
            ),
            sync: SyncService::new("http://192.168.2.141:3000".to_string()),
        })
        .invoke_handler(tauri::generate_handler![submit_journal, sync_xp])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
