use crate::components::boilermaker::ChamferedPanel;
use crate::components::loading_spinner::LoadingSpinner;
use crate::game::engine::GameEngine;
use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
pub fn AskPete() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-coal-dark text-steam-white font-sans selection:bg-purdue-gold selection:text-black">
            // Header Section - Improved Scaling
            <div class="w-full bg-iron-gray border-b-2 border-aged-gold/40 py-6 px-4">
                <div class="max-w-7xl mx-auto">
                    <div class="flex items-center gap-6 mb-6">
                        // Pete Logo
                        <div class="w-16 h-16 bg-gradient-to-br from-purdue-gold to-aged-gold text-black chamfered-corners flex items-center justify-center font-black text-3xl border-2 border-dust-gold shadow-lg shadow-purdue-gold/30">
                            "P"
                        </div>

                        // Title Section
                        <div class="flex-1">
                            <h1 class="text-4xl md:text-5xl font-black text-metallic-gold tracking-wide uppercase mb-1">
                                "Ask Pete"
                            </h1>
                            <p class="text-sm text-purdue-gold/70 uppercase tracking-[0.3em] font-mono">
                                "Purdue Expert Tuition Engine"
                            </p>
                        </div>
                    </div>

                    // Welcome Panel - Better Spacing
                    <ChamferedPanel class="bg-industrial-surface border-2 border-purdue-gold/30 p-8 max-w-4xl">
                        <h2 class="text-3xl font-black text-purdue-gold uppercase tracking-wider mb-6 border-b border-purdue-gold/20 pb-3">
                            "Welcome to the Forge"
                        </h2>

                        <div class="space-y-6 text-steam-white">
                            <p class="text-xl leading-relaxed">
                                "I am "
                                <span class="font-black text-purdue-prime">"Pete"</span>
                                ", your "
                                <span class="font-bold text-purdue-gold">"Socratic Conductor"</span>
                                ". My job is not to give you answers, but to help you "
                                <span class="font-bold text-purdue-gold">"construct"</span>
                                " them."
                            </p>

                            <div class="h-px w-full bg-gradient-to-r from-transparent via-purdue-gold/40 to-transparent"></div>

                            <p class="text-base text-purdue-dust font-mono leading-relaxed">
                                "Below is the active Knowledge Graph, rendered as an interactive exploration. Each choice you make forges a new pathway through the material."
                            </p>

                            <p class="text-lg text-purdue-gold font-bold uppercase tracking-wider flex items-center gap-2">
                                <span class="text-2xl">"→"</span>
                                "Let's begin."
                            </p>
                        </div>
                    </ChamferedPanel>
                </div>
            </div>

            // Main Content Area
            <div class="max-w-7xl mx-auto px-4 py-8">
                <PeteTerminal />
            </div>
        </div>
    }
}

