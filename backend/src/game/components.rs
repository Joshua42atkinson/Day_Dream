use bevy::prelude::*;
use serde::{Serialize, Deserialize};

// The Archetype Component
#[derive(Component, Reflect, Default, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[reflect(Component)]
pub enum Archetype {
    #[default]
    Novice,
    Sage,
    Hero,
    Jester,
}

// The Virtue Stats (LitRPG Implementation)
#[derive(Component, Reflect, Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Virtues {
    pub honesty: f32,
    pub compassion: f32,
    pub valor: f32,
    pub justice: f32,
    pub sacrifice: f32,
    pub honor: f32,
    pub spirituality: f32,
    pub humility: f32, // The Ultima IV set
}

// The Bundle used to spawn a new student entity
#[derive(Bundle)]
pub struct StudentBundle {
    pub archetype: Archetype,
    pub virtues: Virtues,
    pub name: Name,
    pub level: Level,     // Custom LitRPG component
    pub xp: Experience, // Custom LitRPG component
}

#[derive(Component, Reflect, Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Level(pub u32);

#[derive(Component, Reflect, Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Experience(pub u32);
