use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    *,
};

use crate::pages::daydream::Daydream;
use crate::pages::design::Design;
use crate::pages::evaluation::Evaluation;
use crate::pages::foundations::Foundations;
use crate::pages::home::Home;
use crate::pages::planning::Planning;
use crate::pages::systems_thinking::SystemsThinkingPage;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Layout>
                <Routes fallback=|| view! { "Page Not Found" }>
                    <Route path=path!("/") view=Home/>
                    <Route path=path!("/daydream") view=Daydream/>
                    <Route path=path!("/foundations") view=Foundations/>
                    <Route path=path!("/planning") view=Planning/>
                    <Route path=path!("/design") view=Design/>
                    <Route path=path!("/evaluation") view=Evaluation/>
                    <Route path=path!("/systems-thinking") view=SystemsThinkingPage/>
                </Routes>
            </Layout>
        </Router>
    }
}

#[component]
fn Layout(children: Children) -> impl IntoView {
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
                            <a href="/" class="text-xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-cyan-400 to-blue-500">
                                "LDT Portfolio"
                            </a>
                        </div>

                        <div class="hidden md:block">
                            <div class="ml-10 flex items-baseline space-x-4">
                                <a href="/" class="text-gray-300 hover:text-cyan-400 hover:bg-white/5 px-3 py-2 rounded-md text-sm font-medium transition-all duration-200">"Home"</a>
                                <a href="/daydream" class="text-gray-300 hover:text-purple-400 hover:bg-white/5 px-3 py-2 rounded-md text-sm font-medium transition-all duration-200">"Daydream Initiative"</a>
                                <a href="/foundations" class="text-gray-300 hover:text-indigo-400 hover:bg-white/5 px-3 py-2 rounded-md text-sm font-medium transition-all duration-200">"Foundations"</a>
                                <a href="/planning" class="text-gray-300 hover:text-rose-400 hover:bg-white/5 px-3 py-2 rounded-md text-sm font-medium transition-all duration-200">"Planning"</a>
                                <a href="/design" class="text-gray-300 hover:text-pink-400 hover:bg-white/5 px-3 py-2 rounded-md text-sm font-medium transition-all duration-200">"Design"</a>
                                <a href="/evaluation" class="text-gray-300 hover:text-teal-400 hover:bg-white/5 px-3 py-2 rounded-md text-sm font-medium transition-all duration-200">"Evaluation"</a>
                                <a href="/systems-thinking" class="text-gray-300 hover:text-cyan-400 hover:bg-white/5 px-3 py-2 rounded-md text-sm font-medium transition-all duration-200">"Systems Thinking"</a>
                            </div>
                        </div>
                    </div>
                </div>
            </nav>

            <div class="relative z-10 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
                {children()}
            </div>
        </div>
    }
}
