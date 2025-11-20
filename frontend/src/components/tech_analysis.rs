use leptos::*;
use crate::components::glass_panel::GlassPanel;
#[component]
pub fn AnalyzeTechChallenge() -> impl IntoView {
view! {
<div class="space-y-12 animate-fade-in relative">
<div class="border-l-4 border-violet-500 pl-6 py-4 mb-8">
<h2 class="text-4xl font-extrabold text-white tracking-tight">"Strategic Tech Analysis"</h2>
<div class="flex flex-wrap items-center gap-3 mt-3">
<span class="px-3 py-1 rounded-full bg-violet-500/20 text-violet-300 text-xs font-bold uppercase tracking-widest border border-violet-500/30">"Planning & Analysis"</span>
<span class="text-slate-400 text-sm flex items-center gap-2">
<span class="w-1.5 h-1.5 rounded-full bg-violet-500"></span>
"Challenge: Analyze Existing & Emerging Technologies"
</span>
</div>
</div>
<div class="relative z-10">
<GlassPanel class="bg-gradient-to-br from-slate-800/80 to-violet-900/20 border-violet-500/20">
<div class="grid grid-cols-1 md:grid-cols-12 gap-8">
<div class="md:col-span-3 flex flex-col items-center justify-center border-b md:border-b-0 md:border-r border-white/10 pb-6 md:pb-0 md:pr-6">
<div class="w-20 h-20 rounded-2xl bg-violet-500/20 flex items-center justify-center text-violet-400 mb-4 shadow-lg shadow-violet-900/30 ring-1 ring-violet-500/30">
<svg class="w-10 h-10" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"></path></svg>
</div>
<div class="text-xs font-mono text-violet-300/80 uppercase tracking-widest">"Technical Whitepaper"</div>
</div>
<div class="md:col-span-9 space-y-6">
<div>
<h3 class="text-2xl font-bold text-white mb-2">"Unified Technical Architecture"</h3>
<p class="text-sm text-violet-200/70 font-medium">"Comparative Analysis of ID Authoring Tools"</p>
</div>
<div class="bg-slate-900/50 rounded-lg p-4 border border-white/5 relative group">
<div class="absolute top-0 left-0 w-1 h-full bg-violet-500 rounded-l-lg opacity-50 group-hover:opacity-100 transition-opacity"></div>
<p class="text-slate-300 text-sm leading-relaxed italic">
"This architectural analysis evaluates the limitations of existing market leaders (Twine, Storyline) against the project's pedagogical requirements.
It justifies the selection of an emerging, high-performance stack (Rust/Bevy/WASM) to solve the 'Creator Tooling Gap,' bridging the divide between narrative flexibility and complex state management."
</p>
</div>
<div class="flex justify-start pt-2">
<a href="https://docs.google.com/document/d/1EtW32Etg58ZEyc-8R_fQUwyum-7cEoC3qnCr7rn5wkQ/edit?usp=sharing" target="_blank" class="group inline-flex items-center gap-3 px-5 py-2.5 rounded-lg bg-violet-600 hover:bg-violet-500 text-white text-sm font-semibold transition-all shadow-lg shadow-violet-900/30 hover:shadow-violet-700/50">
<span>"View Analysis"</span>
<svg class="w-4 h-4 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"></path></svg>
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
<p> "In the planning phase of the Daydream Initiative, I conducted a rigorous comparative analysis of existing instructional design technologies, specifically <strong class='text-white'>Twine</strong> and <strong class='text-white'>Articulate Storyline 360</strong>.
My analysis revealed a critical dichotomy in the current market: Twine offers superior narrative branching capabilities but lacks robust visual or state management tools, while Storyline excels at visual interactivity but struggles with complex, non-linear variable tracking.
I determined that neither existing tool could support the 'Cognitive-Identity Loop' required for the project without significant compromises to the learner experience."
</p>
<p> "To resolve this 'Tooling Gap,' I evaluated and selected an emerging technology stack: the <span class='text-violet-300 font-medium'>Rust programming language</span> paired with the <span class='text-violet-300 font-medium'>Bevy Engine</span> and <span class='text-violet-300 font-medium'>WebAssembly (WASM)</span>.
While unconventional in traditional ID contexts, I identified that Rust's memory safety and Bevy's Entity Component System (ECS) offered the specific architectural capabilities needed to manage complex game states directly in the client's browser."
</p>
<p> "This process demonstrates a high-level competency in analyzing the characteristics of both established and emerging technologies.
By identifying the limitations of the 'Standard ID Stack' and successfully arguing for the adoption of a high-performance systems language, I moved beyond the role of a tool user to that of a <span class='text-violet-300 font-medium'>Systems Architect</span>."
</p>
</div>
</div>
</div>
}
}