use crate::game::components::*;
use bevy::prelude::*;
use bevy_yarnspinner::events::{DialogueCompleteEvent, ExecuteCommandEvent};

pub fn update_virtue_topology(
    mut query: Query<(&mut VirtueTopology, &StoryProgress), Changed<StoryProgress>>,
) {
    for (mut virtues, progress) in query.iter_mut() {
        // Logic: If the story has progressed, it implies a decision was made.
        // In a real implementation, we would look up the specific choice in a database
        // to determine which virtues to adjust.
        // For now, we simulate the "willingness to make a decision" (Self-Efficacy).

        if let Some(_step) = &progress.current_step_id {
            // Boost Self-Efficacy for taking action
            virtues.self_efficacy = (virtues.self_efficacy + 0.05).min(1.0);
            info!(
                "Virtue Update: Self-Efficacy increased to {}",
                virtues.self_efficacy
            );

            // Placeholder: If the history length is even, boost Self-Esteem (simulating a "good" outcome)
            if progress.history.len() % 2 == 0 {
                virtues.self_esteem = (virtues.self_esteem + 0.02).min(1.0);
                info!(
                    "Virtue Update: Self-Esteem increased to {}",
                    virtues.self_esteem
                );
            }

            // Placeholder: Interdependence grows slowly over time/steps
            virtues.interdependence = (virtues.interdependence + 0.01).min(1.0);
        }
    }
}

pub fn monitor_cognitive_load(
    mut query: Query<(&mut CognitiveLoad, &StoryProgress)>,
    time: Res<Time>,
) {
    for (mut load, _progress) in query.iter_mut() {
        // Simulate dynamic cognitive load
        // Intrinsic load could be based on the complexity of the current step (not yet modeled)
        // Extraneous load could decay over time if the UI is "smooth"

        // Simple simulation: Germane load (learning) oscillates slightly
        let oscillation = (time.elapsed_seconds() * 0.5).sin() * 0.05;
        load.germane = (0.5 + oscillation).clamp(0.0, 1.0);

        // Decay extraneous load
        load.extraneous = (load.extraneous - 0.01 * time.delta_seconds()).max(0.0);
    }
}

pub fn log_research_events(
    mut query: Query<
        (&mut ResearchLog, &VirtueTopology, &StoryProgress),
        (Changed<VirtueTopology>, Changed<StoryProgress>),
    >,
    time: Res<Time>,
) {
    for (mut log, virtues, progress) in query.iter_mut() {
        // Log Virtue Updates
        // In a real system, we'd diff against previous state, but for now we log snapshots on change
        let event = ResearchEvent {
            timestamp: time.elapsed_seconds_f64(),
            event_type: "VIRTUE_SNAPSHOT".to_string(),
            data: serde_json::to_string(virtues).unwrap_or_default(),
        };
        log.events.push(event);

        // Log Narrative Progress
        if let Some(step) = &progress.current_step_id {
            let event = ResearchEvent {
                timestamp: time.elapsed_seconds_f64(),
                event_type: "NARRATIVE_STEP".to_string(),
                data: format!("Step: {}", step),
            };
            log.events.push(event);
        }

        info!("Research Log Updated: {} events recorded", log.events.len());
    }
}

// System to sync YarnSpinner events to our ECS components
pub fn sync_yarn_to_story_progress(
    mut events: EventReader<DialogueCompleteEvent>,
    mut command_events: EventReader<ExecuteCommandEvent>,
    mut query: Query<&mut VirtueTopology>,
    mut whistle_writer: EventWriter<PlayWhistleEvent>,
) {
    for _event in events.read() {
        info!("Dialogue Complete!");
        // Trigger the Whistle on dialogue completion (Reflection Moment)
        whistle_writer.send(PlayWhistleEvent);
    }

    for event in command_events.read() {
        // Parse command: <<add_stat Valor 2>>
        let command = &event.command;
        let parts: Vec<&str> = command.name.split_whitespace().collect();

        if parts.len() >= 1 && parts[0] == "add_stat" {
            if command.parameters.len() >= 2 {
                let stat_name = command.parameters[0].to_string();
                let value_str = command.parameters[1].to_string();

                if let Ok(value) = value_str.parse::<f32>() {
                    for mut virtues in query.iter_mut() {
                        match stat_name.as_str() {
                            "Valor" => virtues.valor += value,
                            "Eloquence" => virtues.competence += value, // Mapping Eloquence to Competence for now
                            "Compassion" => virtues.compassion += value,
                            "SelfEfficacy" => virtues.self_efficacy += value,
                            "Intelligence" => virtues.competence += value, // Mapping Int to Competence
                            "Interdependence" => virtues.interdependence += value,
                            _ => warn!("Unknown stat in Yarn command: {}", stat_name),
                        }
                        info!("Yarn Command Applied: {} +{}", stat_name, value);
                    }
                }
            }
        }
    }
}

// System to sync ECS components to Shared Resources (for Axum)
pub fn sync_ecs_to_shared(
    query: Query<(&ResearchLog, &VirtueTopology), Changed<ResearchLog>>,
    shared_log: Res<crate::game::components::SharedResearchLogResource>,
    shared_virtues: Res<crate::game::components::SharedVirtuesResource>,
) {
    for (log, virtues) in query.iter() {
        if let Ok(mut shared_log_guard) = shared_log.0.write() {
            *shared_log_guard = log.clone();
        }
        if let Ok(mut shared_virtues_guard) = shared_virtues.0.write() {
            *shared_virtues_guard = virtues.clone();
        }
    }
}

