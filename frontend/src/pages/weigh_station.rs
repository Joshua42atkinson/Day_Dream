use leptos::prelude::*;

/// Placeholder "Weigh Station" page for assigning cognitive weights
#[component]
pub fn WeighStation() -> impl IntoView {
    view! {
        <div class="p-8">
            <h2 class="text-3xl mb-4 text-[#CFB991]">"⚖️ The Weigh Station"</h2>
            <p class="text-slate-400 mb-6">
                "Assign cognitive weights to vocabulary terms. "
                "Heavy concepts require more 'steam' (cognitive resources) for students to carry."
            </p>

            <div class="bg-slate-950 border border-[#CFB991]/30 rounded-lg p-6">
                <p class="text-center text-slate-500">
                    "Weigh Station implementation coming in Phase 4..."
                </p>
            </div>
        </div>
    }
}
