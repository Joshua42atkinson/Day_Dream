use common::expert::StoryNode;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
// use web_sys::document; // Removed invalid import

#[component]
pub fn PropertyEditor(
    #[prop(into)] node: RwSignal<StoryNode>,
    #[prop(into)] on_close: Callback<()>,
    #[prop(into)] on_delete: Callback<()>,
) -> impl IntoView {
    let node_data = move || node.get();

    // Cognitive Load Logic
    let passenger_status = move || {
        let count = node_data().passenger_count;
        if count <= 4 {
            (
                "text-signal-green",
                "Safe Load",
                "bg-signal-green/10 border-signal-green",
            )
        } else if count == 5 {
            (
                "text-signal-yellow",
                "At Capacity",
                "bg-signal-yellow/10 border-signal-yellow",
            )
        } else {
            (
                "text-signal-red",
                "OVERLOAD!",
                "bg-signal-red/10 border-signal-red",
            )
        }
    };

    view! {
        <div class="absolute right-0 top-0 bottom-0 w-80 bg-slate-900 border-l border-white/10 p-4 shadow-xl z-20 flex flex-col gap-4 overflow-y-auto">
            <div class="flex justify-between items-center border-b border-white/10 pb-2">
                <h2 class="text-lg font-bold text-white flex items-center gap-2">
                    <span>"🚉"</span> "Station Properties"
                </h2>
                <button
                    class="text-slate-400 hover:text-white"
                    on:click=move |_| on_close.run(())
                >
                    "✕"
                </button>
            </div>

            <div class="space-y-2">
                <label class="block text-sm font-medium text-slate-400">"Station Name"</label>
                <input
                    type="text"
                    class="w-full bg-slate-800 border border-slate-700 rounded p-2 text-white focus:border-boilermaker-gold focus:outline-none"
                    prop:value=move || node_data().title
                    on:input=move |ev| {
                        let val = event_target_value(&ev);
                        node.update(|n| n.title = val);
                    }
                />
            </div>

            // Cognitive Load Section
            <div class="bg-slate-800/50 p-3 rounded border border-slate-700 space-y-3">
                <div class="flex justify-between items-center">
                    <label class="text-sm font-bold text-slate-300">"Passenger Load"</label>
                    <span class="text-xs text-slate-500">"(Concepts)"</span>
                </div>

                <div class="flex items-center gap-3">
                    <button
                        class="w-8 h-8 rounded bg-slate-700 hover:bg-slate-600 text-white font-bold"
                        on:click=move |_| node.update(|n| if n.passenger_count > 0 { n.passenger_count -= 1 })
                    >"-"</button>

                    <span class="text-xl font-mono font-bold text-white w-8 text-center">
                        {move || node_data().passenger_count}
                    </span>

                    <button
                        class="w-8 h-8 rounded bg-slate-700 hover:bg-slate-600 text-white font-bold"
                        on:click=move |_| node.update(|n| if n.passenger_count < 10 { n.passenger_count += 1 })
                    >"+"</button>
                </div>

                // Status Indicator
                <div class=move || format!("p-2 rounded border text-center text-sm font-bold transition-colors {}", passenger_status().2)>
                    <span class=move || passenger_status().0>
                        {move || passenger_status().1}
                    </span>
                </div>

                <p class="text-[10px] text-slate-500 leading-tight">
                    "Novice learners can handle 3-4 new concepts per station. Exceeding this may cause cognitive overload."
                </p>
            </div>

            <div class="space-y-2 flex-grow flex flex-col">
                <label class="block text-sm font-medium text-slate-400">"Learning Objectives & Content"</label>
                <textarea
                    class="w-full h-32 bg-slate-800 border border-slate-700 rounded p-2 text-white focus:border-boilermaker-gold focus:outline-none font-mono text-sm resize-none"
                    prop:value=move || node_data().content
                    on:input=move |ev| {
                        let val = event_target_value(&ev);
                        node.update(|n| n.content = val);
                    }
                />
            </div>

            <div class="text-xs text-slate-500 font-mono">
                "ID: " {move || node_data().id}
            </div>

            // Requirements / Triggers Section
            <div class="space-y-2">
                <label class="block text-sm font-medium text-slate-400">"Requirements (Triggers)"</label>
                <div class="bg-slate-800 border border-slate-700 rounded p-2 space-y-2">
                    <For
                        each=move || node_data().required_stats.into_iter()
                        key=|stat| stat.0.clone()
                        children=move |(stat, val)| {
                            let stat_clone = stat.clone();
                            view! {
                                <div class="flex justify-between items-center bg-slate-700/50 p-1 rounded">
                                    <span class="text-xs text-white font-mono">{stat.clone()} ": " {val}</span>
                                    <button
                                        class="text-red-400 hover:text-red-300 px-1"
                                        on:click=move |_| {
                                            node.update(|n| { n.required_stats.remove(&stat_clone); });
                                        }
                                    >
                                        "×"
                                    </button>
                                </div>
                            }
                        }
                    />

                    <div class="flex gap-2">
                        <input
                            type="text"
                            id="new-stat-name"
                            class="w-2/3 bg-slate-900 border border-slate-600 rounded p-1 text-xs text-white"
                            placeholder="Stat"
                        />
                        <input
                            type="number"
                            id="new-stat-val"
                            class="w-1/3 bg-slate-900 border border-slate-600 rounded p-1 text-xs text-white"
                            placeholder="Val"
                        />
                        <button
                            class="bg-cyan-600 hover:bg-cyan-500 text-white rounded px-2 text-xs"
                            on:click=move |_| {
                                let doc = web_sys::window().unwrap().document().unwrap();
                                let name_el = doc.get_element_by_id("new-stat-name").unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                let val_el = doc.get_element_by_id("new-stat-val").unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                let name = name_el.value();
                                let val = val_el.value().parse::<u32>().unwrap_or(0);

                                if !name.is_empty() {
                                    node.update(|n| { n.required_stats.insert(name, val); });
                                    name_el.set_value("");
                                    val_el.set_value("");
                                }
                            }
                        >
                            "+"
                        </button>
                    </div>
                </div>
            </div>

            <div class="pt-4 border-t border-white/10">
                <button
                    class="w-full px-4 py-2 bg-red-900/50 hover:bg-red-900 text-red-200 border border-red-800 rounded transition-colors text-sm font-bold"
                    on:click=move |_| on_delete.run(())
                >
                    "Demolish Station"
                </button>
            </div>
        </div>
    }
}
