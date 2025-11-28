use leptos::prelude::*;
use leptos_router::components::Outlet;

/// The Instructor's authoring environment ("The Train Yard")
/// IDE-style interface for creating curriculum and managing cognitive load
#[component]
pub fn TrainYardLayout() -> impl IntoView {
    view! {
        <div class="train-yard-layout h-screen bg-slate-900 text-white">
            // Industrial header with navigation
            <header class="border-b border-[#CFB991] bg-slate-950 px-6 py-4">
                <div class="flex items-center justify-between">
                    <div class="flex items-center gap-4">
                        <h1 class="text-2xl font-bold text-[#CFB991]">"THE TRAIN YARD"</h1>
                        <div class="text-xs text-slate-400 uppercase tracking-wider">
                            "Instructor Control Panel"
                        </div>
                    </div>

                    <nav class="flex gap-4">
                        <a href="/yard" class="px-4 py-2 rounded bg-slate-800 hover:bg-slate-700 transition">
                            "Graph Editor"
                        </a>
                        <a href="/yard/weigh-station" class="px-4 py-2 rounded bg-slate-800 hover:bg-slate-700 transition">
                            "Weigh Station"
                        </a>
                        <a href="/yard/library" class="px-4 py-2 rounded bg-slate-800 hover:bg-slate-700 transition">
                            "Knowledge Library"
                        </a>
                    </nav>
                </div>
            </header>

            // Main content area - child routes render here
            <main class="h-[calc(100vh-72px)] overflow-auto">
                <Outlet/>
            </main>
        </div>
    }
}
