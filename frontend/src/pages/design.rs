use crate::components::design_process::DesignProcessChallenge;
use crate::components::motivational_design::MotivationalDesignChallenge;
use crate::components::tech_skills::TechSkillsChallenge;
use crate::components::visual_design::VisualDesignChallenge;
use leptos::prelude::*; // Retain existing component
#[component]
pub fn Design() -> impl IntoView {
    view! {
    <div class="max-w-5xl mx-auto w-full space-y-20 animate-fade-in p-6">
    <div class="text-center space-y-6">
    <h1 class="text-4xl font-bold text-white">"Design & Development"</h1>
    <p class="text-slate-400">"Systematic design, material development, and assessment creation."</p>
    </div>
    <DesignProcessChallenge />
    <div class="h-px bg-white/10 w-full"></div>
    <MotivationalDesignChallenge />
    <div class="h-px bg-white/10 w-full"></div>
    <TechSkillsChallenge />
    <div class="h-px bg-white/10 w-full"></div>
    <VisualDesignChallenge />
    </div>
    }
}
