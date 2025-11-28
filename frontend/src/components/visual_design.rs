use crate::components::glass_panel::GlassPanel;
use leptos::prelude::*;

#[component]
pub fn VisualDesignChallenge() -> impl IntoView {
    view! {
        <div class="space-y-12 animate-fade-in relative">

            // --- Challenge Header ---
            <div class="border-l-4 border-indigo-400 pl-6 py-4 mb-8">
                <h2 class="text-4xl font-extrabold text-white tracking-tight">"The Aesthetic-Usability Effect"</h2>
                <div class="flex flex-wrap items-center gap-3 mt-3">
                    <span class="px-3 py-1 rounded-full bg-indigo-400/20 text-indigo-300 text-xs font-bold uppercase tracking-widest border border-indigo-400/30">
                        "Design & Development"
                    </span>
                    <span class="text-slate-400 text-sm flex items-center gap-2">
                        <span class="w-1.5 h-1.5 rounded-full bg-indigo-400"></span>
                        "Challenge: Use Visual Design Principles"
                    </span>
                </div>
            </div>

            // --- Artifact Showcase ---
            <div class="relative z-10">
                <GlassPanel class="bg-gradient-to-br from-slate-800/80 to-indigo-900/20 border-indigo-500/20">
                    <div class="grid grid-cols-1 md:grid-cols-12 gap-8">

                        // Visual Icon Column
                        <div class="md:col-span-3 flex flex-col items-center justify-center border-b md:border-b-0 md:border-r border-white/10 pb-6 md:pb-0 md:pr-6">
                            <div class="w-20 h-20 rounded-2xl bg-indigo-500/20 flex items-center justify-center text-indigo-400 mb-4 shadow-lg shadow-indigo-900/30 ring-1 ring-indigo-500/30">
                                // Icon: Eye / Palette / Design
                                <svg class="w-10 h-10" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17h.01"></path>
                                </svg>
                            </div>
                            <div class="text-xs font-mono text-indigo-300/80 uppercase tracking-widest">"UI System Architecture"</div>
                        </div>

                        // Content Column
                        <div class="md:col-span-9 space-y-6">
                            <div>
                                <h3 class="text-2xl font-bold text-white mb-2">"The 'Daydream' Design Language"</h3>
                                <p class="text-sm text-indigo-200/70 font-medium">"Glassmorphism & Tailwind CSS Implementation"</p>
                            </div>

                            // Executive Summary Box
                            <div class="bg-slate-900/50 rounded-lg p-4 border border-white/5 relative group">
                                <div class="absolute top-0 left-0 w-1 h-full bg-indigo-400 rounded-l-lg opacity-50 group-hover:opacity-100 transition-opacity"></div>
                                <p class="text-slate-300 text-sm leading-relaxed italic">
                                    "This artifact is the frontend design system of the portfolio itself. It utilizes a 'Glassmorphism' aesthetic implemented via Rust and Tailwind CSS to establish visual hierarchy. By applying the principles of Contrast, Repetition, Alignment, and Proximity (CRAP), the design manages the user's attention, transforming a dense technical portfolio into an accessible, navigable narrative."
                                </p>
                            </div>

                            // Action Button
                            <div class="flex justify-start pt-2">
                                <a
                                    href="https://github.com/Joshua42atkinson/Day_Dream/blob/main/frontend/src/components/glass_panel.rs"
                                    target="_blank"
                                    class="group inline-flex items-center gap-3 px-5 py-2.5 rounded-lg bg-indigo-600 hover:bg-indigo-500 text-white text-sm font-semibold transition-all shadow-lg shadow-indigo-900/30 hover:shadow-indigo-700/50"
                                >
                                    <span>"View UI Code"</span>
                                    <svg class="w-4 h-4 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"></path></svg>
                                </a>
                            </div>
                        </div>
                    </div>
                </GlassPanel>
            </div>

            // --- Connecting Line (Visual Flow) ---
            <div class="absolute left-8 top-[280px] bottom-[100px] w-0.5 bg-gradient-to-b from-indigo-500/50 via-blue-500/30 to-transparent hidden md:block z-0"></div>

            // --- Reflection Section ---
            <div class="relative z-10 pl-0 md:pl-16">
                <div class="flex items-center gap-4 mb-6">
                    <div class="w-10 h-10 rounded-full bg-slate-800 border border-cyan-500/30 flex items-center justify-center text-cyan-400 shadow-lg">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg>
                    </div>
                    <h3 class="text-2xl font-bold text-cyan-100">"Competency Reflection"</h3>
                </div>

                <div class="prose prose-invert prose-lg max-w-none text-slate-300 leading-relaxed space-y-6">
                    <p>
                        "In constructing this portfolio, I applied advanced visual design principles to solve a specific instructional problem: 'Cognitive Overload.' The complexity of the Daydream project (Rust code, Pedagogical Theory, Ethics) risked overwhelming the viewer. To mitigate this, I utilized the <strong class='text-white'>Aesthetic-Usability Effect</strong>, implementing a 'Glassmorphism' design system. By using semi-transparent layers and background blurs (<code>backdrop-blur-xl</code>), I created a visual hierarchy based on <strong class='text-white'>Depth</strong>, allowing the user to intuitively distinguish between content (foreground) and context (background) without explicit instruction."
                    </p>

                    <p>
                        "I rigorously applied the <span class='text-indigo-300 font-medium'>CRAP Principles</span> (Contrast, Repetition, Alignment, Proximity) through the utility-first framework of Tailwind CSS. <strong class='text-white'>Repetition</strong> is established through the consistent use of the 'Aurora' animation and color gradients, creating a cohesive brand identity. <strong class='text-white'>Contrast</strong> is engineered using high-fidelity colors (Cyan/Fuchsia) against a dark slate background to guide the eye to 'Call to Action' elements. This disciplined application of visual rules ensures that the interface is not merely decorative, but functional."
                    </p>

                    <p>
                        "Furthermore, I designed the visual experience to be 'Responsive and Adaptive.' Recognizing that learners (and reviewers) access content on varied devices, I utilized a mobile-first grid architecture (<code>grid-cols-1 md:grid-cols-12</code>). This ensures that the visual integrity—and thus the instructional efficacy—of the portfolio remains intact across all viewports. This work demonstrates that visual design is not an afterthought but a core pedagogical competency, essential for reducing friction and maintaining engagement in complex digital environments."
                    </p>
                </div>
            </div>
        </div>
    }
}
