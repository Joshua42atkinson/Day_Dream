// Daydream Engine — ECS Components
// Each word in the curriculum is an entity. Words support each other
// in the DAG like cards in a trading card game.
//
// The Great Game Framework:
//   Letters SPELL words. Words are spells. Mastering a word means
//   learning to cast that spell — changing your relationship with reality.

use bevy::prelude::*;

// ─── THE FOUR CHANNELS (Card Element Types) ─────────────────────

/// The Four Channels of Consciousness from The Great Game.
/// Each word belongs to a Channel, which determines its card color,
/// the type of engagement it demands, and its synergy relationships.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Channel {
    /// 🟢 The Sage — "What does this mean?"
    /// Analytical, pattern-seeing, metacognitive words.
    Mind,
    /// 🟠 The Mystic — "Where is the love here?"
    /// Emotional, connective, passionate words.
    Heart,
    /// 🔵 The Healer — "What is my body telling me?"
    /// Somatic, intuitive, grounding words.
    Body,
    /// 🟡 The Builder — "How do I make this real?"
    /// Action-oriented, manifesting, crafting words.
    Action,
}

impl Channel {
    /// The signature color for this channel's card frame.
    pub fn color(&self) -> Color {
        match self {
            Channel::Mind   => Color::srgba(0.29, 0.62, 0.43, 1.0),  // #4a9e6e green
            Channel::Heart  => Color::srgba(0.83, 0.47, 0.24, 1.0),  // #d4783c orange
            Channel::Body   => Color::srgba(0.29, 0.49, 0.71, 1.0),  // #4a7eb5 blue
            Channel::Action => Color::srgba(0.77, 0.64, 0.24, 1.0),  // #c4a43c gold
        }
    }

    /// Darker variant for setting backgrounds (15% intensity).
    pub fn background_color(&self) -> Color {
        match self {
            Channel::Mind   => Color::srgba(0.04, 0.09, 0.06, 1.0),
            Channel::Heart  => Color::srgba(0.12, 0.07, 0.04, 1.0),
            Channel::Body   => Color::srgba(0.04, 0.07, 0.11, 1.0),
            Channel::Action => Color::srgba(0.12, 0.10, 0.04, 1.0),
        }
    }

    /// Display label with emoji.
    pub fn label(&self) -> &'static str {
        match self {
            Channel::Mind   => "Mind",
            Channel::Heart  => "Heart",
            Channel::Body   => "Body",
            Channel::Action => "Action",
        }
    }

    /// The core question this channel asks.
    pub fn question(&self) -> &'static str {
        match self {
            Channel::Mind   => "What does this mean?",
            Channel::Heart  => "Where is the love here?",
            Channel::Body   => "What is my body telling me?",
            Channel::Action => "How do I make this real?",
        }
    }
}

// ─── THE FOUR STAGES (Mastery Tiers) ────────────────────────────

/// The Four Stages of Consciousness from The Great Game.
/// Determines how deeply the student must engage with a word.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Stage {
    /// Immersed in the story, absorbing through narrative.
    /// Simple, emotionally grounded words.
    Hero,
    /// Pushing back, discovering tension between the word
    /// and the student's assumptions.
    Outlaw,
    /// Metacognitive — seeing the code behind the story.
    /// The depth prompt becomes a Socratic mirror.
    EdgeLord,
    /// Synthesis — creating new meaning from mastered words.
    /// The student's own journey IS the story.
    BestSelf,
}

impl Stage {
    /// Star indicator for UI display.
    pub fn stars(&self) -> &'static str {
        match self {
            Stage::Hero     => "★",
            Stage::Outlaw   => "★★",
            Stage::EdgeLord => "★★★",
            Stage::BestSelf => "★★★★",
        }
    }

    /// Display name.
    pub fn label(&self) -> &'static str {
        match self {
            Stage::Hero     => "Hero",
            Stage::Outlaw   => "Outlaw",
            Stage::EdgeLord => "Edge Lord",
            Stage::BestSelf => "Best Self",
        }
    }
}

