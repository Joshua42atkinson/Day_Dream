use crate::components::artifact_card::ArtifactCard;
use crate::components::glass_panel::GlassPanel;
use crate::models::{Artifact, GameTurn, PlayerCommand};
use gloo_net::http::Request;
use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
pub fn Daydream() -> impl IntoView {
    // Data for the artifacts
    let artifacts = vec![
        Artifact {
            title: "Gap Analysis & ADDIE".to_string(),
            description: "Identifying the 'Creator Tooling Gap' in EdTech: Why designers are forced to choose between narrative flexibility and technical power.".to_string(),
            tags: vec!["Analysis".to_string(), "ADDIE".to_string()],
            link: Some("https://docs.google.com/document/d/1EtW32Etg58ZEyc-8R_fQUwyum-7cEoC3qnCr7rn5wkQ/edit?usp=sharing".to_string()), // Replace with your actual link
            link_text: "Read Analysis".to_string(),
            icon: "<svg fill='none' stroke='currentColor' viewBox='0 0 24 24'><path stroke-linecap='round' stroke-linejoin='round' stroke-width='2' d='M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z'></path></svg>".to_string(),
        },
        Artifact {
            title: "The VaaM Model".to_string(),
            description: "Vocabulary-as-a-Mechanic: A situated pedagogy moving beyond rote memorization to 'Implicit Assessment' inside the game loop.".to_string(),
            tags: vec!["Research".to_string(), "Pedagogy".to_string()],
            link: Some("https://docs.google.com/document/d/1Nlm2Q5MFzGaa3uL6Xry6gCMrtDjIKw11WIouYPBrIKY/edit?usp=sharing".to_string()),
            link_text: "View Research".to_string(),
            icon: "<svg fill='none' stroke='currentColor' viewBox='0 0 24 24'><path stroke-linecap='round' stroke-linejoin='round' stroke-width='2' d='M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253'></path></svg>".to_string(),
        },
        Artifact {
            title: "Mentor-in-the-Loop".to_string(),
            description: "Integrating Vygotsky's Sociocultural Theory with AI. How the 'Contemplative Guide' acts as a Socratic tutor.".to_string(),
            tags: vec!["AI Strategy".to_string(), "Vygotsky".to_string()],
            link: Some("#".to_string()),
            link_text: "Read Brief".to_string(),
            icon: "<svg fill='none' stroke='currentColor' viewBox='0 0 24 24'><path stroke-linecap='round' stroke-linejoin='round' stroke-width='2' d='M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z'></path></svg>".to_string(),
        },
    ];

    view! {
        <div class="max-w-6xl mx-auto space-y-12 animate-fade-in p-6">
            // Hero Section
            <div class="text-center space-y-6">
                <div class="inline-flex items-center px-4 py-2 rounded-full bg-purple-900/30 border border-purple-500/50 text-purple-300 text-sm font-bold uppercase tracking-widest">
                    <span class="w-2 h-2 bg-purple-400 rounded-full mr-3 animate-pulse"></span>
                    "Capstone Project"
                </div>
                <h1 class="text-5xl md:text-7xl font-black text-white tracking-tight">
                    "The Daydream" <span class="text-transparent bg-clip-text bg-gradient-to-r from-purple-400 to-pink-600">"Initiative"</span>
                </h1>
                <p class="text-xl text-slate-300 max-w-3xl mx-auto leading-relaxed">
                    "A privacy-first 'creator's sandbox' empowering instructional designers to build narrative-driven intelligent tutoring systems."
                </p>
            </div>

            // Video/Info Panel
            <GlassPanel>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-8 items-center">
                    <div class="space-y-4">
                        <h2 class="text-2xl font-bold text-white">"The Engine of Enjoyment"</h2>
                        <p class="text-slate-400">
                            "We solve the 'Edutainment Gap' by fusing the narrative structure of the Hero's Journey with the rigorous scaffolding of Cognitive Load Theory. It is not just a game; it is a psychometric engine."
                        </p>
                    </div>
                    <div class="relative group aspect-video rounded-lg bg-black/50 border border-white/10 flex items-center justify-center overflow-hidden">
                        <div class="absolute inset-0 bg-gradient-to-tr from-purple-600/20 to-cyan-600/20 group-hover:opacity-75 transition-opacity"></div>
                        <div class="text-center relative z-10">
                            <div class="w-16 h-16 rounded-full bg-white/10 flex items-center justify-center mx-auto mb-4 group-hover:scale-110 transition-transform duration-300 border border-white/20">
                                <span class="text-2xl">"▶"</span>
                            </div>
                            <span class="text-sm text-slate-400 uppercase tracking-widest">"Tech Demo Coming Soon"</span>
                        </div>
                    </div>
                </div>
            </GlassPanel>

            // Artifact Grid
            <div class="space-y-8">
                <div class="flex items-center gap-4">
                    <h2 class="text-3xl font-bold text-white">"Project Artifacts"</h2>
                    <div class="h-px flex-grow bg-white/10"></div>
                </div>
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    {artifacts.into_iter().map(|artifact| view! {
                        <ArtifactCard artifact=artifact />
                    }).collect_view()}
                </div>
            </div>

            // Game Terminal Section
            <div class="space-y-8">
                <div class="flex items-center gap-4">
                    <h2 class="text-3xl font-bold text-white">"Game Terminal"</h2>
                    <div class="h-px flex-grow bg-white/10"></div>
                </div>
                <GameTerminal />
            </div>
        </div>
    }
}

