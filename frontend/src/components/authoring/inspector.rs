use common::expert::StoryNode;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

#[component]
pub fn Inspector(
    #[prop(into)] node: RwSignal<StoryNode>,
    #[prop(into)] on_close: Callback<()>,
    #[prop(into)] on_delete: Callback<()>,
) -> impl IntoView {
    let (analysis_result, set_analysis_result) = signal(None::<String>);
    let (is_analyzing, set_is_analyzing) = signal(false);
    let node_data = move || node.get();

    let run_inspection = move |_| {
        set_is_analyzing.set(true);
        set_analysis_result.set(None);

        // Mock AI Inspection for now
        // In real implementation, this calls backend Gemma 3
        leptos::task::spawn_local(async move {
            // Simulate delay
            gloo_timers::future::sleep(std::time::Duration::from_millis(1500)).await;

            let content_len = node.get().content.len();
            let passenger_count = node.get().passenger_count;

            let result = if passenger_count > 4 {
                "⚠️ **Overload Detected!**\n\nThis station has too many passengers (concepts). Consider splitting this node into two stations to reduce Cognitive Load."
            } else if content_len < 20 {
                "⚠️ **Broken Rail!**\n\nThe content here is too sparse. Students might get stranded. Add more context or scaffolding."
            } else {
                "✅ **Track Clear**\n\nThis station is well-balanced. Cognitive Load is within safe limits."
            };

            set_analysis_result.set(Some(result.to_string()));
            set_is_analyzing.set(false);
        });
    };

    view! {
        <div class="absolute right-0 top-0 bottom-0 w-96 bg-slate-900 border-l border-white/10 shadow-2xl flex flex-col z-30">
            // Header
            <div class="h-16 flex items-center justify-between px-6 border-b border-white/10 bg-slate-950 shrink-0">
                <div class="flex items-center gap-2">
                    <span class="text-2xl">"🕵️"</span>
                    <h2 class="text-lg font-bold text-white tracking-wider">"THE INSPECTOR"</h2>
                </div>
                <button
                    class="text-slate-400 hover:text-white transition-colors"
                    on:click=move |_| on_close.run(())
                >
                    "✕"
                </button>
            </div>

            // Content
            <div class="flex-grow overflow-y-auto p-6 space-y-6">
                // Node Identity
                <div class="space-y-2">
                    <label class="text-xs uppercase font-bold text-slate-500">"Station Name"</label>
                    <input
                        type="text"
                        class="w-full bg-slate-800 border border-slate-700 rounded p-2 text-white focus:border-cyan-500 focus:outline-none transition-colors"
                        prop:value=move || node.get().title
                        on:input=move |ev| {
                            let val = event_target_value(&ev);
                            node.update(|n| n.title = val);
                        }
                    />
                </div>

                <div class="space-y-2">
                    <label class="text-xs uppercase font-bold text-slate-500">"Learning Content"</label>
                    <textarea
                        class="w-full h-32 bg-slate-800 border border-slate-700 rounded p-2 text-white focus:border-cyan-500 focus:outline-none transition-colors font-mono text-sm"
                        prop:value=move || node.get().content
                        on:input=move |ev| {
                            let val = event_target_value(&ev);
                            node.update(|n| n.content = val);
                        }
                    />
                </div>

                // Cognitive Load Controls
                <div class="p-4 bg-slate-800 rounded border border-slate-700 space-y-4">
                    <h3 class="font-bold text-slate-300 flex items-center gap-2">
                        "📦 Cargo Manifest"
                    </h3>

                    <div class="space-y-2">
                        <div class="flex justify-between text-sm">
                            <span class="text-slate-400">"Passenger Count"</span>
                            <span class="font-mono font-bold text-white">{move || node.get().passenger_count}</span>
                        </div>
                        <input
                            type="range"
                            min="1"
                            max="10"
                            class="w-full accent-cyan-500"
                            prop:value=move || node.get().passenger_count
                            on:input=move |ev| {
                                let val = event_target_value(&ev).parse::<u8>().unwrap_or(1);
                                node.update(|n| n.passenger_count = val);
                            }
                        />
                        <div class="flex justify-between text-xs text-slate-500">
                            <span>"Light"</span>
                            <span>"Heavy"</span>
                        </div>
                    </div>
                </div>

                // Requirements / Triggers Section
                <div class="space-y-2">
                    <label class="block text-xs uppercase font-bold text-slate-500">"Track Requirements"</label>
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

                // AI Inspector Section
                <div class="border-t border-white/10 pt-6">
                    <button
                        class="w-full py-3 bg-boilermaker-gold hover:bg-white text-black font-bold rounded shadow-lg transition-all flex items-center justify-center gap-2"
                        on:click=run_inspection
                        disabled=move || is_analyzing.get()
                    >
                        {move || if is_analyzing.get() {
                            view! { <span class="animate-spin">"⚙️"</span> " Inspecting..." }.into_any()
                        } else {
                            view! { <span>"🔍"</span> " Run Inspection" }.into_any()
                        }}
                    </button>

                    {move || analysis_result.get().map(|result| {
                        view! {
                            <div class="mt-4 p-4 bg-slate-800 rounded border-l-4 border-cyan-500 animate-in fade-in slide-in-from-bottom-2">
                                <div class="prose prose-invert prose-sm">
                                    <pre class="whitespace-pre-wrap font-sans text-slate-300">{result}</pre>
                                </div>
                            </div>
                        }
                    })}
                </div>

                // Delete Button
                <div class="pt-4 border-t border-white/10">
                    <button
                        class="w-full px-4 py-2 bg-red-900/50 hover:bg-red-900 text-red-200 border border-red-800 rounded transition-colors text-sm font-bold"
                        on:click=move |_| on_delete.run(())
                    >
                        "Demolish Station"
                    </button>
                </div>
            </div>
        </div>
    }
}