// ─── SPELL POWER (Word Mastery Tracking) ────────────────────────

/// How deeply a student has internalized a word-spell.
/// The Great Recycler IS the progression:
///   Junk (unknown) → Process (experience) → Gear (owned spell)
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum MasteryLevel {
    /// Saw the card — the word is known to exist.
    Encountered,
    /// Engaged with the story — felt the word in context.
    Experienced,
    /// Used the word meaningfully — connected to personal meaning.
    Owned,
    /// Connected to other words — sees the web of meaning.
    Mastered,
}

impl MasteryLevel {
    /// Emoji indicator for UI.
    pub fn icon(&self) -> &'static str {
        match self {
            MasteryLevel::Encountered => "🔮",
            MasteryLevel::Experienced => "⚡",
            MasteryLevel::Owned       => "🌟",
            MasteryLevel::Mastered    => "👑",
        }
    }
}

/// Tracks the student's mastery of a specific word entity.
#[derive(Component, Clone, Debug)]
pub struct SpellPower {
    /// Current mastery level.
    pub mastery: MasteryLevel,
    /// How many times the student has seen this word.
    pub times_encountered: u32,
    /// How many times the student swiped "Deeper" on this word.
    pub times_explored_deeper: u32,
    /// How many synergy partners of this word have been visited.
    pub synergies_discovered: u32,
}

impl Default for SpellPower {
    fn default() -> Self {
        Self {
            mastery: MasteryLevel::Encountered,
            times_encountered: 0,
            times_explored_deeper: 0,
            synergies_discovered: 0,
        }
    }
}

// ─── SYNERGY LINKS (Word ↔ Word Relationships) ─────────────────

/// The type of synergy between two words, inspired by the Five Phases
/// (Wǔ Xíng) Generation Cycle from The Great Game.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SynergyType {
    /// Wood feeds Fire: understanding deepens feeling.
    /// Mind words nourish Heart words.
    Nourishing,
    /// Fire creates Earth: passion fuels action.
    /// Heart words generate Action words.
    Generating,
    /// Opposing poles: mastering both creates balance.
    /// Like Courage + Vulnerability from Chapter 12.
    Complementary,
    /// Same family: words reinforce each other's meaning.
    /// Like Presence + Patience (both Body/grounding).
    Resonant,
}

/// A synergy link from this word to another.
/// When both are mastered, a synergy bonus activates.
#[derive(Component, Clone, Debug)]
pub struct SynergyLinks {
    pub links: Vec<SynergyEntry>,
}

/// A single synergy relationship.
#[derive(Clone, Debug)]
pub struct SynergyEntry {
    /// The partner word's name.
    pub partner: String,
    /// The type of synergy.
    pub synergy_type: SynergyType,
    /// Description of the synergy bonus.
    pub bonus: String,
}

impl Default for SynergyLinks {
    fn default() -> Self {
        Self { links: Vec::new() }
    }
}

// ─── WORD CARD (the TCG card entity) ─────────────────────────────

/// The core vocabulary entity. Every word in the curriculum
/// becomes one of these. VAAM: Vocabulary Acquisition Autonomous Meaning.
/// A word isn't defined by other words — it's defined by experience.
/// Letters SPELL words. Words are spells. This card IS a spell.
#[derive(Component, Clone, Debug)]
pub struct WordCard {
    /// The vocabulary word itself: "Resilience", "Bias", "Patience"
    pub word: String,
    /// Socratic question for the "dig deeper" swipe-down action.
    /// This is experiential, not a dictionary definition.
    pub depth_prompt: String,
    /// Thematic tags for DAG relationships: ["courage", "growth"]
    pub themes: Vec<String>,
}

/// DAG edges: which words this word connects to.
/// Forward-only (acyclic) — the student always progresses.
#[derive(Component, Clone, Debug, Default)]
pub struct WordEdges {
    /// Words reachable via "yes" (swipe right)
    pub yes_targets: Vec<String>,
    /// Words reachable via "no" (swipe left) — may loop to gentler concepts
    pub no_targets: Vec<String>,
}

