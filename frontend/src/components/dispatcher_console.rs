use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
#[allow(dead_code)]
enum ActivePanel {
    None,
    Yard,
    WeighStation,
    Comm,
}

#[component]
pub fn DispatcherConsole(children: Children) -> impl IntoView {
    let (active_panel, set_active_panel) = create_signal(ActivePanel::Yard);

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
                    <p class="text-steam-white">"Communication Uplink with Pete"</p>
                    // Add Comm content here
                </div>
            </main>

            // 4. THE INSPECTION PIT (Right/Bottom Panel)
            <aside class="grid-properties text-steam-white">
                <h3 class="text-metallic-gold text-lg mb-4 border-b border-old-gold pb-2">"CARGO MANIFEST"</h3>
                <div class="data-readout font-mono text-sm">
                    "Select a node to inspect payload."
                </div>
            </aside>
        </div>
    }
}
