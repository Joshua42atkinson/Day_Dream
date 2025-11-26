use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    *,
};

use crate::pages::daydream::Daydream;
use crate::pages::not_found::NotFound;
use crate::pages::research_dashboard::ResearchDashboard;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Layout/>
    }
}

#[component]
fn Layout() -> impl IntoView {
    let (_is_menu_open, _set_is_menu_open) = signal(false);

    view! {
        <div class="relative min-h-screen font-inter overflow-hidden selection:bg-cyan-500 selection:text-white">
            // Aurora Background
            <div class="fixed inset-0 z-0 bg-slate-900">
                <div class="absolute inset-0 bg-[radial-gradient(ellipse_at_top,_var(--tw-gradient-stops))] from-slate-900 via-[#0a0a0a] to-black"></div>
                <div class="absolute inset-0 bg-[url('/noise.svg')] opacity-20 mix-blend-soft-light"></div>
            </div>

            // Animated Blobs
            <div class="fixed inset-0 z-0 pointer-events-none">
                <div class="absolute top-0 left-1/4 w-96 h-96 bg-cyan-500/20 rounded-full mix-blend-screen filter blur-3xl opacity-30 animate-blob"></div>
                <div class="absolute top-0 right-1/4 w-96 h-96 bg-blue-600/20 rounded-full mix-blend-screen filter blur-3xl opacity-30 animate-blob animation-delay-2000"></div>
                <div class="absolute -bottom-32 left-1/3 w-96 h-96 bg-purple-600/20 rounded-full mix-blend-screen filter blur-3xl opacity-30 animate-blob animation-delay-4000"></div>
            </div>

            <nav class="sticky top-0 z-50 w-full border-b border-white/10 bg-slate-900/70 backdrop-blur-md">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="flex items-center justify-between h-16">
                        <div class="flex-shrink-0">
                            <a href="/" class="text-xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-purple-400 to-cyan-500">
                                "Daydream Engine"
                            </a>
                        </div>

                        <div class="hidden md:block">
                            <div class="ml-10 flex items-baseline space-x-4">
                                <a href="/" class="text-gray-300 hover:text-purple-400 hover:bg-white/5 px-3 py-2 rounded-md text-sm font-medium transition-all duration-200">"Sandbox"</a>
                                <a href="/research" class="text-gray-300 hover:text-green-400 hover:bg-white/5 px-3 py-2 rounded-md text-sm font-medium transition-all duration-200">"Research Logs"</a>
                            </div>
                        </div>
                    </div>
                </div>
            </nav>

            <main class="relative z-10">
                <Router>
                    <Routes fallback=|| view! { "Page Not Found" }>
                        <Route path=path!("/") view=Daydream/>
                        <Route path=path!("/research") view=ResearchDashboard/>
                        <Route path=path!("/*any") view=NotFound/>
                    </Routes>
                </Router>
            </main>
        </div>
    }
}