/// Visual properties for the card frame.
/// Color is now derived from the Channel, with mood driving the setting.
#[derive(Component, Clone, Debug)]
pub struct CardStyle {
    /// Primary color of the card border/frame — derived from Channel.
    pub color: Color,
    /// The setting mood this card evokes: "mysterious", "warm", "tense"
    pub mood: String,
}

impl Default for CardStyle {
    fn default() -> Self {
        Self {
            color: Color::srgba(0.3, 0.4, 0.6, 1.0),
            mood: "neutral".to_string(),
        }
    }
}

// ─── SETTING (the vibe/genre backdrop) ───────────────────────────

/// The emotional/visual context layer. Changes based on the
/// current word's mood + Channel background color.
/// BOTTOM of the Triple Sandwich = the Body's experience of the word.
#[derive(Component, Clone, Debug)]
pub struct Setting {
    pub mood: String,
    pub genre: String,
    pub background_color: Color,
}

impl Default for Setting {
    fn default() -> Self {
        Self {
            mood: "calm".to_string(),
            genre: "fantasy".to_string(),
            background_color: Color::srgba(0.04, 0.04, 0.06, 1.0),
        }
    }
}

// ─── CHARACTER SHEET (Student Profile) ───────────────────────────

/// The student's living Character Sheet from The Great Game.
/// Tracks channel attunement, emergent class, and active buffs/debuffs.
/// This is the "AI Mirror" — reflecting the student's patterns.
#[derive(Resource, Debug)]
pub struct CharacterSheet {
    /// Channel attunement scores (0.0 = disconnected, 1.0 = integrated)
    pub mind_attunement: f32,
    pub heart_attunement: f32,
    pub body_attunement: f32,
    pub action_attunement: f32,

    /// Emergent class title — derived from dominant channel pattern.
    /// e.g., "The Oracle" (Mind-heavy), "The Bard" (Heart-heavy)
    pub emergent_class: String,

    /// Total words encountered across all sessions.
    pub words_encountered: u32,
    /// Total "Deeper" swipes — a measure of curiosity.
    pub total_deeper_swipes: u32,
}

impl Default for CharacterSheet {
    fn default() -> Self {
        Self {
            mind_attunement: 0.0,
            heart_attunement: 0.0,
            body_attunement: 0.0,
            action_attunement: 0.0,
            emergent_class: "Newcomer".to_string(),
            words_encountered: 0,
            total_deeper_swipes: 0,
        }
    }
}

impl CharacterSheet {
    /// Update attunement based on engaging with a word of this Channel.
    /// Attunement rises slowly (0.05 per engagement), capped at 1.0.
    pub fn engage_channel(&mut self, channel: &Channel) {
        let bump = 0.05;
        match channel {
            Channel::Mind   => self.mind_attunement = (self.mind_attunement + bump).min(1.0),
            Channel::Heart  => self.heart_attunement = (self.heart_attunement + bump).min(1.0),
            Channel::Body   => self.body_attunement = (self.body_attunement + bump).min(1.0),
            Channel::Action => self.action_attunement = (self.action_attunement + bump).min(1.0),
        }
        self.update_class();
    }

    /// Derive the emergent class from the dominant channel pattern.
    fn update_class(&mut self) {
        let scores = [
            (self.mind_attunement,   "Mind"),
            (self.heart_attunement,  "Heart"),
            (self.body_attunement,   "Body"),
            (self.action_attunement, "Action"),
        ];

        let dominant = scores.iter()
            .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal))
            .map(|s| s.1)
            .unwrap_or("Mind");

        self.emergent_class = match dominant {
            "Mind"   => "The Oracle".to_string(),
            "Heart"  => "The Bard".to_string(),
            "Body"   => "The Cultivator".to_string(),
            "Action" => "The Templar".to_string(),
            _        => "The Architect".to_string(),
        };
    }

    /// Render a text-based channel attunement bar for trail review.
    pub fn attunement_display(&self) -> String {
        fn bar(val: f32) -> String {
            let filled = (val * 8.0).round() as usize;
            let empty = 8 - filled;
            format!("{}{}", "█".repeat(filled), "░".repeat(empty))
        }
        format!(
            "Mind:   {} {:.0}%\nHeart:  {} {:.0}%\nBody:   {} {:.0}%\nAction: {} {:.0}%",
            bar(self.mind_attunement), self.mind_attunement * 100.0,
            bar(self.heart_attunement), self.heart_attunement * 100.0,
            bar(self.body_attunement), self.body_attunement * 100.0,
            bar(self.action_attunement), self.action_attunement * 100.0,
        )
    }
}

