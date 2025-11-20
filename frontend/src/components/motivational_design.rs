use leptos::*;
use crate::components::glass_panel::GlassPanel;
#[component]
pub fn MotivationalDesignChallenge() -> impl IntoView {
view! {
<div class="space-y-12 animate-fade-in relative">
<div class="border-l-4 border-pink-500 pl-6 py-4 mb-8">
<h2 class="text-4xl font-extrabold text-white tracking-tight">"Motivational Design"</h2>
<div class="flex flex-wrap items-center gap-3 mt-3">
<span class="px-3 py-1 rounded-full bg-pink-500/20 text-pink-300 text-xs font-bold uppercase tracking-widest border border-pink-500/30">"Design Instructional Interventions"</span>
<span class="text-slate-400 text-sm flex items-center gap-2">
<span class="w-1.5 h-1.5 rounded-full bg-pink-500"></span>
"Challenge: Apply Motivational Principles"
</span>
</div>
</div>
<div class="relative z-10">
<GlassPanel class="bg-gradient-to-br from-slate-800/80 to-pink-900/20 border-pink-500/20">
<div class="grid grid-cols-1 md:grid-cols-12 gap-8">
<div class="md:col-span-3 flex flex-col items-center justify-center border-b md:border-b-0 md:border-r border-white/10 pb-6 md:pb-0 md:pr-6">
<div class="w-20 h-20 rounded-2xl bg-pink-500/20 flex items-center justify-center text-pink-400 mb-4 shadow-lg shadow-pink-900/30 ring-1 ring-pink-500/30">
<svg class="w-10 h-10" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"></path></svg>
</div>
<div class="text-xs font-mono text-pink-300/80 uppercase tracking-widest">"Research Analysis"</div>
</div>
<div class="md:col-span-9 space-y-6">
<div>
<h3 class="text-2xl font-bold text-white mb-2">"The Engine of Enjoyment"</h3>
<p class="text-sm text-pink-200/70 font-medium">"Deep Gamification Architecture"</p>
</div>
<div class="bg-slate-900/50 rounded-lg p-4 border border-white/5 relative group">
<div class="absolute top-0 left-0 w-1 h-full bg-pink-500 rounded-l-lg opacity-50 group-hover:opacity-100 transition-opacity"></div>
<p class="text-slate-300 text-sm leading-relaxed italic">
"This research analysis critiques the 'Motivation Paradox' inherent in shallow gamification (points and badges).
It proposes a 'Tier 3+' architecture based on Self-Determination Theory, where a 'Virtuous Cycle' of Identity, Competence, and Autonomy replaces extrinsic rewards with a self-reinforcing motivation engine."
</p>
</div>
<div class="flex justify-start pt-2">
<a href="https://docs.google.com/document/d/16q5E9qM_l-tV7O4zQ8uC5a6-Qk7tO9q/edit?usp=sharing" target="_blank" class="group inline-flex items-center gap-3 px-5 py-2.5 rounded-lg bg-pink-600 hover:bg-pink-500 text-white text-sm font-semibold transition-all shadow-lg shadow-pink-900/30 hover:shadow-pink-700/50">
<span>"Read Analysis"</span>
<svg class="w-4 h-4 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3"></path></svg>
</a>
</div>
</div>
</div>
</GlassPanel>
</div>
<div class="relative z-10 pl-0 md:pl-16">
<div class="flex items-center gap-4 mb-6">
<div class="w-10 h-10 rounded-full bg-slate-800 border border-cyan-500/30 flex items-center justify-center text-cyan-400 shadow-lg">
<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg>
</div>
<h3 class="text-2xl font-bold text-cyan-100">"Competency Reflection"</h3>
</div>
<div class="prose prose-invert prose-lg max-w-none text-slate-300 leading-relaxed space-y-6">
<p> "For this artifact, I applied <strong class='text-white'>Self-Determination Theory (SDT)</strong> as the primary motivational framework, explicitly rejecting the industry-standard 'shallow gamification' model.
Research indicates that extrinsic rewards (points, badges) often lead to the 'Motivation Paradox,' where external incentives actually decrease intrinsic drive.
To counter this, I designed a 'Motivation Engine' that systematically targets the three universal psychological needs: <span class='text-pink-300 font-medium'>Autonomy</span> (control), <span class='text-pink-300 font-medium'>Competence</span> (mastery), and <span class='text-pink-300 font-medium'>Relatedness</span> (connection)."
</p>
<p> "I operationalized this theory through the <strong class='text-white'>'Virtuous Cycle,'</strong> a closed-loop system where every component reinforces the others.
The 'Persona Engine' fulfills Autonomy by allowing learners to choose their identity (e.g., 'Sage').
This choice is mechanically reinforced by 'LitRPG Stats' (+2 Intelligence), which boosts performance in the 'VaaM Loop' (Competence).
Finally, the 'AI as a Mirror' validates this success as an expression of the learner's true self (Relatedness)."
</p>
<p> "This design demonstrates a shift from 'gamification' as a cosmetic layer to a pedagogical imperative.
By replacing 'points-ification' with <span class='text-pink-300 font-medium'>'Deep Gamification,'</span> the system moves from a model of compulsion (hooking users with tricks) to a model of volition (supporting internal drive)."
</p>
</div>
</div>
</div>
}
}