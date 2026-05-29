// Daydream Engine — Rendering: The Triple Sandwich + Hand
// Bevy 0.18 API: Required Components pattern (no *Bundle types)
//
// Layout (vertical, mobile-first):
//   TOP:    Active Word Card = Mind (what is this word?)
//   MIDDLE: Story Text = Heart (what does it feel like?)
//   BOTTOM: Hand of Cards = Body (what can I change?)
//
// The hand works like Mad Libs: playing a card from your hand
// swaps the active word, and the story/setting adapts. This
// teaches that CONTEXT changes MEANING — the core VAAM insight.

use bevy::prelude::*;
use bevy::text::Justify;
use crate::components::*;


// ─── MARKER COMPONENTS ──────────────────────────────────────────

#[derive(Component)]
pub struct CardFrame;

#[derive(Component)]
pub struct CardWordText;

#[derive(Component)]
pub struct StoryText;

#[derive(Component)]
pub struct SettingBackground;

#[derive(Component)]
pub struct DepthOverlay;

#[derive(Component)]
pub struct DepthText;

#[derive(Component)]
pub struct SwipeHint;

#[derive(Component)]
pub struct TrailElement;

/// Marks a visual entity as part of the HUD (deck counter, channel bars)
#[derive(Component)]
pub struct HudElement;

/// Marks a visual entity as part of the hand display
#[derive(Component)]
pub struct HandCardVisual;

/// Marks a visual entity as the synergy indicator
#[derive(Component)]
pub struct SynergyIndicator;

// ─── SETUP ──────────────────────────────────────────────────────

/// System: set up camera
pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

// ─── HAND RENDERING ─────────────────────────────────────────────

