use leptos::*;
use crate::components::glass_panel::GlassPanel;
#[component]
pub fn KeyConceptsChallenge() -> impl IntoView {
view! {
<div class="space-y-12 animate-fade-in relative">
<div class="border-l-4 border-indigo-500 pl-6 py-4 mb-8">
<h2 class="text-4xl font-extrabold text-white tracking-tight">"Theoretical Synthesis"</h2>
<div class="flex flex-wrap items-center gap-3 mt-3">
<span class="px-3 py-1 rounded-full bg-indigo-500/20 text-indigo-300 text-xs font-bold uppercase tracking-widest border border-indigo-500/30">"Applying ID Research & Theory"</span>
<span class="text-slate-400 text-sm flex items-center gap-2">
<span class="w-1.5 h-1.5 rounded-full bg-indigo-500"></span>
"Challenge: Explain Key Concepts & Principles"
</span>
</div>
</div>
<div class="relative z-10">
<GlassPanel class="bg-gradient-to-br from-slate-800/80 to-indigo-900/20 border-indigo-500/20">
<div class="grid grid-cols-1 md:grid-cols-12 gap-8">
<div class="md:col-span-3 flex flex-col items-center justify-center border-b md:border-b-0 md:border-r border-white/10 pb-6 md:pb-0 md:pr-6">
<div class="w-20 h-20 rounded-2xl bg-indigo-500/20 flex items-center justify-center text-indigo-400 mb-4 shadow-lg shadow-indigo-900/30 ring-1 ring-indigo-500/30">
<svg class="w-10 h-10" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"></path></svg>
</div>
<div class="text-xs font-mono text-indigo-300/80 uppercase tracking-widest">"Research Monograph"</div>
</div>
<div class="md:col-span-9 space-y-6">
<div>
<h3 class="text-2xl font-bold text-white mb-2">"The Professional Self in an Age of Measurement"</h3>
<p class="text-sm text-indigo-200/70 font-medium">"Navigating Authenticity, Learning, and Value"</p>
</div>
<div class="bg-slate-900/50 rounded-lg p-4 border border-white/5 relative group">
<div class="absolute top-0 left-0 w-1 h-full bg-indigo-500 rounded-l-lg opacity-50 group-hover:opacity-100 transition-opacity"></div>
<p class="text-slate-300 text-sm leading-relaxed italic">
"This monograph synthesizes Critical Theory, Andragogy, and Ethics to critique the 'Audit Culture' of modern ed-tech.
It argues that the commodification of learning metrics (ROI) undermines genuine capability.
In response, it proposes a 'Dialogic Model' of practice, grounded in the Capability Approach, where the goal of design is not merely knowledge transfer, but the expansion of human freedom."
</p>
</div>
<div class="flex justify-start pt-2">
<a href="https://docs.google.com/document/d/14K_2L_3M_5N_6O_8P_9Q_0R_1S_2T_3U/edit?usp=sharing" target="_blank" class="group inline-flex items-center gap-3 px-5 py-2.5 rounded-lg bg-indigo-600 hover:bg-indigo-500 text-white text-sm font-semibold transition-all shadow-lg shadow-indigo-900/30 hover:shadow-indigo-700/50">
<span>"Read Monograph"</span>
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
<p> "This artifact serves as a synthesis of foundational and advanced learning theories, interpreting them through the lens of 'Dialogic Ethics' to propose a new model of instructional practice.
I explain <strong class='text-white'>Andragogy</strong> (Malcolm Knowles) not merely as a set of techniques for adult learners, but as an ethical imperative rooted in the 'self-concept' of the learner as a non-dependent agent.
I link this theoretically to the <span class='text-indigo-300 font-medium'>Capability Approach</span> (Sen/Nussbaum), demonstrating that the goal of instructional design must be 'capability expansion'—increasing the learner's effective freedom to achieve the life they value—rather than simple behavioral modification."
</p>
<p> "Moving beyond individual psychology, I apply <span class='text-indigo-300 font-medium'>Social Constructivism</span> and <span class='text-indigo-300 font-medium'>Martin Buber’s Philosophy of Dialogue</span> to critique the traditional 'Expert' model.
I explain the distinction between the 'I-It' relationship (where the learner is an object to be filled) and the 'I-Thou' relationship (a reciprocal encounter).
I argue that effective instructional design must facilitate the latter, transforming the designer from a 'prescriptive' authority into a 'facilitator' of shared meaning."
</p>
<p> "Finally, I utilize the sociological principle of <strong class='text-white'>Campbell's Law</strong> to interpret modern evaluation crises.
By connecting this principle to the cognitive bias of 'confirmation bias,' I provide a rigorous theoretical explanation for why high-stakes testing (like ROI demands in L&D) often leads to the 'corruption of data.'
This synthesis demonstrates a mastery of the interdisciplinary concepts—spanning sociology, philosophy, and cognitive psychology—that underpin robust, ethical instructional design."
</p>
</div>
</div>
</div>
}
}