use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

/// The Student's immersive learning environment ("The Engine Cab")
/// Game-style interface with AI-powered interactions
#[component]
pub fn EngineCabLayout() -> impl IntoView {
    let params = use_params_map();
    let quest_id = move || {
        params.read().get("quest_id").unwrap_or_default()
    };

    view! {
        <div class="engine-cab-layout h-screen bg-gradient-to-b from-slate-900 to-slate-800 text-white">
            // Student HUD with cognitive load gauge
            <header class="border-b border-[#CFB991]/30 bg-black/40 backdrop-blur px-6 py-3">
                <div class="flex items-center justify-between">
                    <div class="flex items-center gap-3">
                        <div class="text-sm text-[#CFB991]/80 uppercase tracking-wider">
                            "Quest: " {move || quest_id()}
                        </div>
                    </div>
                    
                    // Placeholder for Fuel Gauge (Cognitive Load)
                    <div class="w-64 h-3 bg-slate-900 rounded-full border border-[#CFB991]/50">
                        <div class="h-full w-1/3 bg-gradient-to-r from-green-500 to-yellow-500 rounded-full transition-all">
                        </div>
                    </div>
                </div>
            </header>

            // Main game view
            <main class="h-[calc(100vh-60px)] p-6">
                <div class="max-w-4xl mx-auto">
                    <div class="bg-slate-950/50 backdrop-blur border border-[#CFB991]/30 rounded-lg p-8 shadow-2xl">
                        <h2 class="text-3xl mb-6 text-[#CFB991]">"The Journey Begins..."</h2>
                        <p class="text-slate-300 leading-relaxed">
                            "This is the immersive student experience powered by client-side AI. "
                            "Students will interact with the story graph through natural conversation."
                        </p>
                        
                        // Placeholder for chat interface
                        <div class="mt-8 space-y-4">
                            <div class="bg-slate-900/50 rounded p-4 border-l-4 border-[#CFB991]">
                                <p class="text-sm text-slate-400 mb-1">"Pete says:"</p>
                                <p>"Welcome aboard! Ready to explore new concepts?"</p>
                            </div>
                        </div>
                    </div>
                </div>
            </main>
        </div>
    }
}
