use leptos::*;
use crate::components::learner_analysis::TargetPopulationChallenge;
use crate::components::tech_analysis::AnalyzeTechChallenge;
use crate::components::ethical_legal::EthicsConstraintsChallenge;
use crate::components::glass_panel::GlassPanel; // Ensure GlassPanel is imported if needed by the page itself
#[component]
pub fn Planning() -> impl IntoView {
view! {
<div class="max-w-5xl mx-auto w-full space-y-20 animate-fade-in p-6">
<div class="text-center space-y-6">
<h1 class="text-4xl font-bold text-white">"Planning & Analysis"</h1>
<p class="text-slate-400">"Gap analysis, target population assessment, and technology selection."</p>
</div>
<TargetPopulationChallenge />
<div class="h-px bg-white/10 w-full"></div>
<AnalyzeTechChallenge />
<div class="h-px bg-white/10 w-full"></div>
<EthicsConstraintsChallenge />
</div>
}
}