// ─── SPELL BOOK (Word Collection) ────────────────────────────────

/// A record of a single word in the student's SpellBook.
#[derive(Clone, Debug)]
pub struct SpellBookEntry {
    pub word: String,
    pub channel: Channel,
    pub mastery: MasteryLevel,
    pub times_encountered: u32,
}

/// The student's SpellBook — their collection of word-spells.
/// In TCG terms: your card collection. Words you OWN, experientially.
#[derive(Resource, Debug, Default)]
pub struct SpellBook {
    pub entries: Vec<SpellBookEntry>,
}

impl SpellBook {
    /// Add or update a word in the SpellBook.
    pub fn record_encounter(&mut self, word: &str, channel: Channel) {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.word == word) {
            entry.times_encountered += 1;
        } else {
            self.entries.push(SpellBookEntry {
                word: word.to_string(),
                channel,
                mastery: MasteryLevel::Encountered,
                times_encountered: 1,
            });
        }
    }

    /// Upgrade mastery for a word (if it exists).
    pub fn upgrade_mastery(&mut self, word: &str, new_level: MasteryLevel) {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.word == word) {
            if new_level > entry.mastery {
                entry.mastery = new_level;
            }
        }
    }

    /// Count words at each mastery level.
    pub fn summary(&self) -> String {
        let encountered = self.entries.iter().filter(|e| e.mastery == MasteryLevel::Encountered).count();
        let experienced = self.entries.iter().filter(|e| e.mastery == MasteryLevel::Experienced).count();
        let owned = self.entries.iter().filter(|e| e.mastery == MasteryLevel::Owned).count();
        let mastered = self.entries.iter().filter(|e| e.mastery == MasteryLevel::Mastered).count();

        format!(
            "SpellBook: {} words\n🔮 Encountered: {}\n⚡ Experienced: {}\n🌟 Owned: {}\n👑 Mastered: {}",
            self.entries.len(), encountered, experienced, owned, mastered
        )
    }
}

// ─── GAME STATE (Core Resources) ────────────────────────────────

/// The student's journey — their linear path through the word DAG.
/// This IS the pedagogical artifact: the Hero's Journey rendered as data.
#[derive(Resource, Default, Debug)]
pub struct StudentTrail {
    /// Words visited, in order
    pub visited_words: Vec<String>,
    /// How the student responded at each step
    pub swipe_history: Vec<SwipeChoice>,
    /// The currently active word entity
    pub current_word: Option<Entity>,
}

/// How a student responded to a word
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SwipeChoice {
    /// Swipe right: "Yes, I accept this / continue"
    Yes,
    /// Swipe left: "No, I reject this / skip"
    No,
    /// Swipe down: "Tell me more" (VAAM depth — Vulnerability)
    Deeper,
}

/// The current slide state — what's being displayed right now
#[derive(Resource, Default, Debug, Clone)]
pub struct CurrentSlide {
    /// AI-generated (or hardcoded) story text for this moment
    pub story_text: String,
    /// The setting mood that styles the background
    pub setting_mood: String,
    /// Whether the card is ready for swipe input
    pub ready_for_input: bool,
    /// Whether the depth overlay is showing
    pub depth_showing: bool,
}

// ─── GRAMMAR SYMBOLS (Card Type Markers) ────────────────────

