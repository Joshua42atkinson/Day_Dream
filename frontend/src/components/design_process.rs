use crate::components::glass_panel::GlassPanel;
use leptos::prelude::*;

#[component]
pub fn DesignProcessChallenge() -> impl IntoView {
    view! {
        <div class="space-y-12 animate-fade-in relative">
            <div class="border-l-4 border-purple-500 pl-6 py-4 mb-8">
                <h2 class="text-4xl font-extrabold text-white tracking-tight">"Instructional Design Process"</h2>
                <div class="flex flex-wrap items-center gap-3 mt-3">
                    <span class="px-3 py-1 rounded-full bg-purple-500/20 text-purple-300 text-xs font-bold uppercase tracking-widest border border-purple-500/30">"Design & Development"</span>
                    <span class="text-slate-400 text-sm flex items-center gap-2">
                        <span class="w-1.5 h-1.5 rounded-full bg-purple-500"></span>
                        "Challenge: Select or Create a Process"
                    </span>
                </div>
            </div>

            <div class="relative z-10">
                <GlassPanel class="bg-gradient-to-br from-slate-800/80 to-purple-900/20 border-purple-500/20">
                    <div class="grid grid-cols-1 md:grid-cols-12 gap-8">
                        <div class="md:col-span-3 flex flex-col items-center justify-center border-b md:border-b-0 md:border-r border-white/10 pb-6 md:pb-0 md:pr-6">
                            <div class="w-20 h-20 rounded-2xl bg-purple-500/20 flex items-center justify-center text-purple-400 mb-4 shadow-lg shadow-purple-900/30 ring-1 ring-purple-500/30">
                                <svg class="w-10 h-10" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M19.428 15.428a2 2 0 00-1.022-.547l-2.384-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z"></path></svg>
                            </div>
                            <div class="text-xs font-mono text-purple-300/80 uppercase tracking-widest">"Strategic Blueprint"</div>
                        </div>
                        <div class="md:col-span-9 space-y-6">
                            <div>
                                <h3 class="text-2xl font-bold text-white mb-2">"Daydream Unified Specification"</h3>
                                <p class="text-sm text-purple-200/70 font-medium">"The Creator's Sandbox Architecture"</p>
                            </div>
                            <div class="bg-slate-900/50 rounded-lg p-4 border border-white/5 relative group">
                                <div class="absolute top-0 left-0 w-1 h-full bg-purple-500 rounded-l-lg opacity-50 group-hover:opacity-100 transition-opacity"></div>
                                <p class="text-slate-300 text-sm leading-relaxed italic">
                                    "This strategic blueprint synthesizes the narrative flexibility of Twine, the interactive power of Storyline, and the media richness of Genially into a unified 'Creator's Sandbox.'
                                    It defines the custom architectural process used to build the Daydream platform, moving beyond standard ID models to engineer a bespoke authoring workflow."
                                </p>
                            </div>
                            <div class="flex justify-start pt-2">
                                <a href="https://docs.google.com/document/d/1EtW32Etg58ZEyc-8R_fQUwyum-7cEoC3qnCr7rn5wkQ/edit?usp=sharing" target="_blank" class="group inline-flex items-center gap-3 px-5 py-2.5 rounded-lg bg-purple-600 hover:bg-purple-500 text-white text-sm font-semibold transition-all shadow-lg shadow-purple-900/30 hover:shadow-purple-700/50">
                                    <span>"View Full Specification"</span>
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
                        "For the Daydream Initiative, I determined that selecting a single, pre-existing instructional design (ID) process was insufficient to address the project"
                        {"'"}
                        "s unique nature: bridging the gap between narrative complexity and technical interactivity. Traditional linear processes (like standard ADDIE) often fail to account for the non-linear, branching nature of interactive storytelling, while game development processes often lack the pedagogical rigor required for education. Consequently, I "
                        <strong class="text-white">"created a novel instructional design process"</strong>
                        " defined as a "
                        {"'"}
                        "Synthesis of Proven Authoring Paradigms."
                        {"'"}
                    </p>
                    <p>
                        "I designed this process by isolating the "
                        {"'"}
                        "best-in-class"
                        {"'"}
                        " features of existing paradigms and synthesizing them into a coherent architectural specification. I selected "
                        <span class="text-cyan-300 font-medium">
                            "Twine"
                            {"'"}
                            "s node-based logic"
                        </span>
                        " to govern the narrative flow, allowing designers to visualize complex branching paths without getting lost in the weeds. I then integrated "
                        <span class="text-cyan-300 font-medium">
                            "Articulate Storyline"
                            {"'"}
                            "s "
                            {"'"}
                            "triggers and states"
                            {"'"}
                        </span>
                        " model to handle interaction design, enabling non-programmers to build sophisticated cause-and-effect mechanics."
                    </p>
                    <p>
                        "This creation of a bespoke process was driven by a specific design mandate: to empower the Instructional Designer to function as an architect rather than just a content writer. The specification details a modular "
                        <strong class="text-white">"Intelligent Tutoring System (ITS)"</strong>
                        " architecture—comprising Expert, Tutor, Student, and UI modules—that serves as the technical backbone for this new process."
                    </p>
                </div>
            </div>
        </div>
    }
}
