// Daydream Engine — Word DAG Loading
// The "Set": all vocabulary entities and their connections.
// Each word is a spell. The curriculum is a spellbook.
//
// The DAG represents meaning relationships:
//   You can't truly understand "Clarity" without walking through "Resilience".
//   The word's meaning is EARNED through the journey, not read from a definition.

use bevy::prelude::*;
use serde::Deserialize;
use crate::components::*;

/// JSON schema for a word in the curriculum DAG
#[derive(Deserialize, Clone, Debug)]
pub struct WordDef {
    pub word: String,
    pub depth_prompt: String,
    pub themes: Vec<String>,
    pub mood: String,
    pub story_text: String,
    /// Channel: "mind", "heart", "body", "action"
    #[serde(default = "default_channel")]
    pub channel: String,
    /// Stage: "hero", "outlaw", "edge_lord", "best_self"
    #[serde(default = "default_stage")]
    pub stage: String,
    #[serde(default)]
    pub yes_targets: Vec<String>,
    #[serde(default)]
    pub no_targets: Vec<String>,
    /// Synergy partners: ["Patience", "Clarity"]
    #[serde(default)]
    pub synergy_partners: Vec<String>,
}

fn default_channel() -> String { "body".to_string() }
fn default_stage() -> String { "hero".to_string() }

/// JSON schema for a complete curriculum
#[derive(Deserialize, Clone, Debug)]
pub struct CurriculumDef {
    pub name: String,
    pub start_word: String,
    pub words: Vec<WordDef>,
}

/// Resource holding the loaded curriculum name and start word
#[derive(Resource, Debug)]
pub struct Curriculum {
    pub name: String,
    pub start_word: String,
}

/// Parse a channel string to the Channel enum.
fn parse_channel(s: &str) -> Channel {
    match s.to_lowercase().as_str() {
        "mind"   => Channel::Mind,
        "heart"  => Channel::Heart,
        "action" => Channel::Action,
        _        => Channel::Body,
    }
}

/// Parse a stage string to the Stage enum.
fn parse_stage(s: &str) -> Stage {
    match s.to_lowercase().as_str() {
        "outlaw"    => Stage::Outlaw,
        "edge_lord" => Stage::EdgeLord,
        "best_self" => Stage::BestSelf,
        _           => Stage::Hero,
    }
}

/// Spawn all word entities from the hardcoded demo curriculum.
/// In the future, this loads from JSON files authored by parents.
pub fn spawn_demo_curriculum(mut commands: Commands) {
    let curriculum = demo_curriculum();

    // Store curriculum metadata
    commands.insert_resource(Curriculum {
        name: curriculum.name.clone(),
        start_word: curriculum.start_word.clone(),
    });

    // Spawn each word as an ECS entity — a spell card in the set
    for word_def in &curriculum.words {
        let channel = parse_channel(&word_def.channel);
        let stage = parse_stage(&word_def.stage);

        // Build synergy links
        let synergy_links = SynergyLinks {
            links: word_def.synergy_partners.iter().map(|partner| {
                SynergyEntry {
                    partner: partner.clone(),
                    synergy_type: SynergyType::Resonant,
                    bonus: format!("{} + {} amplify each other", word_def.word, partner),
                }
            }).collect(),
        };

        commands.spawn((
            // The spell itself
            WordCard {
                word: word_def.word.clone(),
                depth_prompt: word_def.depth_prompt.clone(),
                themes: word_def.themes.clone(),
            },
            // Which channel this spell belongs to
            channel,
            // What mastery tier this word requires
            stage,
            // DAG edges
            WordEdges {
                yes_targets: word_def.yes_targets.clone(),
                no_targets: word_def.no_targets.clone(),
            },
            // Visual style — color derived from Channel
            CardStyle {
                color: channel.color(),
                mood: word_def.mood.clone(),
            },
            // Setting backdrop — tinted by channel
            Setting {
                mood: word_def.mood.clone(),
                genre: "fantasy".to_string(),
                background_color: channel.background_color(),
            },
            // Mastery tracking — starts at zero
            SpellPower::default(),
            // Synergy relationships
            synergy_links,
        ));
    }

    // Initialize game resources
    commands.insert_resource(StudentTrail::default());
    commands.insert_resource(CurrentSlide::default());
    commands.insert_resource(CharacterSheet::default());
    commands.insert_resource(SpellBook::default());
}

