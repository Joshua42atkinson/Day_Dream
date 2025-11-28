use crate::components::glass_panel::GlassPanel;
use leptos::prelude::*;
#[component]
pub fn TargetPopulationChallenge() -> impl IntoView {
    view! {
    <div class="space-y-12 animate-fade-in relative">
    <div class="border-l-4 border-rose-500 pl-6 py-4 mb-8">
    <h2 class="text-4xl font-extrabold text-white tracking-tight">"Learner Analysis"</h2>
    <div class="flex flex-wrap items-center gap-3 mt-3">
    <span class="px-3 py-1 rounded-full bg-rose-500/20 text-rose-300 text-xs font-bold uppercase tracking-widest border border-rose-500/30">"Planning & Analysis"</span>
    <span class="text-slate-400 text-sm flex items-center gap-2">
    <span class="w-1.5 h-1.5 rounded-full bg-rose-500"></span>
    "Challenge: Determine Characteristics of Target Population"
    </span>
    </div>
    </div>
    <div class="relative z-10">
    <GlassPanel class="bg-gradient-to-br from-slate-800/80 to-rose-900/20 border-rose-500/20">
    <div class="grid grid-cols-1 md:grid-cols-12 gap-8">
    <div class="md:col-span-3 flex flex-col items-center justify-center border-b md:border-b-0 md:border-r border-white/10 pb-6 md:pb-0 md:pr-6">
    <div class="w-20 h-20 rounded-2xl bg-rose-500/20 flex items-center justify-center text-rose-400 mb-4 shadow-lg shadow-rose-900/30 ring-1 ring-rose-500/30">
    <svg class="w-10 h-10" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"></path></svg>
    </div>
    <div class="text-xs font-mono text-rose-300/80 uppercase tracking-widest">"Design Document"</div>
    </div>
    <div class="md:col-span-9 space-y-6">
    <div>
    <h3 class="text-2xl font-bold text-white mb-2">"AI as a Mirror: Learner Analysis"</h3>
    <p class="text-sm text-rose-200/70 font-medium">"Psychographic Profile of the 'Digital Native'"</p>
    </div>
    <div class="bg-slate-900/50 rounded-lg p-4 border border-white/5 relative group">
    <div class="absolute top-0 left-0 w-1 h-full bg-rose-500 rounded-l-lg opacity-50 group-hover:opacity-100 transition-opacity"></div>
    <p class="text-slate-300 text-sm leading-relaxed italic">
"This design document details the psychographic and cognitive profile of the target population: Grades 8-12 students.
It identifies a critical tension—learners are 'digital natives' comfortable with technology but lack the 'affective' skills for deep self-reflection.
The analysis directly informed the decision to use AI not as a tutor, but as a private, non-judgmental mirror."
    </p>
    </div>
    <div class="flex justify-start pt-2">
    <a href="https://docs.google.com/document/d/155-1-3w5_16066-14_555-333_22-11/edit" target="_blank" class="group inline-flex items-center gap-3 px-5 py-2.5 rounded-lg bg-rose-600 hover:bg-rose-500 text-white text-sm font-semibold transition-all shadow-lg shadow-rose-900/30 hover:shadow-rose-700/50">
    <span>"View Learner Analysis"</span>
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
    <p> "In designing the 'AI as a Mirror' module, I conducted a deep learner analysis to determine the specific cognitive and affective characteristics of the target population: students in Grades 8-12.
This demographic is characterized by 'increasing cognitive abilities for abstract thought' yet often lacks the 'structured approach' required to apply those abilities to metacognition.
I identified a critical tension in this group: they are <strong class='text-white'>'digital natives'</strong> comfortable with technology, but their usage is largely passive.
They possess the technical skill to use a chatbot but lack the pedagogical skill to use it for self-discovery."
    </p>
    <p> "This analysis directly impacted my design choices, specifically the decision to frame the instruction around <span class='text-rose-300 font-medium'>'privacy' and 'autonomy.'</span> The analysis revealed that while these learners are 'curious about AI,' they are resistant to 'boring' self-reflection tasks that feel like schoolwork.
To bridge this gap, I designed the learning environment to be asynchronous and private—a 'Performance Context' that simulates their personal life rather than a classroom.
This ensures that the tool feels like a personal utility rather than an assignment."
    </p>
    <p> "Furthermore, recognizing their wide range of 'emotional articulation skills,' I determined that the instruction could not simply ask them to 'reflect.'
    Instead, it required concrete scaffolding. This led to the design of the <strong class='text-white'>'Guided Reflection Worksheet'</strong> with specific sentence stems and prompts, ensuring that students with lower verbal proficiency could still engage in high-level conceptual processing."
    </p>
    </div>
    </div>
    </div>
    }
}
