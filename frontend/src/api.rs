use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReflectionData {
    pub archetype: String,
    pub virtue_focus: String,
    pub dilemma_choice: String, // "A", "B", "C", or "D"
}

#[server]
pub async fn submit_reflection(data: ReflectionData) -> Result<(), ServerFnError> {
    // Here, we would bridge to Bevy to spawn the entity.
    // let world =... (Access Bevy World);
    println!("Received Reflection: {:?}", data);
    Ok(())
}
