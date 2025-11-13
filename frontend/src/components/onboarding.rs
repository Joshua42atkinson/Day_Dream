// frontend/src/components/onboarding.rs
// The UI component for the guided onboarding tutorial.

use leptos::*;

/// A static component that displays the initial steps of the onboarding tutorial.
/// This is designed to help new Instructional Designers understand the core concepts.
#[component]
pub fn OnboardingTutorial() -> impl IntoView {
    view! {
        <div class="leptos-panel border border-gray-700 rounded-lg p-6 mt-8">
            <h2 class="text-2xl font-semibold mb-4 text-cyan-400">"Getting Started Guide"</h2>
            <p class="text-gray-300 mb-4">
                "Welcome to the Daydream Creator's Sandbox! This is a space for you to build narrative-driven learning experiences. Here are the core concepts:"
            </p>
            <ol class="list-decimal list-inside space-y-3 text-gray-300">
                <li>
                    <span class="font-semibold text-white">"Characters:"</span>
                    " Define the actors in your story. The pre-made characters below are your starting point."
                </li>
                <li>
                    <span class="font-semibold text-white">"Quests:"</span>
                    " A 'Quest' is a learning module. It has a goal, steps, and rewards."
                </li>
                <li>
                    <span class="font-semibold text-white">"Game View:"</span>
                    " Use the 'Game' tab to playtest your creation and see it from the learner's perspective."
                </li>
            </ol>
            <p class="mt-4 text-gray-400">
                "Your goal for Phase 1 is to familiarize yourself with these tools to solve the 'blank page' problem."
            </p>
        </div>
    }
}
