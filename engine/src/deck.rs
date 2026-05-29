// Daydream Engine — Deck Mechanics
// TCG draw/play cycle for the ARCANA word-spell card game.
//
// The deck/hand system works like fine-tuning:
//   Deck = training dataset (all words to encounter)
//   Hand = current batch (active cards to engage with)
//   Playing a card = processing one training example
//   Synergy = multi-task learning bonus

use bevy::prelude::*;
use crate::components::*;

// ─── DECK INITIALIZATION ────────────────────────────────────

/// System: shuffle all word entities into the deck at session start.
/// Runs once on entering the Drawing state.
pub fn init_deck(
    mut deck: ResMut<Deck>,
    words: Query<Entity, With<WordCard>>,
) {
    if !deck.cards.is_empty() {
        return; // Already initialized
    }

    let mut all_words: Vec<Entity> = words.iter().collect();

    // Simple deterministic shuffle using entity index bits
    // (We avoid rand dependency to keep the build minimal)
    let len = all_words.len();
    if len > 1 {
        for i in (1..len).rev() {
            let j = (i * 7 + 3) % (i + 1);
            all_words.swap(i, j);
        }
    }

    deck.cards = all_words;
    info!("Deck initialized with {} cards", deck.remaining());
}

/// System: draw cards from deck into hand until hand is full.
/// Transitions to Playing state when hand is ready.
pub fn draw_cards(
    mut deck: ResMut<Deck>,
    mut hand: ResMut<Hand>,
    mut trail: ResMut<StudentTrail>,
    mut spellbook: ResMut<SpellBook>,
    mut sheet: ResMut<CharacterSheet>,
    words: Query<(&WordCard, &Channel)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    while !hand.is_full() {
        if let Some(entity) = deck.draw() {
            hand.cards.push(entity);

            // Record encounter in SpellBook + CharacterSheet
            if let Ok((word, channel)) = words.get(entity) {
                spellbook.record_encounter(&word.word, *channel);
                sheet.engage_channel(channel);
                sheet.words_encountered += 1;

                if !trail.visited_words.contains(&word.word) {
                    trail.visited_words.push(word.word.clone());
                }
            }
        } else {
            break; // Deck is empty
        }
    }

    if hand.card_count() > 0 {
        hand.selected = None;
        next_state.set(GameState::Playing);
    } else {
        // No cards left — session is over
        next_state.set(GameState::TrailReview);
    }
}

// ─── CARD SELECTION ─────────────────────────────────────────

/// System: select a card by keyboard number (1-5).
pub fn select_card_by_key(
    keys: Res<ButtonInput<KeyCode>>,
    mut hand: ResMut<Hand>,
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
) {
    if *state.get() != GameState::Playing { return; }

    let key_map = [
        (KeyCode::Digit1, 0),
        (KeyCode::Digit2, 1),
        (KeyCode::Digit3, 2),
        (KeyCode::Digit4, 3),
        (KeyCode::Digit5, 4),
    ];

    for (key, index) in key_map {
        if keys.just_pressed(key) && index < hand.card_count() {
            hand.selected = Some(index);
            next_state.set(GameState::CardSelected);
            return;
        }
    }

    // Escape deselects
    if keys.just_pressed(KeyCode::Escape) {
        hand.selected = None;
    }
}

// ─── CARD PLAY / DISCARD ────────────────────────────────────

