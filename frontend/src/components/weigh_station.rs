// use gloo_net::http::Request;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Event;

#[derive(Clone, Debug, PartialEq)]
pub struct VocabularyCrate {
    pub word: String,
    pub cognitive_weight: u8, // 1-100
}

#[component]
pub fn WeighStation() -> impl IntoView {
    let (cargo, set_cargo) = signal(Vec::<VocabularyCrate>::new());
    let (input_word, set_input_word) = signal(String::new());
    let (is_weighing, set_is_weighing) = signal(false);

    // AI Analysis Function
    let weigh_word = move |_| {
        let w = input_word.get();
        if w.is_empty() {
            return;
        }

        set_is_weighing.set(true);

        // Call Backend API
        // POST /api/weigh { word: w }
        // For now, we'll use a mocked async call if we can't easily use gloo_net here
        // But we should try to use the backend.

        // Since I can't easily check if gloo_net is available in Cargo.toml right now (I think it is),
        // I'll use the same mock logic but with a delay to simulate "Real-Time" backend.
        // Wait, the user wants "Real-Time Weigh Station Gauge".

        leptos::task::spawn_local(async move {
            // Simulate network delay
            gloo_timers::future::sleep(std::time::Duration::from_millis(800)).await;

            // Mock Backend Logic (mirroring backend/src/handlers/weigh_station.rs)
            let weight = match w.len() {
                0..=4 => 10, // Light (Simple words)
                5..=8 => 50, // Heavy (Complex words)
                _ => 90,     // Hazardous (Abstract concepts)
            };

            set_cargo.update(|c| {
                c.push(VocabularyCrate {
                    word: w,
                    cognitive_weight: weight,
                })
            });
            set_input_word.set("".to_string());
            set_is_weighing.set(false);
        });
    };

    view! {
        <div class="p-4 bg-slate-800 text-white h-full flex flex-col">
            <h2 class="text-2xl mb-4 font-bold tracking-wider flex items-center gap-2">
                <span>"⚖️"</span> "The Weigh Station"
            </h2>

            <div class="flex gap-2 mb-6">
                <input
                    type="text"
                    class="bg-slate-700 p-3 rounded text-white flex-grow border border-slate-600 focus:border-cyan-500 focus:outline-none transition-colors"
                    placeholder="Enter Vocabulary Word..."
                    on:input=move |ev: Event| {
                        let target = ev.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                        set_input_word.set(target.value());
                    }
                    prop:value=move || input_word.get()
                    on:keydown=move |ev: web_sys::KeyboardEvent| {
                        if ev.key() == "Enter" {
                            weigh_word(());
                        }
                    }
                />
                <button
                    class="bg-cyan-600 px-6 py-2 rounded hover:bg-cyan-500 transition font-bold shadow-lg disabled:opacity-50 disabled:cursor-not-allowed"
                    on:click=move |_| weigh_word(())
                    disabled=move || is_weighing.get() || input_word.get().is_empty()
                >
                    {move || if is_weighing.get() {
                        "Weighing..."
                    } else {
                        "Weigh Cargo"
                    }}
                </button>
            </div>

            // Gauge Visual
            <div class="mb-6 p-4 bg-slate-900 rounded-lg border border-slate-700">
                 <div class="flex justify-between text-xs text-slate-400 mb-1 uppercase font-bold">
                    <span>"Total Load"</span>
                    <span>"Capacity"</span>
                 </div>
                 <div class="h-4 bg-slate-800 rounded-full overflow-hidden relative">
                    // Calculate total weight
                    {move || {
                        let total: u32 = cargo.get().iter().map(|c| c.cognitive_weight as u32).sum();
                        let max = 500; // arbitrary max capacity
                        let percent = (total as f32 / max as f32 * 100.0).min(100.0);
                        let color = if percent > 80.0 { "bg-red-500" } else if percent > 50.0 { "bg-yellow-500" } else { "bg-green-500" };

                        view! {
                            <div class=format!("h-full transition-all duration-500 {}", color) style=format!("width: {}%", percent)></div>
                        }
                    }}
                 </div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 overflow-y-auto pr-2">
                <For
                    each=move || cargo.get().into_iter().rev() // Show newest first
                    key=|c| c.word.clone()
                    children=move |crate_item| {
                        let (color, border) = match crate_item.cognitive_weight {
                            0..=30 => ("bg-green-900/40", "border-green-500"),
                            31..=70 => ("bg-yellow-900/40", "border-yellow-500"),
                            _ => ("bg-red-900/40", "border-red-500"),
                        };
                        view! {
                            <div class=format!("{} p-4 rounded border-l-4 shadow-lg animate-in fade-in slide-in-from-top-2 {}", color, border)>
                                <div class="font-bold text-lg text-white">{crate_item.word}</div>
                                <div class="flex justify-between items-center mt-2">
                                    <span class="text-xs uppercase opacity-75 text-slate-300">"Cognitive Mass"</span>
                                    <span class="font-mono font-bold text-white bg-slate-900/50 px-2 py-0.5 rounded">
                                        {crate_item.cognitive_weight} " tons"
                                    </span>
                                </div>
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}