#[component]
fn PeteTerminal() -> impl IntoView {
    // State
    let (engine, set_engine) = signal(None::<GameEngine>);
    let (error_msg, set_error_msg) = signal(None::<String>);

    // Fetch the graph on load
    Effect::new(move |_| {
        spawn_local(async move {
            match crate::api::get_graph().await {
                Ok(graph) => {
                    // Initialize Engine with fetched graph
                    set_engine.set(Some(GameEngine::new(graph)));
                }
                Err(e) => set_error_msg.set(Some(format!("Could not load Pete's brain: {}", e))),
            }
        });
    });

    let handle_choice = move |target_id: String| {
        set_engine.update(|e| {
            if let Some(eng) = e {
                eng.make_choice(target_id);
            }
        });
    };

    let restart = move |_| {
        spawn_local(async move {
            if let Ok(graph) = crate::api::get_graph().await {
                set_engine.set(Some(GameEngine::new(graph)));
            }
        });
    };

    view! {
        <ChamferedPanel class="bg-industrial-surface border-2 border-purdue-gold/30">
            <div class="min-h-[600px] flex flex-col relative p-8">
                // Background Watermark
                <div class="absolute inset-0 flex items-center justify-center opacity-5 pointer-events-none">
                    <h1 class="text-9xl font-black text-white">"PETE"</h1>
                </div>

                <Show when=move || error_msg.get().is_some()>
                    <div class="p-8 text-signal-red bg-signal-red/10 border-2 border-signal-red chamfered-corners text-lg">
                        {move || error_msg.get()}
                    </div>
                </Show>

                <Show
                    when=move || engine.get().is_some()
                    fallback=|| view! {
                        <div class="flex-grow flex items-center justify-center text-old-gold gap-3">
                            <LoadingSpinner message="Stoking the Engine...".to_string() size="lg".to_string() />
                        </div>
                    }
                >
                    {move || {
                        let current_engine = engine.get().unwrap();
                        let current_node = current_engine.get_current_node();
                        let options = current_engine.get_options();

                        view! {
                            <div class="flex-grow flex flex-col justify-between z-10">
                                // Content Area - Better Spacing
                                <div class="space-y-8 animate-fade-in">
                                    <div class="flex items-center gap-3 text-purdue-gold/70 text-xs uppercase tracking-[0.3em] font-bold font-mono">
                                        <span class="w-3 h-3 bg-purdue-gold chamfered-corners animate-pulse-gold"></span>
                                        "Current Node"
                                    </div>

                                    <h2 class="text-5xl md:text-6xl font-black text-steam-white leading-tight">
                                        {current_node.map(|n| n.title.clone()).unwrap_or("End of Path".to_string())}
                                    </h2>

                                    <div class="text-xl text-purdue-dust leading-relaxed max-w-4xl">
                                        {current_node.map(|n| n.content.clone()).unwrap_or("The simulation has ended.".to_string())}
                                    </div>
                                </div>

                                // Choices Area - Larger Buttons
                                <div class="mt-16 space-y-6">
                                    <div class="h-px w-full bg-gradient-to-r from-transparent via-purdue-gold/40 to-transparent"></div>
                                    <p class="text-center text-base text-purdue-gold/60 uppercase font-mono tracking-wider">"Available Actions"</p>

                                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                                        {
                                            let options_for_list = options.clone();
                                            view! {
                                                <For
                                                    each=move || options_for_list.clone()
                                                    key=|opt| opt.0.clone()
                                                    children=move |(id, title)| {
                                                        let target = id.clone();
                                                        view! {
                                                            <button
                                                                class="group relative w-full p-6 text-left transition-all duration-200 hover:scale-[1.02] active:scale-[0.98]"
                                                                on:click=move |_| handle_choice(target.clone())
                                                            >
                                                                <div class="absolute inset-0 bg-iron-gray/50 chamfered-corners border-2 border-purdue-gold/20 group-hover:bg-purdue-gold/10 group-hover:border-purdue-gold transition-all shadow-lg group-hover:shadow-purdue-gold/30 pointer-events-none"></div>
                                                                <div class="relative flex items-center justify-between">
                                                                    <span class="text-lg font-bold text-steam-white group-hover:text-purdue-prime transition-colors">
                                                                        {title}
                                                                    </span>
                                                                    <span class="text-2xl text-purdue-gold opacity-0 group-hover:opacity-100 transition-all transform group-hover:translate-x-1">
                                                                        "→"
                                                                    </span>
                                                                </div>
                                                            </button>
                                                        }
                                                    }
                                                />
                                            }
                                        }

                                        <Show when=move || options.is_empty()>
                                            <button
                                                class="mechanical-button-primary w-full text-xl py-6"
                                                on:click=restart
                                            >
                                                "↺ Restart Simulation"
                                            </button>
                                        </Show>
                                    </div>
                                </div>
                            </div>
                        }
                    }}
                </Show>
            </div>
        </ChamferedPanel>
    }
}
