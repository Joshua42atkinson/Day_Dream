use bevy::prelude::*;
use common::{PlayerCharacter, QUEST_DATA};
use tokio::sync::{mpsc::Receiver, oneshot};

pub fn process_command(world: &mut World, command_text: String) -> PlayerCharacter {
    let mut player_to_return: Option<PlayerCharacter> = None;
    let mut query = world.query::<&mut PlayerCharacter>();

    for mut player in query.iter_mut(world) {
        if command_text.starts_with("set_archetype") {
            let parts: Vec<&str> = command_text.splitn(3, ' ').collect();
            if parts.len() == 3 {
                if let (Ok(archetype_id), Ok(stats)) =
                    (parts[1].parse::<i32>(), serde_json::from_str(parts[2]))
                {
                    player.primary_archetype_id = Some(archetype_id);
                    player.stats = stats;
                    info!("Archetype set for player");
                }
            }
        } else if let (Some(quest_id), Some(step_id)) = (
            player.current_quest_id.as_deref(),
            player.current_step_id.as_deref(),
        ) {
            if let Some(quest) = QUEST_DATA.get(quest_id) {
                if let Some(step) = quest.steps.get(step_id) {
                    let command_lower = command_text.trim().to_lowercase();
                    if let Some(choice) = step.choices.iter().find(|c| {
                        c.command == command_lower
                            && (c.required_archetype_id.is_none()
                                || c.required_archetype_id == player.primary_archetype_id)
                    }) {
                        if let Some(next_step_data) = quest.steps.get(&choice.next_step) {
                            player.current_step_id = Some(choice.next_step.clone());
                            player.current_step_description = next_step_data.description.clone();
                            info!("Quest advanced to step: {}", choice.next_step);
                        }
                    }
                }
            }
        }
        player_to_return = Some(player.clone());
    }

    // If no player found (shouldn't happen in this sim), return a default or handle error
    // For now, we assume one player exists as per the setup
    player_to_return.expect("No player found in world")
}