/// Demo curriculum: "Bias & Mirrors" — a 5-word set about self-awareness
/// Each word is tagged with its Channel and Stage from The Great Game.
fn demo_curriculum() -> CurriculumDef {
    CurriculumDef {
        name: "Bias & Mirrors".to_string(),
        start_word: "Presence".to_string(),
        words: vec![
            WordDef {
                word: "Presence".to_string(),
                depth_prompt: "What does it mean to be here? Not yesterday, not tomorrow — just now, with the light on your face.".to_string(),
                themes: vec!["awareness".to_string(), "grounding".to_string()],
                mood: "calm".to_string(),
                story_text: "You stand before an ancient stone archway. Warm light spills through from the other side. The air is still.".to_string(),
                channel: "body".to_string(),    // Grounding, somatic awareness
                stage: "hero".to_string(),       // Entry point — absorb through story
                yes_targets: vec!["Bias".to_string()],
                no_targets: vec!["Patience".to_string()],
                synergy_partners: vec!["Patience".to_string()],
            },
            WordDef {
                word: "Bias".to_string(),
                depth_prompt: "The whisper uses your own voice. Why do we believe the worst stories we tell about ourselves?".to_string(),
                themes: vec!["self-awareness".to_string(), "critical-thinking".to_string()],
                mood: "mysterious".to_string(),
                story_text: "Dark glass trees reflect a warped version of you. A whisper echoes: 'You will fail here, just as before.'".to_string(),
                channel: "mind".to_string(),     // Metacognitive — pattern-seeing
                stage: "edge_lord".to_string(),  // Requires seeing the code behind the story
                yes_targets: vec!["Resilience".to_string()],
                no_targets: vec!["Patience".to_string()],
                synergy_partners: vec!["Clarity".to_string()],
            },
            WordDef {
                word: "Patience".to_string(),
                depth_prompt: "Patience isn't waiting. It's the ability to be at peace while things unfold. What grows when you stop pulling at it?".to_string(),
                themes: vec!["growth".to_string(), "stillness".to_string()],
                mood: "warm".to_string(),
                story_text: "A walled garden filled with sunlight and birdsong. A fountain murmurs at the center. Nothing here demands anything of you.".to_string(),
                channel: "body".to_string(),     // Stillness, felt sense
                stage: "hero".to_string(),       // Gentle entry — absorb the feeling
                yes_targets: vec!["Resilience".to_string()],
                no_targets: vec!["Clarity".to_string()],
                synergy_partners: vec!["Presence".to_string(), "Resilience".to_string()],
            },
            WordDef {
                word: "Resilience".to_string(),
                depth_prompt: "Resilience isn't about not falling. It's about what you do with the wind. Can you lean into it?".to_string(),
                themes: vec!["courage".to_string(), "persistence".to_string()],
                mood: "tense".to_string(),
                story_text: "A narrow bridge over a deep canyon. The wind pushes hard. Every step forward is a choice to keep going.".to_string(),
                channel: "action".to_string(),   // Doing despite difficulty
                stage: "outlaw".to_string(),     // Requires pushing back, finding will
                yes_targets: vec!["Clarity".to_string()],
                no_targets: vec!["Patience".to_string()],
                synergy_partners: vec!["Patience".to_string()],
            },
            WordDef {
                word: "Clarity".to_string(),
                depth_prompt: "Clarity isn't knowing the answer. It's seeing the question clearly for the first time.".to_string(),
                themes: vec!["wisdom".to_string(), "resolution".to_string()],
                mood: "transcendent".to_string(),
                story_text: "Above the clouds. The world stretches out below — every path you've walked visible from here.".to_string(),
                channel: "mind".to_string(),     // Synthesis, integration
                stage: "best_self".to_string(),  // Requires connecting prior words
                yes_targets: vec![],
                no_targets: vec![],
                synergy_partners: vec!["Bias".to_string()],
            },
        ],
    }
}
