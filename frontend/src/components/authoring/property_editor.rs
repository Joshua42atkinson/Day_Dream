use common::expert::{StoryNode, StoryChoice};
use leptos::prelude::*;

#[component]
pub fn PropertyEditor(
    #[prop(into)] node: RwSignal<StoryNode>,
    #[prop(into)] on_close: Callback<()>,
    #[prop(into)] on_delete: Callback<()>,
) -> impl IntoView {
    let node_data = move || node.get();

    view! {
        <div class="absolute right-0 top-0 bottom-0 w-80 bg-slate-900 border-l border-white/10 p-4 shadow-xl z-20 flex flex-col gap-4 overflow-y-auto">
            <div class="flex justify-between items-center border-b border-white/10 pb-2">
                <h2 class="text-lg font-bold text-white">"Properties"</h2>
                <button
                    class="text-slate-400 hover:text-white"
                    on:click=move |_| on_close.run(())
                >
                    "✕"
                </button>
            </div>

            // Core Info
            <div class="space-y-2">
                <label class="block text-xs font-bold uppercase tracking-wider text-slate-400">"Title"</label>
                <input
                    type="text"
                    class="w-full bg-slate-800 border border-slate-700 rounded p-2 text-white text-sm focus:border-cyan-500 focus:outline-none"
                    prop:value=move || node_data().title
                    on:input=move |ev| {
                        let val = event_target_value(&ev);
                        node.update(|n| n.title = val);
                    }
                />
            </div>

            <div class="space-y-2">
                <label class="block text-xs font-bold uppercase tracking-wider text-slate-400">"Story Content"</label>
                <textarea
                    class="w-full h-24 bg-slate-800 border border-slate-700 rounded p-2 text-white text-sm focus:border-cyan-500 focus:outline-none font-sans resize-none"
                    prop:value=move || node_data().content
                    on:input=move |ev| {
                        let val = event_target_value(&ev);
                        node.update(|n| n.content = val);
                    }
                />
            </div>

            // Somatic & Lesson Elements (VAAM Power Word, Song, Image, Frequency Gate)
            <div class="pt-4 border-t border-white/5 space-y-3">
                <h3 class="text-xs font-extrabold text-cyan-400 uppercase tracking-widest">"Somatic & Lesson Elements"</h3>
                
                // Active Subject Power Word (VAAM)
                <div class="space-y-1">
                    <label class="block text-xs font-medium text-slate-400">"Subject Word (VAAM)"</label>
                    <input
                        type="text"
                        placeholder="e.g. Unison, Courage"
                        class="w-full bg-slate-800 border border-slate-700 rounded p-2 text-white text-xs focus:border-cyan-500 focus:outline-none"
                        prop:value=move || node_data().subject_word
                        on:input=move |ev| {
                            let val = event_target_value(&ev);
                            node.update(|n| n.subject_word = val);
                        }
                    />
                </div>

                // Background Image Path
                <div class="space-y-1">
                    <label class="block text-xs font-medium text-slate-400">"Illustration Image URL"</label>
                    <input
                        type="text"
                        placeholder="e.g. /assets/adventures/arrival.png"
                        class="w-full bg-slate-800 border border-slate-700 rounded p-2 text-white text-xs focus:border-cyan-500 focus:outline-none"
                        prop:value=move || node_data().image_url.unwrap_or_default()
                        on:input=move |ev| {
                            let val = event_target_value(&ev);
                            node.update(|n| n.image_url = if val.is_empty() { None } else { Some(val) });
                        }
                    />
                </div>

                // Song/Audio Path
                <div class="space-y-1">
                    <label class="block text-xs font-medium text-slate-400">"Backing Audio URL"</label>
                    <input
                        type="text"
                        placeholder="e.g. /assets/audio/houlton_skies.mp3"
                        class="w-full bg-slate-800 border border-slate-700 rounded p-2 text-white text-xs focus:border-cyan-500 focus:outline-none"
                        prop:value=move || node_data().audio_url.unwrap_or_default()
                        on:input=move |ev| {
                            let val = event_target_value(&ev);
                            node.update(|n| n.audio_url = if val.is_empty() { None } else { Some(val) });
                        }
                    />
                </div>

                // Target Frequency Gate (Hz)
                <div class="space-y-1">
                    <label class="block text-xs font-medium text-slate-400">"Somatic Pitch Gate (Hz)"</label>
                    <input
                        type="number"
                        step="0.01"
                        placeholder="e.g. 440.0"
                        class="w-full bg-slate-800 border border-slate-700 rounded p-2 text-white text-xs focus:border-cyan-500 focus:outline-none font-mono"
                        prop:value=move || node_data().target_freq.map(|f| f.to_string()).unwrap_or_default()
                        on:input=move |ev| {
                            let val = event_target_value(&ev);
                            node.update(|n| {
                                n.target_freq = if val.is_empty() {
                                    None
                                } else {
                                    val.parse::<f32>().ok()
                                };
                            });
                        }
                    />
                </div>

                // Channel (MIND | HEART | BODY | ACTION)
                <div class="space-y-1">
                    <label class="block text-xs font-medium text-slate-400">"Channel (MIND | HEART | BODY | ACTION)"</label>
                    <select
                        class="w-full bg-slate-800 border border-slate-700 rounded p-2 text-white text-xs focus:border-cyan-500 focus:outline-none"
                        prop:value=move || node_data().channel.clone().unwrap_or_default()
                        on:change=move |ev| {
                            let val = event_target_value(&ev);
                            node.update(|n| n.channel = if val.is_empty() { None } else { Some(val) });
                        }
                    >
                        <option value="">"Auto-detect"</option>
                        <option value="mind">"MIND — The Sage"</option>
                        <option value="heart">"HEART — The Mystic"</option>
                        <option value="body">"BODY — The Healer"</option>
                        <option value="action">"ACTION — The Builder"</option>
                    </select>
                </div>

                // Depth Question (Socratic reflection)
                <div class="space-y-1">
                    <label class="block text-xs font-medium text-slate-400">"Depth Question (Double-tap)"</label>
                    <textarea
                        placeholder="e.g. What does it feel like to choose the harder path?"
                        class="w-full h-16 bg-slate-800 border border-slate-700 rounded p-2 text-white text-xs focus:border-cyan-500 focus:outline-none font-sans resize-none"
                        prop:value=move || node_data().depth.clone().unwrap_or_default()
                        on:input=move |ev| {
                            let val = event_target_value(&ev);
                            node.update(|n| n.depth = if val.is_empty() { None } else { Some(val) });
                        }
                    />
                </div>
            </div>

            // Branching choices section
            <div class="pt-4 border-t border-white/5 space-y-3">
                <div class="flex justify-between items-center">
                    <h3 class="text-xs font-extrabold text-cyan-400 uppercase tracking-widest">"Branching Paths"</h3>
                    <button
                        class="px-2 py-0.5 bg-slate-850 hover:bg-slate-800 border border-white/10 rounded text-slate-300 hover:text-white text-[10px] font-extrabold tracking-wider uppercase transition-colors"
                        on:click=move |_| {
                            let new_choice = StoryChoice {
                                id: uuid::Uuid::new_v4().to_string(),
                                label: "Action".to_string(),
                                description: "Description of what happens...".to_string(),
                                leads_to: "".to_string(),
                                pitch_gate: None,
                                virtue: None,
                            };
                            node.update(|n| n.choices.push(new_choice));
                        }
                    >
                        "+ Add Choice"
                    </button>
                </div>

                <div class="space-y-3">
                    {move || node_data().choices.into_iter().enumerate().map(|(idx, choice)| {
                        let choice_id = choice.id.clone();
                        let del_id = choice_id.clone();
                        let label_id = choice_id.clone();
                        let desc_id = choice_id.clone();
                        let leads_id = choice_id.clone();
                        let pitch_id = choice_id.clone();
                        let virtue_id = choice_id.clone();
                        view! {
                            <div class="border border-white/5 p-2 rounded bg-slate-950/40 flex flex-col gap-2 text-xs">
                                <div class="flex justify-between items-center border-b border-white/5 pb-1">
                                    <span class="text-cyan-500 font-bold">"Path #" {idx + 1}</span>
                                    <button
                                        class="text-red-400 hover:text-red-300 font-bold text-[10px] uppercase"
                                        on:click=move |_| {
                                            node.update(|n| {
                                                n.choices.retain(|c| c.id != del_id);
                                            });
                                        }
                                    >
                                        "Delete"
                                    </button>
                                </div>
                                
                                // Choice Label
                                <div class="space-y-0.5">
                                    <span class="text-[9px] uppercase tracking-wider text-slate-500 font-medium">"Action Label"</span>
                                    <input
                                        type="text"
                                        placeholder="e.g. State your name"
                                        class="w-full bg-slate-900 border border-slate-800 rounded p-1.5 text-white focus:border-cyan-500 focus:outline-none"
                                        prop:value=choice.label.clone()
                                        on:input=move |ev| {
                                            let val = event_target_value(&ev);
                                            let target_id = label_id.clone();
                                            node.update(|n| {
                                                if let Some(c) = n.choices.iter_mut().find(|c| c.id == target_id) {
                                                    c.label = val;
                                                }
                                            });
                                        }
                                    />
                                </div>

                                // Choice Description
                                <div class="space-y-0.5">
                                    <span class="text-[9px] uppercase tracking-wider text-slate-500 font-medium">"Narrative Description"</span>
                                    <input
                                        type="text"
                                        placeholder="e.g. You bow deeply..."
                                        class="w-full bg-slate-900 border border-slate-800 rounded p-1.5 text-white focus:border-cyan-500 focus:outline-none"
                                        prop:value=choice.description.clone()
                                        on:input=move |ev| {
                                            let val = event_target_value(&ev);
                                            let target_id = desc_id.clone();
                                            node.update(|n| {
                                                if let Some(c) = n.choices.iter_mut().find(|c| c.id == target_id) {
                                                    c.description = val;
                                                }
                                            });
                                        }
                                    />
                                </div>

                                // Destination ID
                                <div class="space-y-0.5">
                                    <span class="text-[9px] uppercase tracking-wider text-slate-500 font-medium">"Destination Node ID"</span>
                                    <input
                                        type="text"
                                        placeholder="Target Node UUID"
                                        class="w-full bg-slate-900 border border-slate-800 rounded p-1.5 text-white focus:border-cyan-500 focus:outline-none font-mono text-[10px]"
                                        prop:value=choice.leads_to.clone()
                                        on:input=move |ev| {
                                            let val = event_target_value(&ev);
                                            let target_id = leads_id.clone();
                                            node.update(|n| {
                                                if let Some(c) = n.choices.iter_mut().find(|c| c.id == target_id) {
                                                    c.leads_to = val;
                                                }
                                            });
                                        }
                                    />
                                </div>

                                // Choice Pitch Gate (Hz)
                                <div class="space-y-0.5">
                                    <span class="text-[9px] uppercase tracking-wider text-slate-500 font-medium">"Choice Pitch Gate Hz (Optional)"</span>
                                    <input
                                        type="number"
                                        step="0.01"
                                        placeholder="e.g. 440.0"
                                        class="w-full bg-slate-900 border border-slate-800 rounded p-1.5 text-white focus:border-cyan-500 focus:outline-none font-mono text-[10px]"
                                        prop:value=choice.pitch_gate.map(|f| f.to_string()).unwrap_or_default()
                                        on:input=move |ev| {
                                            let val = event_target_value(&ev);
                                            let target_id = pitch_id.clone();
                                            node.update(|n| {
                                                if let Some(c) = n.choices.iter_mut().find(|c| c.id == target_id) {
                                                    c.pitch_gate = if val.is_empty() {
                                                        None
                                                    } else {
                                                        val.parse::<f32>().ok()
                                                    };
                                                }
                                            });
                                        }
                                    />
                                </div>

                                // Choice Virtue Tag
                                <div class="space-y-0.5">
                                    <span class="text-[9px] uppercase tracking-wider text-slate-500 font-medium">"Virtue Tag (Optional)"</span>
                                    <input
                                        type="text"
                                        placeholder="e.g. curiosity, courage, depth"
                                        class="w-full bg-slate-900 border border-slate-800 rounded p-1.5 text-white focus:border-cyan-500 focus:outline-none text-[10px]"
                                        prop:value=choice.virtue.clone().unwrap_or_default()
                                        on:input=move |ev| {
                                            let val = event_target_value(&ev);
                                            let target_id = virtue_id.clone();
                                            node.update(|n| {
                                                if let Some(c) = n.choices.iter_mut().find(|c| c.id == target_id) {
                                                    c.virtue = if val.is_empty() { None } else { Some(val) };
                                                }
                                            });
                                        }
                                    />
                                </div>
                            </div>
                        }

                    }).collect::<Vec<_>>()}
                </div>
            </div>

            // Node Metadata
            <div class="text-[10px] text-slate-500 font-mono mt-auto pt-4 border-t border-white/5">
                "Node ID: " {move || node_data().id}
            </div>

            // Actions
            <div class="pt-2 border-t border-white/10">
                <button
                    class="w-full px-4 py-2 bg-red-900/50 hover:bg-red-900 text-red-200 border border-red-800 rounded transition-colors text-xs font-bold uppercase tracking-wider"
                    on:click=move |_| on_delete.run(())
                >
                    "Delete Node"
                </button>
            </div>
        </div>
    }
}

