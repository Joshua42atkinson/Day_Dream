// frontend/src/components/persistent_ui.rs
// A sidebar panel for the "Learner Preview".

use leptos::*;

/// This component displays a persistent sidebar with placeholder data
/// representing the current state of the learner's experience.
#[component]
pub fn PersistentUiPanel() -> impl IntoView {
    // --- Placeholder Data ---
    // This data will eventually be driven by the live application state.
    let character_name = "Totem";
    let character_status = "Observing";
    let quest_title = "First Impressions";
    let quest_objective = "Look around the Town Square. Describe what catches your eye first.";
    let inventory = vec!["Trembling Porcupine Companion"];

    view! {
        // This outer div is a fixed-position container for the sidebar.
        <div class="fixed top-16 right-0 h-full w-72 p-4">
            <div class="leptos-panel border border-gray-700 rounded-lg p-6 shadow-lg h-full">
                <h2 class="text-xl font-semibold mb-4 text-cyan-400">"Learner Preview"</h2>

                // Character Status Section
                <div class="mb-6">
                    <h3 class="text-lg font-medium text-white mb-2">"Status"</h3>
                    <p class="text-gray-300"><strong>Name:</strong> {character_name}</p>
                    <p class="text-gray-300"><strong>Status:</strong> {character_status}</p>
                </div>

                // Quest Objective Section
                <div class="mb-6">
                    <h3 class="text-lg font-medium text-white mb-2">"Objective"</h3>
                    <p class="text-cyan-300 font-semibold">{quest_title}</p>
                    <p class="text-gray-300">{quest_objective}</p>
                </div>

                // Inventory Section
                <div>
                    <h3 class="text-lg font-medium text-white mb-2">"Inventory"</h3>
                    <ul class="list-disc list-inside text-gray-300">
                        {inventory.into_iter()
                            .map(|item| view! { <li>{item}</li>})
                            .collect_view()
                        }
                    </ul>
                </div>
            </div>
        </div>
    }
}
