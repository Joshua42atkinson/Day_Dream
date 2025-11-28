use crate::components::glass_panel::GlassPanel;
use leptos::prelude::*;

#[component]
pub fn FormativeEvalChallenge() -> impl IntoView {
    view! {
        <div class="space-y-12 animate-fade-in relative">
            <div class="border-l-4 border-teal-500 pl-6 py-4 mb-8">
                <h2 class="text-4xl font-extrabold text-white tracking-tight">"The Ethical Self-Audit"</h2>
                <div class="flex flex-wrap items-center gap-3 mt-3">
                    <span class="px-3 py-1 rounded-full bg-teal-500/20 text-teal-300 text-xs font-bold uppercase tracking-widest border border-teal-500/30">"Evaluation & Implementation"</span>
                    <span class="text-slate-400 text-sm flex items-center gap-2">
                        <span class="w-1.5 h-1.5 rounded-full bg-teal-500"></span>
                        "Challenge: Implement Formative Evaluation Plans"
                    </span>
                </div>
            </div>
            <div class="relative z-10">
                <GlassPanel class="bg-gradient-to-br from-slate-800/80 to-teal-900/20 border-teal-500/20">
                    <div class="grid grid-cols-1 md:grid-cols-12 gap-8">
                        <div class="md:col-span-3 flex flex-col items-center justify-center border-b md:border-b-0 md:border-r border-white/10 pb-6 md:pb-0 md:pr-6">
                            <div class="w-20 h-20 rounded-2xl bg-teal-500/20 flex items-center justify-center text-teal-400 mb-4 shadow-lg shadow-teal-900/30 ring-1 ring-teal-500/30">
                                <svg class="w-10 h-10" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M3 6l3 1m0 0l-3 9a5.002 5.002 0 006.001 0M6 7l3 9M6 7l6-2m6 2l3-1m-3 1l-3 9a5.002 5.002 0 006.001 0M18 7l3 9m-3-9l-6-2m0-2v2m0 16V5m0 16H9m3 0h3"></path></svg>
                            </div>
                            <div class="text-xs font-mono text-teal-300/80 uppercase tracking-widest">"Ethical Monograph"</div>
                        </div>
                        <div class="md:col-span-9 space-y-6">
                            <div>
                                <h3 class="text-2xl font-bold text-white mb-2">"The Professional Self in an Age of Measurement"</h3>
                                <p class="text-sm text-teal-200/70 font-medium">"Navigating Authenticity, Learning, and Value"</p>
                            </div>
                            <div class="bg-slate-900/50 rounded-lg p-4 border border-white/5 relative group">
                                <div class="absolute top-0 left-0 w-1 h-full bg-teal-500 rounded-l-lg opacity-50 group-hover:opacity-100 transition-opacity"></div>
                                <p class="text-slate-300 text-sm leading-relaxed italic">
                                    "This critical analysis reframes evaluation from a tool of 'budget defense' (ROI) to a practice of 'Developmental Learning.'
                                    It argues that true formative evaluation requires a 'Self-Audit' of the designer's own ethics, moving beyond the 'Audit Culture' of vanity metrics to a 'Dialogic Model' that expands human capability and freedom."
                                </p>
                            </div>
                            <div class="flex justify-start pt-2">
                                <a href="https://docs.google.com/document/d/14K_2L_3M_5N_6O_8P_9Q_0R_1S_2T_3U/edit?usp=sharing" target="_blank" class="group inline-flex items-center gap-3 px-5 py-2.5 rounded-lg bg-teal-600 hover:bg-teal-500 text-white text-sm font-semibold transition-all shadow-lg shadow-teal-900/30 hover:shadow-teal-700/50">
                                    <span>"Read Evaluation Framework"</span>
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
                    <p>
                        "To reclaim the integrity of evaluation, I reframed the practice as a metacognitive discipline guided by "
                        <strong class="text-white">"Phronesis"</strong>
                        " (practical wisdom). Instead of merely measuring learner 'outputs,' I implemented a 'Self-Audit of Professional Identity.' This process requires the designer—and by extension, the learner—to critically examine the gap between their 'marketable brand' (external image) and their 'conscience of craft' (internal values). This shifts the focus of evaluation from 'proving value' to 'improving capability,' aligning with the "
                        <span class="text-teal-300 font-medium">"'Capability Approach'"</span>
                        " of Amartya Sen."
                    </p>
                    <p>
                        "Consequently, I designed the formative evaluation plan not as a series of quizzes, but as a 'Dialogic Encounter' that fosters "
                        <span class="text-teal-300 font-medium">"'Developmental Evaluation'"</span>
                        ". By embedding 'Humble Inquiry' and 'Appreciative Inquiry' into the design loop, the system acts as a mirror for metacognition. This ensures that evaluation is no longer a punitive judgment of the learner"
                        {"'"}
                        "s performance, but a supportive structure for 'conscious self-authorship,' proving that the most rigorous metric is not financial return, but the expansion of human freedom and autonomy."
                    </p>
                </div>
            </div>
        </div>
    }
}
