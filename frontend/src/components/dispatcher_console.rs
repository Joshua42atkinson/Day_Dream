use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq)]
#[allow(dead_code)]
enum ActivePanel {
    None,
    Yard,
    WeighStation,
    Comm,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct PhysicsState {
    pub mass: f32,
    pub power: f32,
    pub velocity: f32,
    pub miles: f32,
}

#[component]
pub fn DispatcherConsole(children: Children) -> impl IntoView {
    let (active_panel, set_active_panel) = signal(ActivePanel::Yard);
    let (physics_state, set_physics_state) = signal(PhysicsState::default());
    let (is_active, set_is_active) = signal(true);

    // Poll simulation state
    Effect::new(move |_| {
        spawn_local(async move {
            while is_active.get() {
                if let Ok(response) =
                    gloo_net::http::Request::get("http://localhost:3000/api/simulation/state")
                        .send()
                        .await
                {
                    if let Ok(state) = response.json::<PhysicsState>().await {
                        set_physics_state.set(state);
                    }
                }
                gloo_timers::future::sleep(std::time::Duration::from_millis(100)).await;
            }
        });

        on_cleanup(move || {
            set_is_active.set(false);
        });
    });

    view! {
        <div class="dispatcher-grid">
            // 1. THE SIGNAL GANTRY (Top Header)
            <header class="grid-header">
                <div class="logo text-metallic-gold text-xl">"ASK PETE: STATION MASTER"</div>
                <div class="flex-grow"></div>
                <div class="status-lights flex gap-4">
                    <span class="text-gauge-green font-mono">"SYSTEM: ONLINE"</span>
                </div>
            </header>

            // 2. THE SWITCH TRACK (Left Sidebar)
            <nav class="grid-sidebar p-2 gap-4">
                <button
                    class=move || if active_panel.get() == ActivePanel::Yard { "mechanical-button-primary w-full text-xs" } else { "mechanical-button-secondary w-full text-xs" }
                    on:click=move |_| set_active_panel.set(ActivePanel::Yard)
                >
                    "YARD"
                </button>
                <button
                    class=move || if active_panel.get() == ActivePanel::WeighStation { "mechanical-button-primary w-full text-xs" } else { "mechanical-button-secondary w-full text-xs" }
                    on:click=move |_| set_active_panel.set(ActivePanel::WeighStation)
                >
                    "WEIGH"
                </button>
                <button
                    class=move || if active_panel.get() == ActivePanel::Comm { "mechanical-button-primary w-full text-xs" } else { "mechanical-button-secondary w-full text-xs" }
                    on:click=move |_| set_active_panel.set(ActivePanel::Comm)
                >
                    "COMM"
                </button>
            </nav>

            // 3. THE MAIN LINE (Main Content Area)
            <main class="grid-main relative">
                <div class="viewport-container h-full w-full overflow-auto">
                    {children()}
                </div>

                // Floating Glass Panels
                <div
                    class="glass-panel absolute top-4 left-4 bottom-4 w-96 transition-transform duration-300 transform z-10 p-6"
                    class:translate-x-0=move || active_panel.get() == ActivePanel::WeighStation
                    class:-translate-x-full=move || active_panel.get() != ActivePanel::WeighStation
                >
                    <h2 class="text-metallic-gold text-2xl mb-4">"WEIGH STATION"</h2>
                    <p class="text-steam-white">"Vocabulary and Concept Weight Analysis"</p>
                    // Add Weigh Station content here
                </div>

                <div
                    class="glass-panel absolute top-4 left-4 bottom-4 w-96 transition-transform duration-300 transform z-10 p-6"
                    class:translate-x-0=move || active_panel.get() == ActivePanel::Comm
                    class:-translate-x-full=move || active_panel.get() != ActivePanel::Comm
                >
                    <h2 class="text-metallic-gold text-2xl mb-4">"MENTOR COMM"</h2>
                    <p class="text-steam-white mb-4">"Communication Uplink with Pete"</p>
                    <crate::components::ai_mirror_chat::AiMirrorChat />
                </div>
            </main>

            // 4. THE INSPECTION PIT (Right/Bottom Panel)
            <aside class="grid-properties text-steam-white p-4 overflow-hidden">
                <h3 class="text-metallic-gold text-lg mb-4 border-b border-old-gold pb-2">"TELEMETRY"</h3>
                <div class="grid grid-cols-2 gap-4 font-mono text-sm">
                    <div class="flex flex-col">
                        <span class="text-purdue-gold/50 text-xs">"VELOCITY"</span>
                        <span class="text-xl text-gauge-green">{move || format!("{:.2} m/s", physics_state.get().velocity)}</span>
                    </div>
                    <div class="flex flex-col">
                        <span class="text-purdue-gold/50 text-xs">"DISTANCE"</span>
                        <span class="text-xl text-steam-white">{move || format!("{:.2} mi", physics_state.get().miles)}</span>
                    </div>
                    <div class="flex flex-col">
                        <span class="text-purdue-gold/50 text-xs">"LOAD (MASS)"</span>
                        <span class="text-xl text-signal-red">{move || format!("{:.2} kg", physics_state.get().mass)}</span>
                    </div>
                    <div class="flex flex-col">
                        <span class="text-purdue-gold/50 text-xs">"WILL (POWER)"</span>
                        <span class="text-xl text-purdue-prime">{move || format!("{:.2} kW", physics_state.get().power)}</span>
                    </div>
                </div>
            </aside>
        </div>
    }
}
