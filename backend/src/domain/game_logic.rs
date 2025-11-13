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
                        if step.trigger_condition == "ai_check:initial_description_provided"
                            && !command.is_empty()
                        {
                            player.current_step_id = step.next_step.clone();
                            info!("Quest step completed!");
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
