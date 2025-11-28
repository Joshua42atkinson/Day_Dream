use crate::components::glass_panel::GlassPanel;
use crate::components::systems_thinking::SystemsThinkingChallenge;
use crate::components::theoretical_synthesis::KeyConceptsChallenge;
use leptos::prelude::*; // Ensure GlassPanel is imported if needed by the page itself
#[component]
pub fn Foundations() -> impl IntoView {
    view! {
    <div class="max-w-5xl mx-auto w-full space-y-20 animate-fade-in p-6">
    <div class="text-center space-y-6">
    <h1 class="text-4xl font-bold text-white">"Professional Foundations"</h1>
    <p class="text-slate-400">"Core competencies in Instructional Design standards and ethics."</p>
    </div>
    <KeyConceptsChallenge />
    <div class="h-px bg-white/10 w-full"></div>
    <SystemsThinkingChallenge />
    </div>
    }
}