/// System: spawn the full play view — active card, story, hand, HUD.
/// Runs on entering the Playing state.
pub fn spawn_play_view(
    mut commands: Commands,
    hand: Res<Hand>,
    deck: Res<Deck>,
    slide: Res<CurrentSlide>,
    trail: Res<StudentTrail>,
    sheet: Res<CharacterSheet>,
    synergies: Res<ActiveSynergies>,
    words: Query<(&WordCard, &Channel, &Stage, &CardStyle, &Setting, &SpellPower)>,
    // Clean up any old visuals first
    old_visuals: Query<Entity, Or<(
        With<CardFrame>, With<CardWordText>, With<StoryText>,
        With<SettingBackground>, With<SwipeHint>, With<HudElement>,
        With<HandCardVisual>, With<SynergyIndicator>,
    )>>,
) {
    // Despawn previous frame
    for e in old_visuals.iter() {
        commands.entity(e).despawn();
    }

    // Get the active word (first card played / current context)
    let active_entity = trail.current_word;
    let active_data = active_entity.and_then(|e| words.get(e).ok());

    // ─── SETTING BACKGROUND (full screen tint) ──────────────
    let bg_color = active_data
        .map(|(_, ch, _, _, _, _)| ch.background_color())
        .unwrap_or(Color::srgba(0.04, 0.04, 0.08, 1.0));

    commands.spawn((
        Sprite {
            color: bg_color,
            custom_size: Some(Vec2::new(2000.0, 2000.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -10.0),
        SettingBackground,
    ));

    // ─── TOP: Active Word Card ──────────────────────────────
    if let Some((word, channel, stage, style, _setting, power)) = active_data {
        let card_w = 300.0;
        let card_h = 160.0;
        let card_y = 200.0;

        // Card border (channel color)
        commands.spawn((
            Sprite {
                color: style.color,
                custom_size: Some(Vec2::new(card_w, card_h)),
                ..default()
            },
            Transform::from_xyz(0.0, card_y, 1.0),
            CardFrame,
        ));

        // Card inner fill
        commands.spawn((
            Sprite {
                color: Color::srgba(0.06, 0.06, 0.1, 0.95),
                custom_size: Some(Vec2::new(card_w - 8.0, card_h - 8.0)),
                ..default()
            },
            Transform::from_xyz(0.0, card_y, 1.5),
            CardFrame,
        ));

        // Mastery glow (subtle outline brightens with mastery)
        let glow_alpha = match power.mastery {
            MasteryLevel::Encountered => 0.0,
            MasteryLevel::Experienced => 0.1,
            MasteryLevel::Owned => 0.25,
            MasteryLevel::Mastered => 0.5,
        };
        if glow_alpha > 0.0 {
            commands.spawn((
                Sprite {
                    color: Color::srgba(1.0, 0.95, 0.6, glow_alpha),
                    custom_size: Some(Vec2::new(card_w + 6.0, card_h + 6.0)),
                    ..default()
                },
                Transform::from_xyz(0.0, card_y, 0.9),
                CardFrame,
            ));
        }

        // Word name (large, centered)
        commands.spawn((
            Text2d::new(word.word.to_uppercase()),
            TextFont::from_font_size(32.0),
            TextColor(Color::WHITE),
            Transform::from_xyz(0.0, card_y + 22.0, 2.0),
            CardWordText,
        ));

        // Channel + Stage label
        let label = format!(
            "{} {} {}",
            channel.label(), stage.stars(), stage.label()
        );
        commands.spawn((
            Text2d::new(label),
            TextFont::from_font_size(11.0),
            TextColor(channel.color()),
            Transform::from_xyz(0.0, card_y, 2.0),
            CardWordText,
        ));

        // Theme tags
        let themes_str = word.themes.join(" · ");
        commands.spawn((
            Text2d::new(themes_str),
            TextFont::from_font_size(11.0),
            TextColor(Color::srgba(1.0, 1.0, 1.0, 0.35)),
            Transform::from_xyz(0.0, card_y - 18.0, 2.0),
            CardWordText,
        ));

        // Channel question (the soul of this card type)
        commands.spawn((
            Text2d::new(channel.question()),
            TextFont::from_font_size(10.0),
            TextColor(Color::srgba(1.0, 1.0, 1.0, 0.15)),
            Transform::from_xyz(0.0, card_y - 35.0, 2.0),
            CardWordText,
        ));
    }

    // ─── MIDDLE: Story Text ─────────────────────────────────
    commands.spawn((
        Text2d::new(&slide.story_text),
        TextFont::from_font_size(17.0),
        TextColor(Color::srgba(1.0, 1.0, 1.0, 0.8)),
        TextLayout::new_with_justify(Justify::Center),
        Transform::from_xyz(0.0, 30.0, 2.0),
        StoryText,
    ));

    // ─── SYNERGY INDICATOR ──────────────────────────────────
    if !synergies.synergies.is_empty() {
        let syn_text: Vec<String> = synergies.synergies.iter()
            .map(|s| format!("⚡ {} + {} = +{}", s.source_word, s.target_word, s.bonus))
            .collect();
        commands.spawn((
            Text2d::new(syn_text.join("  ")),
            TextFont::from_font_size(11.0),
            TextColor(Color::srgba(1.0, 0.85, 0.3, 0.7)),
            TextLayout::new_with_justify(Justify::Center),
            Transform::from_xyz(0.0, -30.0, 2.0),
            SynergyIndicator,
        ));
    }

    // ─── BOTTOM: Hand of Cards ──────────────────────────────
    let hand_count = hand.card_count();
    if hand_count > 0 {
        let card_w = 90.0;
        let card_h = 120.0;
        let spacing = 105.0;
        let hand_y = -180.0;
        let total_width = (hand_count as f32 - 1.0) * spacing;
        let start_x = -total_width / 2.0;

        for (i, entity) in hand.cards.iter().enumerate() {
            if let Ok((word, channel, _stage, style, _setting, _power)) = words.get(*entity) {
                let x = start_x + i as f32 * spacing;
                let is_selected = hand.selected == Some(i);

                // Selected card lifts up and grows
                let y_offset = if is_selected { 20.0 } else { 0.0 };
                let scale = if is_selected { 1.15 } else { 1.0 };

                // Card border
                let border_color = if is_selected {
                    Color::WHITE
                } else {
                    style.color
                };

                commands.spawn((
                    Sprite {
                        color: border_color,
                        custom_size: Some(Vec2::new(card_w, card_h)),
                        ..default()
                    },
                    Transform::from_xyz(x, hand_y + y_offset, 3.0)
                        .with_scale(Vec3::splat(scale)),
                    HandCardVisual,
                    HandSlot(i),
                ));

                // Card inner fill
                commands.spawn((
                    Sprite {
                        color: Color::srgba(0.06, 0.06, 0.1, 0.95),
                        custom_size: Some(Vec2::new(card_w - 6.0, card_h - 6.0)),
                        ..default()
                    },
                    Transform::from_xyz(x, hand_y + y_offset, 3.5)
                        .with_scale(Vec3::splat(scale)),
                    HandCardVisual,
                ));

                // Card word name
                commands.spawn((
                    Text2d::new(word.word.to_uppercase()),
                    TextFont::from_font_size(14.0),
                    TextColor(Color::WHITE),
                    Transform::from_xyz(x, hand_y + y_offset + 20.0, 4.0),
                    HandCardVisual,
                ));

                // Channel color dot + label
                let ch_label = format!("{}", channel.label());
                commands.spawn((
                    Text2d::new(ch_label),
                    TextFont::from_font_size(10.0),
                    TextColor(channel.color()),
                    Transform::from_xyz(x, hand_y + y_offset - 5.0, 4.0),
                    HandCardVisual,
                ));

                // Slot number hint (keyboard shortcut)
                commands.spawn((
                    Text2d::new(format!("{}", i + 1)),
                    TextFont::from_font_size(9.0),
                    TextColor(Color::srgba(1.0, 1.0, 1.0, 0.2)),
                    Transform::from_xyz(x, hand_y + y_offset - 25.0, 4.0),
                    HandCardVisual,
                ));
            }
        }
    }

    // ─── SWIPE HINTS ────────────────────────────────────────
    let hint_text = if hand.selected.is_some() {
        "→ play    ← skip    ↓ dig deeper    [esc] deselect"
    } else {
        "tap 1-3 to select a card"
    };
    commands.spawn((
        Text2d::new(hint_text),
        TextFont::from_font_size(11.0),
        TextColor(Color::srgba(1.0, 1.0, 1.0, 0.2)),
        TextLayout::new_with_justify(Justify::Center),
        Transform::from_xyz(0.0, -270.0, 2.0),
        SwipeHint,
    ));

    // ─── HUD: Top bar ───────────────────────────────────────
    // Deck counter (top-left)
    commands.spawn((
        Text2d::new(format!("⟐ {}", deck.remaining())),
        TextFont::from_font_size(13.0),
        TextColor(Color::srgba(1.0, 1.0, 1.0, 0.4)),
        Transform::from_xyz(-280.0, 300.0, 2.0),
        HudElement,
    ));

    // Channel attunement mini-bars (top-right)
    let attn_vals = [
        ("🟢", sheet.mind_attunement),
        ("🟠", sheet.heart_attunement),
        ("🔵", sheet.body_attunement),
        ("🟡", sheet.action_attunement),
    ];
    let max_val = attn_vals.iter().map(|(_, v)| *v).fold(0.01_f32, f32::max);
    let bar_text: Vec<String> = attn_vals.iter().map(|(icon, val)| {
        let bars = (val / max_val * 5.0).round() as usize;
        let filled = "█".repeat(bars);
        let empty = "░".repeat(5 - bars);
        format!("{}{}{}", icon, filled, empty)
    }).collect();

    commands.spawn((
        Text2d::new(bar_text.join(" ")),
        TextFont::from_font_size(10.0),
        TextColor(Color::srgba(1.0, 1.0, 1.0, 0.35)),
        Transform::from_xyz(180.0, 300.0, 2.0),
        HudElement,
    ));
}

// ─── DEPTH OVERLAY ──────────────────────────────────────────────

/// System: spawn depth overlay when entering DepthView state.
pub fn spawn_depth_view(
    mut commands: Commands,
    hand: Res<Hand>,
    trail: Res<StudentTrail>,
    words: Query<(&WordCard, &Channel)>,
    mut spellbook: ResMut<SpellBook>,
    mut spell_powers: Query<&mut SpellPower>,
    mut sheet: ResMut<CharacterSheet>,
) {
    // Figure out which word to show depth for
    let target_entity = if let Some(idx) = hand.selected {
        hand.cards.get(idx).copied()
    } else {
        trail.current_word
    };

    let Some(entity) = target_entity else { return };
    let Ok((word, channel)) = words.get(entity) else { return };

    // Track the deeper interaction
    if let Ok(mut sp) = spell_powers.get_mut(entity) {
        sp.times_explored_deeper += 1;
        if sp.mastery == MasteryLevel::Encountered {
            sp.mastery = MasteryLevel::Experienced;
            spellbook.upgrade_mastery(&word.word, MasteryLevel::Experienced);
        }
    }
    sheet.total_deeper_swipes += 1;

    // Dark overlay
    commands.spawn((
        Sprite {
            color: Color::srgba(0.02, 0.02, 0.05, 0.92),
            custom_size: Some(Vec2::new(2000.0, 2000.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 5.0),
        DepthOverlay,
    ));

    // Word title — colored by channel
    commands.spawn((
        Text2d::new(word.word.to_uppercase()),
        TextFont::from_font_size(40.0),
        TextColor(channel.color()),
        TextLayout::new_with_justify(Justify::Center),
        Transform::from_xyz(0.0, 80.0, 6.0),
        DepthText,
    ));

    // "Dig Deeper" label
    commands.spawn((
        Text2d::new("— dig deeper —"),
        TextFont::from_font_size(12.0),
        TextColor(Color::srgba(1.0, 1.0, 1.0, 0.3)),
        TextLayout::new_with_justify(Justify::Center),
        Transform::from_xyz(0.0, 50.0, 6.0),
        DepthText,
    ));

    // Depth question — the VAAM experiential prompt
    commands.spawn((
        Text2d::new(&word.depth_prompt),
        TextFont::from_font_size(18.0),
        TextColor(Color::srgba(1.0, 1.0, 1.0, 0.7)),
        TextLayout::new_with_justify(Justify::Center),
        Transform::from_xyz(0.0, -10.0, 6.0),
        DepthText,
    ));

    // Dismiss hint
    commands.spawn((
        Text2d::new("[esc / space to return]"),
        TextFont::from_font_size(11.0),
        TextColor(Color::srgba(1.0, 1.0, 1.0, 0.2)),
        TextLayout::new_with_justify(Justify::Center),
        Transform::from_xyz(0.0, -100.0, 6.0),
        DepthText,
    ));
}

/// System: dismiss depth overlay and clean up entities
pub fn dismiss_depth_view(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut slide: ResMut<CurrentSlide>,
    mut next_state: ResMut<NextState<GameState>>,
    depth_entities: Query<Entity, Or<(With<DepthOverlay>, With<DepthText>)>>,
    mut commands: Commands,
) {
    if keys.just_pressed(KeyCode::Escape) || keys.just_pressed(KeyCode::Space)
        || mouse.just_pressed(MouseButton::Left)
    {
        slide.depth_showing = false;
        for e in depth_entities.iter() {
            commands.entity(e).despawn();
        }
        // Return to card selected if a card was selected, otherwise playing
        next_state.set(GameState::CardSelected);
    }
}

// ─── TRAIL REVIEW ───────────────────────────────────────────────

/// System: spawn end-of-journey trail review with Character Sheet.
pub fn spawn_trail_review(
    mut commands: Commands,
    trail: Res<StudentTrail>,
    sheet: Res<CharacterSheet>,
    spellbook: Res<SpellBook>,
    // Clean up play view
    old_visuals: Query<Entity, Or<(
        With<CardFrame>, With<CardWordText>, With<StoryText>,
        With<SettingBackground>, With<SwipeHint>, With<HudElement>,
        With<HandCardVisual>, With<SynergyIndicator>,
    )>>,
) {
    for e in old_visuals.iter() {
        commands.entity(e).despawn();
    }

    // Dark background
    commands.spawn((
        Sprite {
            color: Color::srgba(0.03, 0.03, 0.06, 1.0),
            custom_size: Some(Vec2::new(2000.0, 2000.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -10.0),
        TrailElement,
    ));

    // Title
    commands.spawn((
        Text2d::new("Your journey, traced."),
        TextFont::from_font_size(36.0),
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(Justify::Center),
        Transform::from_xyz(0.0, 250.0, 2.0),
        TrailElement,
    ));

    // Trail path with arrows
    let symbols: Vec<String> = trail.visited_words.iter().enumerate().map(|(i, w)| {
        if i < trail.swipe_history.len() {
            let arrow = match trail.swipe_history[i] {
                SwipeChoice::Yes => "→",
                SwipeChoice::No => "←",
                SwipeChoice::Deeper => "↓",
            };
            format!("{} {}", w, arrow)
        } else {
            w.clone()
        }
    }).collect();

    commands.spawn((
        Text2d::new(symbols.join("  ")),
        TextFont::from_font_size(14.0),
        TextColor(Color::srgba(1.0, 1.0, 1.0, 0.5)),
        TextLayout::new_with_justify(Justify::Center),
        Transform::from_xyz(0.0, 190.0, 2.0),
        TrailElement,
    ));

    // Emergent class title
    commands.spawn((
        Text2d::new(format!("You walk the path of: {}", sheet.emergent_class)),
        TextFont::from_font_size(20.0),
        TextColor(Color::srgba(1.0, 0.85, 0.5, 1.0)),
        TextLayout::new_with_justify(Justify::Center),
        Transform::from_xyz(0.0, 130.0, 2.0),
        TrailElement,
    ));

    // Channel attunement display
    commands.spawn((
        Text2d::new(sheet.attunement_display()),
        TextFont::from_font_size(13.0),
        TextColor(Color::srgba(1.0, 1.0, 1.0, 0.6)),
        TextLayout::new_with_justify(Justify::Center),
        Transform::from_xyz(0.0, 60.0, 2.0),
        TrailElement,
    ));

    // SpellBook summary
    commands.spawn((
        Text2d::new(spellbook.summary()),
        TextFont::from_font_size(13.0),
        TextColor(Color::srgba(1.0, 1.0, 1.0, 0.5)),
        TextLayout::new_with_justify(Justify::Center),
        Transform::from_xyz(0.0, -30.0, 2.0),
        TrailElement,
    ));

    // Journey insight
    let yes_count = trail.swipe_history.iter().filter(|s| matches!(s, SwipeChoice::Yes)).count();
    let no_count = trail.swipe_history.iter().filter(|s| matches!(s, SwipeChoice::No)).count();
    let deep_count = trail.swipe_history.iter().filter(|s| matches!(s, SwipeChoice::Deeper)).count();

    let pattern = if deep_count >= yes_count && deep_count >= no_count {
        "You asked 'why' more than 'what.' You look beneath the surface.\nThe Vulnerability skill is strong in you."
    } else if yes_count >= no_count {
        "You moved forward with trust. Your instinct is to explore.\nThe Ownership skill guides your path."
    } else {
        "You paused when uncertain. Wisdom often sounds like silence.\nThe Stewardship skill is your compass."
    };

    commands.spawn((
        Text2d::new(pattern),
        TextFont::from_font_size(14.0),
        TextColor(Color::srgba(1.0, 1.0, 1.0, 0.45)),
        TextLayout::new_with_justify(Justify::Center),
        Transform::from_xyz(0.0, -120.0, 2.0),
        TrailElement,
    ));

    // Stats
    let stats_text = format!(
        "Words encountered: {}  |  Deeper dives: {}",
        sheet.words_encountered, sheet.total_deeper_swipes
    );
    commands.spawn((
        Text2d::new(stats_text),
        TextFont::from_font_size(11.0),
        TextColor(Color::srgba(1.0, 1.0, 1.0, 0.3)),
        TextLayout::new_with_justify(Justify::Center),
        Transform::from_xyz(0.0, -200.0, 2.0),
        TrailElement,
    ));
}
