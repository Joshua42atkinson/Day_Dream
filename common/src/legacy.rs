// ============================================================================
// LEGACY MODULE — Archived from the Daydream LitRPG / Iron Road era
// ============================================================================
//
// These data structures were part of the original 3D NPC sandbox and Python
// quest engine. They are preserved here for reference and backward compat,
// but are NOT used by the current Somatic Branching Sandbox architecture.
//
// If you need to restore any of these, move them back to lib.rs.
// If they remain unused after 90 days, delete this file entirely.
//
// Archived: May 20, 2026

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

// --- Quest System (from quests.py) ---

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct QuestReward {
    #[serde(rename = "type")]
    pub reward_type: String,
    #[serde(default)]
    pub value: Option<i32>,
    #[serde(default)]
    pub details: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub target: Option<String>,
    #[serde(default)]
    pub change: Option<i32>,
    #[serde(default)]
    pub set_flag: Option<HashMap<String, bool>>,
    #[serde(default)]
    pub silent: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Choice {
    pub text: String,
    pub command: String,
    pub next_step: String,
    #[serde(default)]
    pub required_archetype_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuestStep {
    pub description: String,
    #[serde(default)]
    pub choices: Vec<Choice>,
    pub trigger_condition: String,
    pub next_step: Option<String>,
    pub step_reward: Option<QuestReward>,
    #[serde(default)]
    pub is_major_plot_point: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Quest {
    pub title: String,
    pub chapter_theme: String,
    pub description: String,
    pub starting_step: String,
    pub completion_reward: QuestReward,
    pub steps: HashMap<String, QuestStep>,
}

pub type QuestData = HashMap<String, Quest>;

// --- Character System (from premade_characters.json) ---

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CharacterTemplate {
    pub id: String,
    pub name: String,
    pub race_name: String,
    pub class_name: String,
    pub philosophy_name: String,
    pub boon: String,
    pub backstory: String,
    pub starting_quest_id: String,
    pub display_desc: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RaceData {
    pub abilities: Vec<String>,
    pub fate_point_mod: i32,
}

// --- PlayerCharacter (the "master" character struct) ---

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PlayerCharacter {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub race_name: String,
    pub class_name: String,
    pub philosophy_name: String,
    pub boon: String,
    pub backstory: String,
    pub abilities: Vec<String>,
    pub aspects: Vec<String>,
    pub inventory: Vec<String>,
    pub quest_flags: HashMap<String, bool>,
    pub current_location: String,
    pub current_quest_id: Option<String>,
    pub current_step_id: Option<String>,
    pub current_quest_title: String,
    pub current_step_description: String,
    pub fate_points: i32,
    pub report_summaries: Vec<ReportSummary>,
    #[serde(default)]
    pub primary_archetype_id: Option<i32>,
    #[serde(default)]
    pub stats: HashMap<String, i32>,
    #[serde(skip)]
    pub learned_vocab: HashSet<String>,
}

// --- Profile, Journal, Vocab ---

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ProfileData {
    pub email: String,
    pub has_premium: bool,
    pub characters: Vec<CharacterSummary>,
    pub premade_characters: Vec<CharacterTemplate>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CharacterSummary {
    pub id: String,
    pub name: String,
    pub race: String,
    pub class_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JournalData {
    pub awl_words: Vec<VocabEntry>,
    pub ai_word_lists: HashMap<String, Vec<VocabEntry>>,
    pub report_summaries: Vec<ReportSummary>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct VocabEntry {
    pub word: String,
    pub definition: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ReportSummary {
    pub chapter: u32,
    pub summary: String,
    pub comprehension_score: f32,
    pub player_xp_gained: i32,
}

// --- Persona Engine Data ---

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Archetype {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Stat {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ArchetypeStatBuff {
    pub archetype_id: i32,
    pub stat_id: i32,
    pub buff_value: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Dilemma {
    pub id: i32,
    pub title: String,
    pub dilemma_text: String,
    pub choices: Vec<DilemmaChoice>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DilemmaChoice {
    pub id: i32,
    pub dilemma_id: i32,
    pub choice_text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DilemmaChoiceArchetypePoint {
    pub dilemma_choice_id: i32,
    pub archetype_id: i32,
    pub points: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct QuizSubmission {
    pub answers: HashMap<i32, i32>,
}

// --- Wire Protocol (legacy game loop) ---

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PlayerCommand {
    pub command_text: String,
    pub current_character: PlayerCharacter,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PlayerProfile {
    pub id: i32,
    pub username: String,
    pub archetype: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GameTurn {
    pub player_command: String,
    pub ai_narrative: String,
    pub system_message: Option<String>,
    pub updated_character: PlayerCharacter,
}