/// Grammar symbols that teach parts of speech through gameplay.
/// A ◆Stone (noun) behaves differently than a ◇Spark (verb).
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Symbol {
    /// ◆ Noun — persists on field
    Stone,
    /// ◇ Verb — one-time cast effect
    Spark,
    /// △ Adjective/Adverb — attaches to boost another card
    Prism,
    /// ○ Abstract concept — resonates with any card type
    Void,
    /// ☆ Key term / proper noun — anchors a synergy chain
    Star,
}

impl Symbol {
    /// Unicode icon for display
    pub fn icon(&self) -> &'static str {
        match self {
            Symbol::Stone => "◆",
            Symbol::Spark => "◇",
            Symbol::Prism => "△",
            Symbol::Void  => "○",
            Symbol::Star  => "☆",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Symbol::Stone => "Stone",
            Symbol::Spark => "Spark",
            Symbol::Prism => "Prism",
            Symbol::Void  => "Void",
            Symbol::Star  => "Star",
        }
    }
}

// ─── DECK / HAND / DISCARD (TCG Resources) ──────────────────

/// The student's deck — all unplayed word cards for this session.
/// Cards are drawn from the deck into the hand.
#[derive(Resource, Debug)]
pub struct Deck {
    /// Word entities in draw order (top of deck = last element)
    pub cards: Vec<Entity>,
}

impl Default for Deck {
    fn default() -> Self {
        Self { cards: Vec::new() }
    }
}

impl Deck {
    pub fn remaining(&self) -> usize {
        self.cards.len()
    }

    /// Draw the top card (removes from deck)
    pub fn draw(&mut self) -> Option<Entity> {
        self.cards.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
}

/// The student's hand — cards available to play right now.
#[derive(Resource, Debug)]
pub struct Hand {
    /// Word entities currently in hand (ordered left to right)
    pub cards: Vec<Entity>,
    /// Maximum hand size
    pub max_size: usize,
    /// Currently selected card index (None = no card selected)
    pub selected: Option<usize>,
}

impl Default for Hand {
    fn default() -> Self {
        Self {
            cards: Vec::new(),
            max_size: 3,
            selected: None,
        }
    }
}

impl Hand {
    pub fn is_full(&self) -> bool {
        self.cards.len() >= self.max_size
    }

    pub fn card_count(&self) -> usize {
        self.cards.len()
    }

    /// Remove a card by index and return its entity
    pub fn remove_card(&mut self, index: usize) -> Option<Entity> {
        if index < self.cards.len() {
            self.selected = None;
            Some(self.cards.remove(index))
        } else {
            None
        }
    }

    /// Remove the currently selected card
    pub fn play_selected(&mut self) -> Option<Entity> {
        if let Some(idx) = self.selected {
            self.remove_card(idx)
        } else {
            None
        }
    }
}

/// The discard pile — played/skipped word cards.
#[derive(Resource, Default, Debug)]
pub struct DiscardPile {
    pub cards: Vec<Entity>,
}

// ─── HAND SLOT MARKER ───────────────────────────────────────

/// Marks a visual entity as occupying a specific hand slot (0-4).
/// Used by the renderer to position cards in the fan layout.
#[derive(Component, Debug)]
pub struct HandSlot(pub usize);

/// Marker: this card entity is currently selected/highlighted.
#[derive(Component, Debug)]
pub struct Selected;

// ─── ACTIVE SYNERGY ─────────────────────────────────────────

/// An active synergy bonus between two cards in the hand.
#[derive(Debug, Clone)]
pub struct ActiveSynergy {
    pub source_word: String,
    pub target_word: String,
    pub bonus: i32,
    pub flavor_text: String,
}

/// Resource: all currently active synergies in the hand.
#[derive(Resource, Default, Debug)]
pub struct ActiveSynergies {
    pub synergies: Vec<ActiveSynergy>,
}

/// Application states
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    Loading,
    /// Drawing cards into hand at session start or after playing
    Drawing,
    /// Hand is visible, waiting for card selection
    Playing,
    /// A card is selected, showing detail + waiting for action
    CardSelected,
    /// Depth view overlay (dig deeper on selected card)
    DepthView,
    /// Session complete, reviewing the trail
    TrailReview,
}
