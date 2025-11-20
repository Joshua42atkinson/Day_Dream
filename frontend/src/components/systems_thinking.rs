use leptos::*;
use crate::components::glass_panel::GlassPanel;
#[component]
pub fn SystemsThinkingChallenge() -> impl IntoView {
view! {
<div class="space-y-12 animate-fade-in relative">
<div class="border-l-4 border-blue-500 pl-6 py-4 mb-8">
<h2 class="text-4xl font-extrabold text-white tracking-tight">"Systems Thinking in ID"</h2>
<div class="flex flex-wrap items-center gap-3 mt-3">
<span class="px-3 py-1 rounded-full bg-blue-500/20 text-blue-300 text-xs font-bold uppercase tracking-widest border border-blue-500/30">"Applying ID Research & Theory"</span>
<span class="text-slate-400 text-sm flex items-center gap-2">
<span class="w-1.5 h-1.5 rounded-full bg-blue-500"></span>
"Challenge: Apply Systems Thinking"
</span>
</div>
</div>
<div class="relative z-10">
<GlassPanel class="bg-gradient-to-br from-slate-800/80 to-blue-900/20 border-blue-500/20">
<div class="grid grid-cols-1 md:grid-cols-12 gap-8">
<div class="md:col-span-3 flex flex-col items-center justify-center border-b md:border-b-0 md:border-r border-white/10 pb-6 md:pb-0 md:pr-6">
<div class="w-20 h-20 rounded-2xl bg-blue-500/20 flex items-center justify-center text-blue-400 mb-4 shadow-lg shadow-blue-900/30 ring-1 ring-blue-500/30">
<svg class="w-10 h-10" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M19.428 15.428a2 2 0 00-1.022-.547l-2.384-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z"></path></svg>
</div>
<div class="text-xs font-mono text-blue-300/80 uppercase tracking-widest">"Strategy Brief"</div>
</div>
<div class="md:col-span-9 space-y-6">
<div>
<h3 class="text-2xl font-bold text-white mb-2">"Mentor-in-the-Loop Strategy"</h3>
<p class="text-sm text-blue-200/70 font-medium">"Unified Pedagogical Framework"</p>
</div>
<div class="bg-slate-900/50 rounded-lg p-4 border border-white/5 relative group">
<div class="absolute top-0 left-0 w-1 h-full bg-blue-500 rounded-l-lg opacity-50 group-hover:opacity-100 transition-opacity"></div>
<p class="text-slate-300 text-sm leading-relaxed italic">
"This strategy brief resolves the tension between Cognitive Load Theory (CLT) and Vygotsky's Sociocultural Theory by identifying 'Psychological Safety' as the system's hinge.
It maps a sequential flow of data and psychological states—from private AI reflection to social human mentorship—modeling the learning environment as a complex, interdependent ecosystem."
</p>
</div>
<div class="flex justify-start pt-2">
<a href="#" class="group inline-flex items-center gap-3 px-5 py-2.5 rounded-lg bg-blue-600 hover:bg-blue-500 text-white text-sm font-semibold transition-all shadow-lg shadow-blue-900/30 hover:shadow-blue-700/50">
<span>"View Framework"</span>
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
<p> "This artifact demonstrates the application of <strong class='text-white'>Systems Thinking</strong> to resolve a fundamental tension between two opposing pedagogical theories: Cognitive Load Theory (CLT) and Vygotsky’s Sociocultural Theory.
In isolation, CLT suggests minimizing social interaction to reduce 'extraneous load,' while Vygotsky argues that learning requires social interaction (the 'More Knowledgeable Other').
A linear design approach would view these as contradictory; however, using a systems approach, I identified <span class='text-blue-300 font-medium'>Psychological Safety (Edmondson)</span> as the 'hinge' variable that connects them into a unified whole."
</p>
<p> "I mapped this system as a sequential flow of data and psychological states.
The system begins with the <span class='text-blue-300 font-medium'>'AI as a Mirror' (Phase 1)</span>, which functions as a 'Safety Generator' by using the privacy of the AI to reduce extraneous load and build confidence.
Only once this system state is achieved does the learner opt-in to the <span class='text-blue-300 font-medium'>'Mentor-in-the-Loop' (Phase 2)</span>, activating the Vygotskian social scaffold."
</p>
<p> "By modeling the learning environment as a complex system of feedback loops—using <strong class='text-white'>Hattie & Timperley’s</strong> model of 'Feed-Up, Feed-Back, and Feed-Forward'—I defined the specific data packets (e.g., reflection_summary.json) that must flow between the Learner, the AI, and the Mentor.
This proves that effective instructional design requires looking beyond individual learning activities to engineer the holistic ecosystem in which those activities reside."
</p>
</div>
</div>
</div>
}
}