/// System: handle swipe actions on the selected card.
/// RIGHT = cast (play into story), LEFT = discard, DOWN = dig deeper.
pub fn handle_card_action(
    keys: Res<ButtonInput<KeyCode>>,
    mut hand: ResMut<Hand>,
    mut discard: ResMut<DiscardPile>,
    mut trail: ResMut<StudentTrail>,
    mut slide: ResMut<CurrentSlide>,
    mut spellbook: ResMut<SpellBook>,
    mut sheet: ResMut<CharacterSheet>,
    words: Query<(&WordCard, &Channel, &WordEdges, &SpellPower)>,
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
) {
    if *state.get() != GameState::CardSelected { return; }

    let action = if keys.just_pressed(KeyCode::ArrowRight) || keys.just_pressed(KeyCode::KeyD) {
        Some(SwipeChoice::Yes)
    } else if keys.just_pressed(KeyCode::ArrowLeft) || keys.just_pressed(KeyCode::KeyA) {
        Some(SwipeChoice::No)
    } else if keys.just_pressed(KeyCode::ArrowDown) || keys.just_pressed(KeyCode::KeyS)
        || keys.just_pressed(KeyCode::Space) {
        Some(SwipeChoice::Deeper)
    } else if keys.just_pressed(KeyCode::Escape) {
        // Deselect — go back to hand view
        hand.selected = None;
        next_state.set(GameState::Playing);
        return;
    } else {
        None
    };

    let Some(choice) = action else { return };

    match choice {
        SwipeChoice::Yes => {
            // CAST: play the card into the story
            if let Some(entity) = hand.play_selected() {
                if let Ok((word, channel, _edges, _power)) = words.get(entity) {
                    // Update trail
                    trail.current_word = Some(entity);
                    trail.swipe_history.push(SwipeChoice::Yes);

                    // Update story
                    slide.story_text = get_play_text(&word.word);
                    slide.ready_for_input = true;

                    // Advance mastery
                    sheet.engage_channel(channel);
                    spellbook.record_encounter(&word.word, *channel);
                }

                discard.cards.push(entity);

                // Draw to refill hand
                next_state.set(GameState::Drawing);
            }
        },
        SwipeChoice::No => {
            // DISCARD: skip this card, draw a new one
            if let Some(entity) = hand.play_selected() {
                if let Ok((word, _ch, _edges, _power)) = words.get(entity) {
                    trail.swipe_history.push(SwipeChoice::No);
                }
                discard.cards.push(entity);
                next_state.set(GameState::Drawing);
            }
        },
        SwipeChoice::Deeper => {
            // DIG DEEPER: show depth overlay for selected card
            slide.depth_showing = true;
            next_state.set(GameState::DepthView);
        },
    }
}

/// System: dismiss depth view and return to card selected state.
pub fn dismiss_depth(
    keys: Res<ButtonInput<KeyCode>>,
    mut slide: ResMut<CurrentSlide>,
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
) {
    if *state.get() != GameState::DepthView { return; }

    if keys.just_pressed(KeyCode::Escape) || keys.just_pressed(KeyCode::Space)
        || keys.just_pressed(KeyCode::ArrowUp)
    {
        slide.depth_showing = false;
        next_state.set(GameState::CardSelected);
    }
}

// ─── SYNERGY DETECTION ──────────────────────────────────────

/// System: check for synergies between cards in the current hand.
pub fn detect_synergies(
    hand: Res<Hand>,
    words: Query<(&WordCard, &SynergyLinks)>,
    mut synergies: ResMut<ActiveSynergies>,
) {
    synergies.synergies.clear();

    // Build a set of words currently in hand
    let hand_words: Vec<(Entity, String)> = hand.cards.iter()
        .filter_map(|e| words.get(*e).ok().map(|(w, _)| (*e, w.word.clone())))
        .collect();

    // Check each card's synergy partners against the hand
    for (entity, _word_name) in &hand_words {
        if let Ok((word, links)) = words.get(*entity) {
            for entry in &links.links {
                if hand_words.iter().any(|(_, w)| w == &entry.partner) {
                    synergies.synergies.push(ActiveSynergy {
                        source_word: word.word.clone(),
                        target_word: entry.partner.clone(),
                        bonus: 2,
                        flavor_text: format!(
                            "{} + {} resonate together",
                            word.word, entry.partner
                        ),
                    });
                }
            }
        }
    }
}

// ─── HELPERS ────────────────────────────────────────────────

/// Get the story text when a card is played.
/// In the future this comes from the curriculum JSON or LLM.
fn get_play_text(word: &str) -> String {
    match word {
        "Presence" => "You step through the stone archway. The warm light \
            envelops you. For a moment, everything is still.".to_string(),
        "Bias" => "The glass trees reflect a version of you that isn't \
            quite right. What are you not seeing?".to_string(),
        "Patience" => "The garden fountain whispers. A feather lands on \
            the water. You realize you've been holding your breath.".to_string(),
        "Resilience" => "The bridge sways beneath your feet. The canyon \
            wind howls. You take another step forward.".to_string(),
        "Clarity" => "Above the clouds, the whole world is a map below \
            you. Everything connects. Everything makes sense.".to_string(),
        _ => format!("You cast the spell of {}. The world shifts around you.", word),
    }
}
