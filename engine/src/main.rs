// Daydream Engine — Main Entry Point
//
// Letters SPELL words. Words are spells. The Great Game is the curriculum.
//
// A TCG-style meaning-making engine where:
// - Each word is an ECS entity — a spell card with Channel, Stage, and Symbol
// - The student holds a hand of cards and plays them into a story
// - Playing a card swaps the active word — like Mad Libs, context changes meaning
// - The Triple Sandwich (Mind/Heart/Body) maps to card/story/setting
// - Swipe right = cast, left = skip, down = dig deeper

mod components;
mod dag;
mod deck;
mod input;
mod render;

use bevy::prelude::*;
use components::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Daydream — The Great Game".to_string(),
                resolution: bevy::window::WindowResolution::new(700, 900),
                canvas: Some("#daydream-canvas".to_string()),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: true,
                ..default()
            }),
            ..default()
        }))
        // State
        .init_state::<GameState>()
        // Resources — TCG deck/hand system
        .init_resource::<Deck>()
        .init_resource::<Hand>()
        .init_resource::<DiscardPile>()
        .init_resource::<ActiveSynergies>()
        .init_resource::<input::DragState>()
        .init_resource::<input::PendingSwipe>()
        // Startup: camera + curriculum data
        .add_systems(Startup, (
            render::setup_camera,
            dag::spawn_demo_curriculum,
        ))
        // Loading → Drawing: init the deck after curriculum spawns
        .add_systems(OnEnter(GameState::Loading), (
            deck::init_deck.after(dag::spawn_demo_curriculum),
        ))
        // Drawing: fill hand from deck, then transition to Playing
        .add_systems(OnEnter(GameState::Drawing),
            deck::draw_cards,
        )
        // Playing: show hand, detect card selection + synergies
        .add_systems(OnEnter(GameState::Playing),
            render::spawn_play_view,
        )
        .add_systems(Update, (
            deck::select_card_by_key,
            deck::detect_synergies,
        ).run_if(in_state(GameState::Playing)))
        // CardSelected: handle cast/skip/deeper actions
        .add_systems(OnEnter(GameState::CardSelected),
            render::spawn_play_view,
        )
        .add_systems(Update,
            deck::handle_card_action
                .run_if(in_state(GameState::CardSelected)),
        )
        // DepthView: show depth overlay, dismiss on keypress
        .add_systems(OnEnter(GameState::DepthView),
            render::spawn_depth_view,
        )
        .add_systems(Update,
            render::dismiss_depth_view
                .run_if(in_state(GameState::DepthView)),
        )
        // TrailReview: end-of-session summary
        .add_systems(OnEnter(GameState::TrailReview),
            render::spawn_trail_review,
        )
        .run();
}
