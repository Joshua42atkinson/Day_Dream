use leptos::prelude::*;

#[component]
pub fn PearlSetup() -> impl IntoView {
    let (subject, set_subject) = signal(String::new());
    let (vision, set_vision) = signal(String::new());

    let handle_submit = move |_| {
        // Here we would typically save the PEARL to state or backend
        // For now, we can just log it or advance to the Daydream slide.
        leptos::logging::log!("PEARL Locked: Subject='{}', Vision='{}'", subject.get(), vision.get());
        
        // Navigation could happen here:
        // use leptos_router::hooks::use_navigate;
        // let navigate = use_navigate();
        // navigate("/", Default::default());
    };

    view! {
        <div class="flex flex-col items-center justify-center min-h-[calc(100vh-4rem)] p-4">
            <div class="w-full max-w-2xl">
                // Title Area
                <div class="text-center mb-12">
                    <h1 class="text-4xl md:text-5xl font-light text-white tracking-widest mb-4">
                        "SESSION "
                        <span class="font-bold text-transparent bg-clip-text bg-gradient-to-r from-cyan-400 to-purple-500">
                            "ZERO"
                        </span>
                    </h1>
                    <p class="text-slate-400 font-light tracking-wide max-w-lg mx-auto">
                        "Define the PEARL (lens) through which the story will be generated."
                    </p>
                </div>

                // Glassmorphic Form Container
                <div class="relative bg-slate-900/40 backdrop-blur-xl border border-white/10 rounded-3xl p-8 md:p-12 shadow-2xl overflow-hidden">
                    // Decorative glow
                    <div class="absolute -top-24 -right-24 w-48 h-48 bg-cyan-500/20 rounded-full blur-3xl pointer-events-none"></div>
                    <div class="absolute -bottom-24 -left-24 w-48 h-48 bg-purple-500/20 rounded-full blur-3xl pointer-events-none"></div>

                    <div class="relative z-10 flex flex-col gap-8">
                        // Subject Input
                        <div class="flex flex-col gap-2">
                            <label class="text-cyan-400 text-sm font-semibold tracking-widest uppercase">
                                "1. Subject (VAAM Focus)"
                            </label>
                            <p class="text-slate-400 text-xs mb-2">
                                "The core concept you want to explore (e.g., 'Cognitive Bias', 'Resilience', 'Photosynthesis')."
                            </p>
                            <input
                                type="text"
                                class="bg-black/30 border border-white/10 rounded-xl px-4 py-3 text-white placeholder-slate-500 focus:outline-none focus:border-cyan-500/50 focus:ring-1 focus:ring-cyan-500/50 transition-all"
                                placeholder="Enter your subject..."
                                on:input=move |ev| set_subject.set(event_target_value(&ev))
                                prop:value=subject
                            />
                        </div>

                        // Vision Input
                        <div class="flex flex-col gap-2">
                            <label class="text-purple-400 text-sm font-semibold tracking-widest uppercase">
                                "2. Aesthetic Vision"
                            </label>
                            <p class="text-slate-400 text-xs mb-2">
                                "The narrative wrapper (e.g., 'Sci-Fi Survival', 'LitRPG Fantasy', 'Noir Detective')."
                            </p>
                            <input
                                type="text"
                                class="bg-black/30 border border-white/10 rounded-xl px-4 py-3 text-white placeholder-slate-500 focus:outline-none focus:border-purple-500/50 focus:ring-1 focus:ring-purple-500/50 transition-all"
                                placeholder="Enter your aesthetic vision..."
                                on:input=move |ev| set_vision.set(event_target_value(&ev))
                                prop:value=vision
                            />
                        </div>

                        // Submit Button
                        <button
                            on:click=handle_submit
                            class="mt-6 group relative w-full flex justify-center py-4 px-4 border border-transparent text-sm font-medium rounded-xl text-white bg-slate-800 hover:bg-slate-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-cyan-500 transition-all overflow-hidden"
                        >
                            <div class="absolute inset-0 w-full h-full bg-gradient-to-r from-cyan-600/20 to-purple-600/20 opacity-0 group-hover:opacity-100 transition-opacity"></div>
                            <span class="relative tracking-widest uppercase flex items-center gap-2">
                                "Initialize PEARL"
                                <svg class="w-4 h-4 group-hover:translate-x-1 transition-transform" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3" />
                                </svg>
                            </span>
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}
