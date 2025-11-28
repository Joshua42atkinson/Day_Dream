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

    // Mock AI Analysis Function
    let weigh_word = move |_| {
        let w = input_word.get();
        if w.is_empty() {
            return;
        }

        // In production, this calls the Backend (Large AI) to analyze the word
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
    };

    view! {
        <div class="p-4 bg-slate-800 text-white h-full">
            <h2 class="text-2xl mb-4">"The Weigh Station"</h2>
            <div class="flex gap-2 mb-4">
                <input
                    type="text"
                    class="bg-slate-700 p-2 rounded text-white"
                    placeholder="Enter Vocabulary Word..."
                    on:input=move |ev: Event| {
                        let target = ev.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                        set_input_word.set(target.value());
                    }
                    prop:value=move || input_word.get()
                />
                <button
                    class="bg-cyan-600 px-4 py-2 rounded hover:bg-cyan-500 transition"
                    on:click=weigh_word
                >
                    "Weigh Cargo"
                </button>
            </div>

            <div class="grid grid-cols-3 gap-4">
                <For
                    each=move || cargo.get()
                    key=|c| c.word.clone()
                    children=move |crate_item| {
                        let color = match crate_item.cognitive_weight {
                            0..=30 => "bg-green-600",
                            31..=70 => "bg-yellow-600",
                            _ => "bg-red-600",
                        };
                        view! {
                            <div class=format!("{} p-4 rounded border border-slate-500 shadow-lg", color)>
                                <div class="font-bold text-lg">{crate_item.word}</div>
                                <div class="text-xs uppercase opacity-75">
                                    "Weight: " {crate_item.cognitive_weight} " tons"
                                </div>
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}