// System to play the "Whistle" sound on specific events
pub fn whistle_system(
    mut events: EventReader<PlayWhistleEvent>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for _event in events.read() {
        info!("Playing Whistle Sound!");
        commands.spawn(AudioBundle {
            source: asset_server.load("whistle.ogg"),
            settings: PlaybackSettings::DESPAWN,
        });
    }
}

// Resource to hold the channel receiver for download progress
#[derive(Resource)]
pub struct DownloadChannel(
    pub tokio::sync::mpsc::Receiver<crate::services::downloader::DownloadProgress>,
);

// System to handle StartDownloadEvent
pub fn download_manager_system(
    mut events: EventReader<StartDownloadEvent>,
    mut commands: Commands,
) {
    for event in events.read() {
        info!("Starting download for: {}", event.model_config.name);

        let (tx, rx) = tokio::sync::mpsc::channel(100);
        commands.insert_resource(DownloadChannel(rx));

        let url = event.model_config.hf_repo.clone(); // In real implementation, construct full URL
        let filename = event.model_config.filename.clone();

        // Construct full URL (simplified for now)
        let full_url = format!("https://huggingface.co/{}/resolve/main/{}", url, filename);
        let path = std::path::PathBuf::from("models").join(filename);

        // Spawn async task
        tokio::spawn(async move {
            if let Err(e) = crate::services::downloader::download_file(full_url, path, tx).await {
                error!("Download failed: {}", e);
            }
        });
    }
}

// System to sync Inbox (from Axum) to ECS Events
pub fn sync_inbox_to_events(
    inbox: Res<crate::game::components::DownloadCommandInbox>,
    mut event_writer: EventWriter<StartDownloadEvent>,
) {
    if let Ok(mut guard) = inbox.0.write() {
        for event in guard.drain(..) {
            info!(
                "Received download command from API: {}",
                event.model_config.name
            );
            event_writer.send(event);
        }
    }
}

// System to update progress from the channel
pub fn progress_update_system(
    channel: Option<ResMut<DownloadChannel>>,
    mut event_writer: EventWriter<DownloadProgressEvent>,
    shared_state: Res<crate::game::components::SharedDownloadStateResource>,
) {
    if let Some(mut rx) = channel {
        // Drain all available messages
        while let Ok(progress) = rx.0.try_recv() {
            // info!("Download Progress: {:.2}%", progress.percent); // Reduce log spam

            let event = DownloadProgressEvent {
                percent: progress.percent,
                downloaded_bytes: progress.downloaded_bytes,
                total_bytes: progress.total_bytes,
            };

            // Send internal event
            event_writer.send(event.clone());

            // Update shared state for API
            if let Ok(mut guard) = shared_state.0.write() {
                *guard = Some(event);
            }

            if progress.percent >= 100.0 {
                info!("Download Complete!");
            }
        }
    }
}

// [NEW] Physics System: Calculate Train Velocity based on Mass and Power
pub fn calculate_train_velocity(
    mut query: Query<(&mut TrainVelocity, &Mass, &EnginePower, &mut CognitiveLoad)>,
) {
    for (mut velocity, mass, power, mut load) in query.iter_mut() {
        // Physics: Velocity = Power / Mass
        // Ensure mass is at least 1.0 to avoid division by zero
        let effective_mass = mass.0.max(1.0);
        velocity.0 = power.0 / effective_mass;

        // Pedagogical Link: If Velocity drops below threshold, increase Extraneous Load
        // This simulates "The Drag" of too much content
        if velocity.0 < 5.0 {
            // Arbitrary threshold for "slow"
            // Increase extraneous load slightly
            load.extraneous = (load.extraneous + 0.001).min(1.0);
        }
    }
}

// [NEW] System to bridge Bevy Events <-> Shared Resources for Pete AI
pub fn sync_pete_bridge(
    mut ask_events: EventReader<AskPeteEvent>,
    mut response_writer: EventWriter<PeteResponseEvent>,
    command_inbox: Res<PeteCommandInbox>,
    response_outbox: Res<PeteResponseOutbox>,
) {
    // 1. Sync Outbound Commands (Bevy -> Axum/Tokio)
    if !ask_events.is_empty() {
        if let Ok(mut inbox) = command_inbox.0.write() {
            for event in ask_events.read() {
                info!("Sending AskPeteEvent to Inbox: {}", event.content);
                inbox.push(event.clone());
            }
        }
    }

    // 2. Sync Inbound Responses (Axum/Tokio -> Bevy)
    if let Ok(mut outbox) = response_outbox.0.write() {
        if !outbox.is_empty() {
            for response in outbox.drain(..) {
                info!("Received Pete Response: {}", response.content);
                response_writer.send(response);
            }
        }
    }
}

// [NEW] System to track student miles
pub fn track_student_miles(mut query: Query<(&TrainVelocity, &mut StudentMiles)>, time: Res<Time>) {
    for (velocity, mut miles) in query.iter_mut() {
        // Simple simulation: velocity (units/sec) * time (sec) = distance (units)
        // We treat units as "miles" for simplicity
        let distance = velocity.0 * time.delta_seconds();
        if distance > 0.0 {
            miles.total_miles += distance;
            // info!("Miles Traveled: {:.2} (Total: {:.2})", distance, miles.total_miles);
        }
    }
}

// [NEW] System to sync Physics State to Shared Resource (for Axum)
pub fn sync_physics_to_shared(
    query: Query<(&Mass, &EnginePower, &TrainVelocity, &StudentMiles)>,
    shared_physics: Res<crate::game::components::SharedPhysicsResource>,
) {
    if let Ok((mass, power, velocity, miles)) = query.get_single() {
        if let Ok(mut guard) = shared_physics.0.write() {
            guard.mass = mass.0;
            guard.power = power.0;
            guard.velocity = velocity.0;
            guard.miles = miles.total_miles;
        }
    }
}