#[component]
fn GameTerminal() -> impl IntoView {
    let (history, set_history) = create_signal(vec![
        "Welcome to the Daydream Initiative terminal.".to_string(),
    ]);
    let (command, set_command) = create_signal(String::new());

    let send_command = move |_| {
        let cmd = command.get_untracked();
        if !cmd.is_empty() {
            set_history.update(|h| h.push(format!("> {}", cmd)));
            let cmd_clone = cmd.clone();
            spawn_local(async move {
                let command_payload = PlayerCommand {
                    command_text: cmd_clone,
                };
                let response = Request::post("/api/submit_command")
                    .json(&command_payload)
                    .expect("Failed to build request.")
                    .send()
                    .await;

                match response {
                    Ok(resp) => {
                        if resp.ok() {
                            let game_turn: GameTurn =
                                resp.json().await.expect("Failed to parse response.");
                            set_history.update(|h| h.push(game_turn.ai_narrative));
                            if let Some(msg) = game_turn.system_message {
                                set_history.update(|h| h.push(format!("[SYSTEM] {}", msg)));
                            }
                        } else {
                            set_history
                                .update(|h| h.push(format!("Error: {}", resp.status_text())));
                        }
                    }
                    Err(_) => {
                        set_history
                            .update(|h| h.push("Error: Could not reach backend.".to_string()));
                    }
                }
            });
            set_command.set(String::new());
        }
    };

    view! {
        <GlassPanel>
            <div class="flex flex-col h-96">
                <div class="flex-grow overflow-y-auto bg-black/50 p-4 rounded-t-lg font-mono text-sm text-green-400 space-y-2">
                    <For
                        each=move || history.get().into_iter().enumerate()
                        key=|(index, _)| *index
                        children=move |(_, line)| view! { <p>{line}</p> }
                    />
                </div>
                <div class="flex-shrink-0 p-4 bg-black/30 rounded-b-lg">
                    <div class="flex items-center gap-4">
                        <input
                            type="text"
                            class="flex-grow bg-transparent border-b border-purple-500/50 text-white focus:outline-none focus:border-purple-400"
                            placeholder="Type your command..."
                            on:input=move |ev| set_command.set(event_target_value(&ev))
                            prop:value=command
                        />
                        <button
                            class="px-4 py-2 bg-purple-600 hover:bg-purple-500 rounded-lg text-white font-bold"
                            on:click=send_command
                        >
                            "Send"
                        </button>
                    </div>
                </div>
            </div>
        </GlassPanel>
    }
}
