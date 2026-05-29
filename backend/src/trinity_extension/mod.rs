// ═══════════════════════════════════════════════════════════════════════════════
// DAY_DREAM — Trinity Extension
// ═══════════════════════════════════════════════════════════════════════════════
//
// FILE:         trinity_extension/mod.rs
// PURPOSE:      Day_Dream's implementation of TrinityExtension trait
//               Game-specific narrative, book writing, and quest systems
//
// ARCHITECTURE:
//   • book — Book of the Bible narrative system
//   • game_loop — CreepBestiary and game state management
//   • narrative — Prose generation for game events
//   • great_recycler — LongCat integration for narration
//
// CHANGES:
//   2026-05-26  Extracted from trinity-iron-road during three-way split
//
// ═══════════════════════════════════════════════════════════════════════════════

pub mod book;
pub mod game_loop;
pub mod narrative;
pub mod great_recycler;

pub use book::*;
pub use game_loop::*;
pub use narrative::*;
pub use great_recycler::*;
