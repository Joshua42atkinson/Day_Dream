use bevy::prelude::*;
use common::{PlayerCharacter, QUEST_DATA};
use tokio::sync::{mpsc::Receiver, oneshot};

pub fn process_command(
    world: &mut World,
    rx: &mut Receiver<(String, oneshot::Sender<PlayerCharacter>)>,
) {
    if let Ok((command, tx)) = rx.try_recv() {
        let mut player_to_send: Option<PlayerCharacter> = None;
        let mut query = world.query::<&mut PlayerCharacter>();
        for mut player in query.iter_mut(world) {
            if let (Some(quest_id), Some(step_id)) =
                (player.current_quest_id.as_deref(), player.current_step_id.as_deref())
            {
                if let Some(quest) = QUEST_DATA.get(quest_id) {
                    if let Some(step) = quest.steps.get(step_id) {
                        let command_lower = command.trim().to_lowercase();
                        if let Some(choice) = step.choices.iter().find(|c| c.command == command_lower) {
                            if let Some(next_step_data) = quest.steps.get(&choice.next_step) {
                                player.current_step_id = Some(choice.next_step.clone());
                                player.current_step_description = next_step_data.description.clone();
                                info!("Quest advanced to step: {}", choice.next_step);
                            }
                        }
                    }
                }
            }
            player_to_send = Some(player.clone());
        }
        if let Some(player) = player_to_send {
            tx.send(player).unwrap();
        }
    }
}
