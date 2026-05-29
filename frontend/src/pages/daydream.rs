use crate::components::glass_panel::GlassPanel;
use crate::components::loading_spinner::LoadingSpinner;
use crate::api::{get_graph, save_graph};
use common::expert::{StoryGraph, StoryNode, StoryChoice, Connection};
use leptos::prelude::*;
use leptos::task::spawn_local;
#[cfg(target_arch = "wasm32")]
use std::cell::RefCell;
#[cfg(target_arch = "wasm32")]
use std::rc::Rc;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

#[cfg(target_arch = "wasm32")]
async fn async_sleep(ms: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        if let Some(window) = web_sys::window() {
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms);
        }
    });
    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
}

#[cfg(not(target_arch = "wasm32"))]
async fn async_sleep(_ms: i32) {}

// ---------------------------------------------------------
// 1. Thread-Local Browser Storage (Client Only - non-Send/non-Sync)
// ---------------------------------------------------------
#[cfg(target_arch = "wasm32")]
thread_local! {
    static AUDIO_CONTEXT: RefCell<Option<web_sys::AudioContext>> = RefCell::new(None);
    static OSCILLATOR: RefCell<Option<web_sys::OscillatorNode>> = RefCell::new(None);
    static GAIN: RefCell<Option<web_sys::GainNode>> = RefCell::new(None);
    static ANALYSER: RefCell<Option<web_sys::AnalyserNode>> = RefCell::new(None);
    static MIC_STREAM: RefCell<Option<web_sys::MediaStream>> = RefCell::new(None);
}

// ---------------------------------------------------------
// 2. Web Audio Helper Synthesizers (Compile-Gated)
// ---------------------------------------------------------
#[cfg(target_arch = "wasm32")]
fn get_audio_context() -> Option<web_sys::AudioContext> {
    AUDIO_CONTEXT.with(|ctx| {
        let mut borrow = ctx.borrow_mut();
        if borrow.is_none() {
            if let Ok(c) = web_sys::AudioContext::new() {
                *borrow = Some(c);
            }
        }
        borrow.clone()
    })
}

#[cfg(target_arch = "wasm32")]
fn play_reference_tone(freq: f32) {
    stop_reference_tone();
    if let Some(ctx) = get_audio_context() {
        let _ = ctx.resume();
        if let Ok(osc) = ctx.create_oscillator() {
            if let Ok(gain) = ctx.create_gain() {
                osc.set_type(web_sys::OscillatorType::Sine);
                osc.frequency().set_value(freq);
                gain.gain().set_value(0.12); // Soft background humming volume

                let _ = osc.connect_with_audio_node(&gain);
                let _ = gain.connect_with_audio_node(&ctx.destination());
                let _ = osc.start();

                OSCILLATOR.with(|o| *o.borrow_mut() = Some(osc));
                GAIN.with(|g| *g.borrow_mut() = Some(gain));
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn play_reference_tone(_freq: f32) {}

#[cfg(target_arch = "wasm32")]
fn stop_reference_tone() {
    OSCILLATOR.with(|o| {
        if let Some(osc) = o.borrow_mut().take() {
            let _ = osc.stop();
        }
    });
    GAIN.with(|g| *g.borrow_mut() = None);
}

#[cfg(not(target_arch = "wasm32"))]
fn stop_reference_tone() {}

#[cfg(target_arch = "wasm32")]
fn play_pling_success() {
    if let Some(ctx) = get_audio_context() {
        let _ = ctx.resume();
        let now = ctx.current_time();
        
        // Build E-major fifth harmonic pling chord (golden feedback)
        for &freq in &[432.0, 648.0, 864.0] {
            if let Ok(osc) = ctx.create_oscillator() {
                if let Ok(gain) = ctx.create_gain() {
                    osc.set_type(web_sys::OscillatorType::Sine);
                    osc.frequency().set_value(freq);

                    let _ = gain.gain().set_value_at_time(0.0, now);
                    let _ = gain.gain().linear_ramp_to_value_at_time(0.15, now + 0.06);
                    let _ = gain.gain().exponential_ramp_to_value_at_time(0.0001, now + 1.4);

                    let _ = osc.connect_with_audio_node(&gain);
                    let _ = gain.connect_with_audio_node(&ctx.destination());
                    let _ = osc.start();
                    let _ = osc.stop_with_when(now + 1.5);
                }
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn play_pling_success() {}

// ---------------------------------------------------------
// 3. Microphone Capture Lifecycle
// ---------------------------------------------------------
#[cfg(target_arch = "wasm32")]
fn start_microphone(
    set_listening: WriteSignal<bool>,
) {
    let window = web_sys::window().unwrap();
    let navigator = window.navigator();
    let media_devices = navigator.media_devices().unwrap();
    
    let mut constraints = web_sys::MediaStreamConstraints::new();
    constraints.audio(&JsValue::from_bool(true));
    
    set_listening.set(true);
    
    spawn_local(async move {
        let mut success = false;
        if let Ok(promise) = media_devices.get_user_media_with_constraints(&constraints) {
            if let Ok(stream_val) = wasm_bindgen_futures::JsFuture::from(promise).await {
                let stream: web_sys::MediaStream = stream_val.into();
                MIC_STREAM.with(|s| *s.borrow_mut() = Some(stream.clone()));
                
                if let Some(ctx) = get_audio_context() {
                    let _ = ctx.resume();
                    if let Ok(source) = ctx.create_media_stream_source(&stream) {
                        if let Ok(node) = ctx.create_analyser() {
                            node.set_fft_size(2048);
                            let _ = source.connect_with_audio_node(&node);
                            ANALYSER.with(|a| *a.borrow_mut() = Some(node));
                            leptos::logging::log!("Microphone connected successfully.");
                            success = true;
                        }
                    }
                }
            }
        }
        if !success {
            set_listening.set(false);
            leptos::logging::warn!("Microphone access failed or was denied.");
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn start_microphone(_s: WriteSignal<bool>) {}

#[cfg(target_arch = "wasm32")]
fn stop_microphone(
    set_listening: WriteSignal<bool>,
) {
    MIC_STREAM.with(|s| {
        if let Some(stream) = s.borrow_mut().take() {
            let tracks = stream.get_tracks();
            for i in 0..tracks.length() {
                if let Ok(track) = tracks.get(i).dyn_into::<web_sys::MediaStreamTrack>() {
                    track.stop();
                }
            }
        }
    });
    ANALYSER.with(|a| *a.borrow_mut() = None);
    set_listening.set(false);
}

#[cfg(not(target_arch = "wasm32"))]
fn stop_microphone(_s: WriteSignal<bool>) {}

// ---------------------------------------------------------
// 4. Pitch Detection (Autocorrelation)
// ---------------------------------------------------------
#[cfg(target_arch = "wasm32")]
fn detect_pitch(buffer: &[f32], sample_rate: f32) -> Option<f32> {
    let len = buffer.len();
    let min_lag = (sample_rate / 1000.0) as usize;
    let max_lag = (sample_rate / 50.0) as usize;
    
    if max_lag >= len {
        return None;
    }

    // Root Mean Square (RMS) energy threshold to filter out low-volume ambient room noise
    let mut rms = 0.0;
    for &sample in buffer {
        rms += sample * sample;
    }
    rms = (rms / len as f32).sqrt();
    if rms < 0.008 {
        return None; // sound is too quiet to calibrate
    }
    
    let mut r = vec![0.0; max_lag];
    let mut max_val = 0.0;
    
    for lag in min_lag..max_lag {
        let mut sum = 0.0;
        for i in 0..(len - lag) {
            sum += buffer[i] * buffer[i + lag];
        }
        r[lag] = sum;
    }
    
    let mut peak_lag = -1;
    for lag in (min_lag + 1)..(max_lag - 1) {
        if r[lag] > r[lag - 1] && r[lag] > r[lag + 1] {
            if r[lag] > max_val {
                max_val = r[lag];
                peak_lag = lag as i32;
            }
        }
    }
    
    if peak_lag != -1 && max_val > 0.02 {
        let freq = sample_rate / (peak_lag as f32);
        if freq >= 50.0 && freq <= 1000.0 {
            return Some(freq);
        }
    }
    None
}

// RequestAnimationFrame helper
#[cfg(target_arch = "wasm32")]
fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    if let Some(window) = web_sys::window() {
        let _ = window.request_animation_frame(f.as_ref().unchecked_ref());
    }
}


// ---------------------------------------------------------
// 5. Main Component Page
// ---------------------------------------------------------
#[component]
pub fn Daydream() -> impl IntoView {
    // Story Graph State
    let (story_graph, set_story_graph) = signal(None::<StoryGraph>);
    let (active_node, set_active_node) = signal(None::<StoryNode>);
    let (is_graph_loading, set_is_graph_loading) = signal(true);
    let (transitioning, set_transitioning) = signal(false);

    // Somatic Gate States
    let (is_listening, set_is_listening) = signal(false);
    let (detected_pitch, set_detected_pitch) = signal(None::<f32>);
    let (match_progress, set_match_progress) = signal(0.0f32);
    let (is_matched, set_is_matched) = signal(false);

    // Reference Synth state
    let (is_playing_ref, set_is_playing_ref) = signal(false);
    let (vocal_match_percentage, set_vocal_match_percentage) = signal(50.0);
    let (waveform_heights, set_waveform_heights) = signal(vec![0.0f32; 16]);
    
    // Suppress target-dependent unused warnings on server targets
    let _ = set_waveform_heights;

    // Word encounter log (for SpellBook)
    let (words_encountered, set_words_encountered) = signal(Vec::<String>::new());
    let _ = words_encountered; // Will be used by SpellBook route

    // Load active graph on mount
    Effect::new(move |_| {
        spawn_local(async move {
            set_is_graph_loading.set(true);
            match get_graph().await {
                Ok(graph) => {
                    if !graph.nodes.is_empty() {
                        set_story_graph.set(Some(graph.clone()));
                        if let Some(start) = graph.nodes.first() {
                            set_active_node.set(Some(start.clone()));
                            if start.target_freq.is_none() || start.target_freq.unwrap_or(0.0) <= 0.0 {
                                set_is_matched.set(true);
                                set_match_progress.set(100.0f32);
                            }
                        }
                    }
                }
                Err(e) => leptos::logging::error!("Failed to fetch graph: {}", e),
            }
            set_is_graph_loading.set(false);
        });
    });

    // Register unmount cleanup handler to prevent audio leaks when user navigates away
    on_cleanup(move || {
        stop_reference_tone();
        #[cfg(target_arch = "wasm32")]
        {
            MIC_STREAM.with(|s| {
                if let Some(stream) = s.borrow_mut().take() {
                    let tracks = stream.get_tracks();
                    for i in 0..tracks.length() {
                        if let Ok(track) = tracks.get(i).dyn_into::<web_sys::MediaStreamTrack>() {
                            track.stop();
                        }
                    }
                }
            });
            ANALYSER.with(|a| *a.borrow_mut() = None);
        }
    });

    // Seed Demo Adventure to Database
    let seed_demo_handler = move |_| {
        spawn_local(async move {
            set_is_graph_loading.set(true);
            
            let portal_node = StoryNode {
                id: "threshold_presence".to_string(),
                title: "The Threshold of Presence".to_string(),
                content: "You stand before a towering ancient portal of blue crystalline stone. The air hums with a deep, slow drone. To step through, you must find unison with the environment, quiet your mind, and speak with the voice of the portal.".to_string(),
                x: 100.0,
                y: 200.0,
                subject_word: "Presence".to_string(),
                image_url: Some("/assets/adventures/portal.png".to_string()),
                audio_url: None,
                target_freq: Some(432.0),
                choices: vec![
                    StoryChoice {
                        id: "choice_portal_enter".to_string(),
                        label: "Step Into the Glowing Portal".to_string(),
                        description: "You align your breath and walk into the blue light.".to_string(),
                        leads_to: "forest_mirrors".to_string(),
                        pitch_gate: None,
                        ..Default::default()
                    }
                ],
                ..Default::default()
            };
            
            let forest_node = StoryNode {
                id: "forest_mirrors".to_string(),
                title: "The Forest of Obsidian Mirrors".to_string(),
                content: "You emerge into a grove where the trees are made of highly polished black glass. A whisper echoes from your reflection: 'You will fail here, just as you have failed before.' It is the cognitive bias of self-doubt. How will you respond?".to_string(),
                x: 400.0,
                y: 200.0,
                subject_word: "Bias".to_string(),
                image_url: Some("/assets/adventures/mirrors.png".to_string()),
                audio_url: None,
                target_freq: Some(440.0),
                choices: vec![
                    StoryChoice {
                        id: "choice_curiosity".to_string(),
                        label: "Examine the glass leaf with Curiosity".to_string(),
                        description: "You choose to face the reflection and look closer.".to_string(),
                        leads_to: "crucible_resilience".to_string(),
                        pitch_gate: None,
                        ..Default::default()
                    },
                    StoryChoice {
                        id: "choice_fear".to_string(),
                        label: "Retreat into the quiet shade of Withdrawal".to_string(),
                        description: "You step back into the dark cave to avoid the voice.".to_string(),
                        leads_to: "sanctuary_withdrawal".to_string(),
                        pitch_gate: None,
                        ..Default::default()
                    }
                ],
                ..Default::default()
            };
            
            let resilience_node = StoryNode {
                id: "crucible_resilience".to_string(),
                title: "The Crucible of Resilience".to_string(),
                content: "As you look closer, the obsidian leaf turns fully transparent. You realize the whisper was not an absolute truth, but a defensive shadow. A sudden feeling of warm agency fills your chest.".to_string(),
                x: 700.0,
                y: 100.0,
                subject_word: "Growth".to_string(),
                image_url: Some("/assets/adventures/resilience.png".to_string()),
                audio_url: None,
                target_freq: Some(480.0),
                choices: vec![
                    StoryChoice {
                        id: "choice_resilience_complete".to_string(),
                        label: "Return to the Gateway of Presence".to_string(),
                        description: "You return to safety, stronger and more aware.".to_string(),
                        leads_to: "threshold_presence".to_string(),
                        pitch_gate: None,
                        ..Default::default()
                    }
                ],
                ..Default::default()
            };
            
            let withdrawal_node = StoryNode {
                id: "sanctuary_withdrawal".to_string(),
                title: "The Sanctuary of Withdrawal".to_string(),
                content: "You sit in the cold silence of the cave. Here, you are shielded from the mirrors and the voice, but the air is stagnant. You realize: withdrawing is a story, too. And you hold the pen to rewrite it.".to_string(),
                x: 700.0,
                y: 350.0,
                subject_word: "Self-Reflection".to_string(),
                image_url: Some("/assets/adventures/withdrawal.png".to_string()),
                audio_url: None,
                target_freq: Some(350.0),
                choices: vec![
                    StoryChoice {
                        id: "choice_withdrawal_return".to_string(),
                        label: "Return to the Portal of Presence".to_string(),
                        description: "You decide to try again, seeking connection.".to_string(),
                        leads_to: "threshold_presence".to_string(),
                        pitch_gate: None,
                        ..Default::default()
                    }
                ],
                ..Default::default()
            };
            
            let connections = vec![
                Connection {
                    id: "c1".to_string(),
                    from_node: "threshold_presence".to_string(),
                    to_node: "forest_mirrors".to_string(),
                },
                Connection {
                    id: "c2".to_string(),
                    from_node: "forest_mirrors".to_string(),
                    to_node: "crucible_resilience".to_string(),
                },
                Connection {
                    id: "c3".to_string(),
                    from_node: "forest_mirrors".to_string(),
                    to_node: "sanctuary_withdrawal".to_string(),
                }
            ];
            
            let demo_graph = StoryGraph {
                id: "demo_graph".to_string(),
                title: "Somatic Journey of Self-Awareness".to_string(),
                nodes: vec![portal_node, forest_node, resilience_node, withdrawal_node],
                connections,
                ..Default::default()
            };
            
            match save_graph(demo_graph).await {
                Ok(saved) => {
                    set_story_graph.set(Some(saved.clone()));
                    if let Some(start) = saved.nodes.first() {
                        set_active_node.set(Some(start.clone()));
                        set_is_matched.set(false);
                        set_match_progress.set(0.0f32);
                        stop_reference_tone();
                    }
                    leptos::logging::log!("Successfully seeded default story graph.");
                }
                Err(e) => leptos::logging::error!("Failed to save seed graph: {}", e),
            }
            set_is_graph_loading.set(false);
        });
    };

    // Microphone toggle
    let toggle_microphone_handler = move |_| {
        if is_listening.get() {
            stop_microphone(set_is_listening);
            set_detected_pitch.set(None);
            stop_reference_tone();
            set_is_playing_ref.set(false);
        } else {
            start_microphone(set_is_listening);
        }
    };

    // Reactive pitch detection frame loop (client WASM target only)
    Effect::new(move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            if is_listening.get() {
                let f = Rc::new(RefCell::new(None));
                let g = f.clone();
                
                *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
                    let node_opt = ANALYSER.with(|a| a.borrow().clone());
                    if node_opt.is_none() {
                        return; // Stopped
                    }
                    
                    let node = node_opt.unwrap();
                    let mut buffer = vec![0.0f32; 2048];
                    node.get_float_time_domain_data(&mut buffer);
                    
                    // Downsample FFT float buffer to 16 display heights
                    let mut heights = vec![0.0f32; 16];
                    let chunk = 2048 / 16;
                    for i in 0..16 {
                        let mut sum = 0.0;
                        for j in 0..chunk {
                            sum += buffer[i * chunk + j].abs();
                        }
                        let avg = sum / chunk as f32;
                        heights[i] = (avg * 350.0).min(100.0);
                    }
                    set_waveform_heights.set(heights);

                    // Detect vocal pitch
                    if let Some(pitch) = detect_pitch(&buffer, 48000.0) {
                        set_detected_pitch.set(Some(pitch));
                        
                        if let Some(node_data) = active_node.get() {
                            if let Some(target) = node_data.target_freq {
                                let error = pitch - target;
                                let pct = (((error + 30.0) / 60.0) * 100.0).clamp(0.0, 100.0);
                                set_vocal_match_percentage.set(pct);

                                if (pitch - target).abs() <= 8.0 {
                                    set_match_progress.update(|p| {
                                        *p = (*p + 2.5f32).min(100.0f32);
                                    });
                                    
                                    if match_progress.get_untracked() >= 100.0f32 && !is_matched.get_untracked() {
                                        set_is_matched.set(true);
                                        play_pling_success();
                                        stop_reference_tone();
                                        set_is_playing_ref.set(false);
                                    }
                                } else {
                                    set_match_progress.update(|p| {
                                        *p = (*p - 0.8f32).max(0.0f32);
                                    });
                                }
                            }
                        }
                    } else {
                        set_detected_pitch.set(None);
                        set_vocal_match_percentage.set(50.0);
                        set_match_progress.update(|p| {
                            *p = (*p - 0.5f32).max(0.0f32);
                        });
                    }
                    
                    if let Some(ref ref_closure) = *f.borrow() {
                        request_animation_frame(ref_closure);
                    }
                }) as Box<dyn FnMut()>));
                
                request_animation_frame(g.borrow().as_ref().unwrap());
            }
        }
    });

    // Somatic Gate Override (Accessibility bypass)
    let force_unlock_gate = move |_| {
        set_is_matched.set(true);
        set_match_progress.set(100.0f32);
        play_pling_success();
        stop_reference_tone();
        set_is_playing_ref.set(false);
    };

    // Traverse to destination node — POSTS choice to Bevy ECS backend
    let handle_choice_selection = move |leads_to: String, word: String, choice_id: String, graph_id: String| {
        if transitioning.get() {
            return;
        }

        // Trigger transition fade out
        set_transitioning.set(true);
        stop_reference_tone();
        set_is_playing_ref.set(false);

        let next_node_id = leads_to.clone();
        let action_word = word.clone();
        let action_choice = choice_id.clone();
        let action_graph = graph_id.clone();
        let action_leads = leads_to.clone();

        spawn_local(async move {
            // Record the vocabulary word the player just encountered
            set_words_encountered.update(|w| {
                if !w.contains(&action_word) {
                    w.push(action_word.clone());
                }
            });

            async_sleep(300).await;

            if let Some(ref graph) = story_graph.get_untracked() {
                if let Some(next_node) = graph.nodes.iter().find(|n| n.id == next_node_id) {
                    set_active_node.set(Some(next_node.clone()));

                    set_is_matched.set(false);
                    set_match_progress.set(0.0f32);
                    set_detected_pitch.set(None);
                    set_vocal_match_percentage.set(50.0);

                    if next_node.target_freq.is_none() || next_node.target_freq.unwrap_or(0.0) <= 0.0 {
                        set_is_matched.set(true);
                        set_match_progress.set(100.0f32);
                    }
                }
            }

            set_transitioning.set(false);
        });
    };



    view! {
        <div class="max-w-6xl mx-auto space-y-12 animate-fade-in p-6">
            
            // Premium Header with Glowing Badges
            <div class="text-center space-y-6">
                <div class="inline-flex items-center px-4 py-2 rounded-full bg-cyan-950/40 border border-cyan-500/30 text-cyan-300 text-xs font-bold uppercase tracking-widest shadow-[0_0_15px_rgba(6,182,212,0.15)]">
                    <span class="w-2.5 h-2.5 bg-cyan-400 rounded-full mr-3 animate-ping"></span>
                    "Somatic Branching Engine"
                </div>
                <h1 class="text-5xl md:text-7xl font-extrabold text-white tracking-tight">
                    "The Daydream" <span class="text-transparent bg-clip-text bg-gradient-to-r from-cyan-400 via-indigo-400 to-purple-500">"Sandbox"</span>
                </h1>
                <p class="text-base md:text-lg text-slate-400 max-w-3xl mx-auto leading-relaxed">
                    "Ground your body, find your frequency, and walk your own story path."
                </p>
            </div>

            // The Slide Engine
            <div class="relative min-h-[400px]">
                <div class="space-y-8">
                        {move || {
                            if is_graph_loading.get() {
                                view! {
                                    <div class="flex flex-col items-center justify-center py-20 gap-4">
                                        <LoadingSpinner message="Whispering portal codes...".to_string() size="lg".to_string()/>
                                    </div>
                                }.into_any()
                            } else if story_graph.get().is_none() {
                                // EMPTY STATE: Call to seed demo adventure
                                view! {
                                    <GlassPanel class="max-w-2xl mx-auto text-center space-y-6">
                                        <div class="w-16 h-16 rounded-full bg-cyan-950/50 border border-cyan-500/30 flex items-center justify-center mx-auto text-cyan-400 text-2xl font-bold animate-pulse">
                                            "?"
                                        </div>
                                        <div class="space-y-2">
                                            <h3 class="text-2xl font-bold text-white">"The Sandbox is Silent"</h3>
                                            <p class="text-sm text-slate-400">
                                                "There are currently no story graphs loaded in the Bevy ECS registry or SQL database. You can author a custom branching curriculum, or instantly seed a premium narrative adventure below."
                                            </p>
                                        </div>
                                        <div class="flex flex-col sm:flex-row items-center justify-center gap-4 pt-2">
                                            <a 
                                                href="/authoring"
                                                class="w-full sm:w-auto px-6 py-3 bg-slate-800 hover:bg-slate-700 border border-white/10 text-white text-sm font-bold tracking-wider uppercase rounded-xl transition-all"
                                            >
                                                "Go to Authoring"
                                            </a>
                                            <button 
                                                on:click=seed_demo_handler
                                                class="w-full sm:w-auto px-6 py-3 bg-cyan-600 hover:bg-cyan-500 text-white text-sm font-bold tracking-wider uppercase rounded-xl transition-all shadow-lg shadow-cyan-600/30"
                                            >
                                                "Seed Demo Adventure"
                                            </button>
                                        </div>
                                    </GlassPanel>
                                }.into_any()
                            } else {
                                // NARRATIVE INTERACTION ACTIVE STATE
                                let node = active_node.get().unwrap();
                                let target_pitch = node.target_freq.unwrap_or(0.0);
                                let has_gate = target_pitch > 0.0;
                                let is_locked = !is_matched.get();
                                
                                view! {
                                    <div class=move || {
                                        let base = "grid grid-cols-1 lg:grid-cols-12 gap-8 items-start transition-all duration-300 ";
                                        if transitioning.get() {
                                            format!("{} opacity-0 translate-y-4 scale-95", base)
                                        } else {
                                            format!("{} opacity-100 translate-y-0 scale-100", base)
                                        }
                                    }>
                                        
                                        // Left Column: Active slide (Story Card)
                                        <div class="lg:col-span-7 space-y-6">
                                            <GlassPanel class="relative overflow-hidden group">
                                                
                                                // Decorative Abstract Glowing Backdrop Gradient matching the concept
                                                <div class="absolute inset-0 z-0 opacity-20 bg-gradient-to-tr from-cyan-600/20 via-purple-600/20 to-black pointer-events-none" />
                                                
                                                // Slide Header
                                                <div class="relative z-10 flex justify-between items-center border-b border-white/5 pb-4">
                                                    <span class="text-[10px] font-extrabold tracking-widest text-cyan-400 uppercase">
                                                        "Thought Sanctuary"
                                                    </span>
                                                    <div class="flex items-center gap-2">
                                                        <span class="w-1.5 h-1.5 bg-green-400 rounded-full animate-pulse" />
                                                        <span class="text-[9px] font-mono text-slate-500 uppercase tracking-wider">
                                                            "Active Node: " {node.id.chars().take(8).collect::<String>()}
                                                        </span>
                                                    </div>
                                                </div>
                                                
                                                // Narrative Details
                                                <div class="relative z-10 py-6 space-y-4">
                                                    <h2 class="text-3xl font-black text-white tracking-tight">
                                                        {node.title.clone()}
                                                    </h2>
                                                    
                                                    // VAAM Semantic Anchor
                                                    <div class="inline-flex items-center gap-2 px-3 py-1 rounded bg-slate-900/80 border border-cyan-500/20 text-cyan-300 text-xs font-bold tracking-wider animate-pulse">
                                                        <span>"Vocabulary Word:"</span>
                                                        <span class="text-white uppercase font-black">{node.subject_word.clone()}</span>
                                                    </div>

                                                    <p class="text-slate-300 text-lg leading-relaxed pt-2 font-light">
                                                        {node.content.clone()}
                                                    </p>
                                                </div>

                                                // Slide Choices Actions (Fades in when Somatic Gate Unlocked)
                                                <div class="relative z-10 border-t border-white/5 pt-6 space-y-4">
                                                    <h4 class="text-[10px] font-extrabold uppercase tracking-widest text-slate-400 mb-2">
                                                        {move || if is_locked { "Choices Locked (Ground yourself to unlock)" } else { "Pathways Open" }}
                                                    </h4>
                                                    
                                                    <div class="flex flex-col gap-3">
                                                        {node.choices.clone().into_iter().map(|choice| {
                                                            let leads = choice.leads_to.clone();
                                                            let word = node.subject_word.clone();
                                                            let cid = choice.id.clone();
                                                            let gid = story_graph.get().as_ref().map(|g| g.id.clone()).unwrap_or_default();
                                                            view! {
                                                                <button
                                                                    disabled=is_locked
                                                                    on:click=move |_| handle_choice_selection(leads.clone(), word.clone(), cid.clone(), gid.clone())
                                                                    class=move || {
                                                                        let base = "w-full text-left p-4 rounded-xl border transition-all duration-300 flex items-center justify-between ";
                                                                        if is_locked {
                                                                            format!("{} bg-white/[0.02] border-white/5 text-slate-500 cursor-not-allowed opacity-50 select-none", base)
                                                                        } else {
                                                                            format!("{} bg-slate-900/60 hover:bg-slate-900 border-cyan-500/30 hover:border-cyan-400 text-white shadow-lg hover:shadow-cyan-500/10 hover:-translate-y-0.5", base)
                                                                        }
                                                                    }
                                                                >
                                                                    <div class="space-y-1 pr-4">
                                                                        <div class="font-extrabold text-sm tracking-wide">
                                                                            {choice.label.clone()}
                                                                        </div>
                                                                        <div class="text-xs text-slate-400">
                                                                            {choice.description.clone()}
                                                                        </div>
                                                                    </div>
                                                                    <div class=move || {
                                                                        if is_locked {
                                                                            "text-slate-600 font-bold"
                                                                        } else {
                                                                            "text-cyan-400 font-bold group-hover:translate-x-1 transition-transform"
                                                                        }
                                                                    }>
                                                                        {move || if is_locked { "🔒" } else { "➔" }}
                                                                    </div>
                                                                </button>
                                                            }
                                                        }).collect_view()}
                                                    </div>
                                                </div>
                                            </GlassPanel>
                                        </div>
                                        
                                        // Right Column: Somatic Gate controls
                                        <div class="lg:col-span-5 space-y-6">
                                            
                                            // The Somatic Gate matching controller
                                            <GlassPanel class="space-y-6 relative overflow-hidden">
                                                <div class="absolute inset-0 bg-cyan-950/10 z-0 pointer-events-none" />
                                                
                                                <div class="relative z-10 flex items-center justify-between border-b border-white/5 pb-4">
                                                    <h3 class="text-sm font-extrabold text-cyan-400 uppercase tracking-widest">
                                                        "Somatic Pitch Gate"
                                                    </h3>
                                                    <span class=move || {
                                                        let base = "px-2 py-0.5 rounded text-[10px] font-bold uppercase tracking-wider ";
                                                        if is_locked {
                                                            format!("{} bg-amber-900/30 border border-amber-500/30 text-amber-400 animate-pulse", base)
                                                        } else {
                                                            format!("{} bg-green-900/30 border border-green-500/30 text-green-400", base)
                                                        }
                                                    }>
                                                        {move || if is_locked { "Gated" } else { "Aligned" }}
                                                    </span>
                                                </div>

                                                // Reference Tone Section
                                                {move || if has_gate {
                                                    view! {
                                                        <div class="relative z-10 bg-slate-950/40 p-4 rounded-xl border border-white/5 flex items-center justify-between">
                                                            <div class="space-y-1">
                                                                <span class="text-[10px] font-extrabold tracking-wider text-slate-500 uppercase block">
                                                                    "Reference Frequency"
                                                                </span>
                                                                <span class="text-xl font-mono text-cyan-300 font-bold">
                                                                    {target_pitch.to_string()} " Hz"
                                                                </span>
                                                            </div>
                                                            <button
                                                                on:click=move |_| {
                                                                    if is_playing_ref.get() {
                                                                        stop_reference_tone();
                                                                        set_is_playing_ref.set(false);
                                                                    } else {
                                                                        play_reference_tone(target_pitch);
                                                                        set_is_playing_ref.set(true);
                                                                    }
                                                                }
                                                                class=move || {
                                                                    let base = "px-4 py-2 rounded-lg text-xs font-extrabold tracking-wider uppercase transition-all duration-200 flex items-center gap-2 ";
                                                                    if is_playing_ref.get() {
                                                                        format!("{} bg-red-900/50 hover:bg-red-950 border border-red-800 text-red-200", base)
                                                                    } else {
                                                                        format!("{} bg-cyan-600 hover:bg-cyan-500 border border-cyan-500/30 text-white shadow-lg shadow-cyan-600/10", base)
                                                                    }
                                                                }
                                                            >
                                                                {move || if is_playing_ref.get() { "■ Mute" } else { "🔊 Play Drone" }}
                                                            </button>
                                                        </div>
                                                    }.into_any()
                                                } else {
                                                    view! {
                                                        <div class="relative z-10 text-center py-4 bg-slate-950/30 rounded-xl border border-white/5 text-slate-400 text-xs">
                                                            "🕊️ This slide is silent. Find peace here."
                                                        </div>
                                                    }.into_any()
                                                }}

                                                // Mic input dials & wave visualizer
                                                {move || if has_gate {
                                                    view! {
                                                        <div class="relative z-10 space-y-6">
                                                            
                                                            // Microphone Enable button
                                                            <div class="flex items-center justify-between bg-slate-950/20 p-3 rounded-lg border border-white/5">
                                                                <span class="text-xs text-slate-300 font-medium">"Vocalizer Input"</span>
                                                                <button
                                                                    on:click=toggle_microphone_handler
                                                                    class=move || {
                                                                        let base = "px-3 py-1.5 rounded-lg text-xs font-bold tracking-wide transition-all ";
                                                                        if is_listening.get() {
                                                                            format!("{} bg-green-600 text-white", base)
                                                                        } else {
                                                                            format!("{} bg-slate-800 text-slate-300 hover:bg-slate-750", base)
                                                                        }
                                                                    }
                                                                >
                                                                    {move || if is_listening.get() { "🎙️ Listening" } else { "🎙️ Turn On Mic" }}
                                                                </button>
                                                            </div>

                                                            // Real-time Audio Wave Visualizer
                                                            {move || if is_listening.get() {
                                                                view! {
                                                                    <div class="space-y-1">
                                                                        <span class="text-[9px] uppercase tracking-wider text-slate-500 block text-center">
                                                                            "Vocal Resonance Wave"
                                                                        </span>
                                                                        <div class="flex items-center justify-center gap-1 h-12 w-full max-w-xs mx-auto">
                                                                            {waveform_heights.get().into_iter().map(|h| {
                                                                                view! {
                                                                                    <div 
                                                                                        class="w-1.5 bg-gradient-to-t from-cyan-600 to-cyan-400 rounded-full transition-all duration-75"
                                                                                        style=format!("height: {}%", h.max(8.0))
                                                                                    />
                                                                                }
                                                                            }).collect_view()}
                                                                        </div>
                                                                    </div>
                                                                }.into_any()
                                                            } else {
                                                                view! { <span/> }.into_any()
                                                            }}

                                                            // Proximity Tuning Needle (Grounding game)
                                                            <div class="space-y-2 bg-slate-950/60 p-4 rounded-xl border border-white/5">
                                                                <div class="flex justify-between items-center text-xs">
                                                                    <span class="text-slate-500 uppercase tracking-wider text-[9px] font-bold">"Vocal Alignment Needle"</span>
                                                                    <span class="text-cyan-400 font-mono text-xs font-semibold">
                                                                        {move || {
                                                                            if is_listening.get() {
                                                                                detected_pitch.get().map(|p| format!("{:.1} Hz", p)).unwrap_or_else(|| "Calibrating...".to_string())
                                                                            } else {
                                                                                "Mic Offline".to_string()
                                                                            }
                                                                        }}
                                                                    </span>
                                                                </div>
                                                                
                                                                // Tracking Line
                                                                <div class="relative h-4 bg-slate-900 rounded border border-white/5 overflow-hidden">
                                                                    // Target Line
                                                                    <div class="absolute top-0 bottom-0 w-0.5 bg-red-500 left-1/2 z-10 shadow-[0_0_8px_red]" />
                                                                    
                                                                    // Voice Position Needle
                                                                    <div 
                                                                        class=move || {
                                                                            let base = "absolute top-0 bottom-0 w-3 rounded-full transition-all duration-100 -ml-1.5 ";
                                                                            if detected_pitch.get().is_some() {
                                                                                format!("{} bg-cyan-400 shadow-[0_0_10px_cyan]", base)
                                                                            } else {
                                                                                format!("{} bg-transparent", base)
                                                                            }
                                                                        }
                                                                        style=move || format!("left: {}%", vocal_match_percentage.get())
                                                                    />
                                                                </div>

                                                                <div class="flex justify-between text-[9px] font-mono text-slate-500 uppercase tracking-wider">
                                                                    <span>"Low (-30Hz)"</span>
                                                                    <span>"Unison"</span>
                                                                    <span>"High (+30Hz)"</span>
                                                                </div>
                                                            </div>

                                                            // Sustain Match Progress
                                                            <div class="space-y-1.5">
                                                                <div class="flex justify-between text-xs font-medium">
                                                                    <span class="text-slate-400">"Somatic Locking Gasket"</span>
                                                                    <span class="text-cyan-300 font-bold">{move || format!("{:.0}%", match_progress.get())}</span>
                                                                </div>
                                                                <div class="h-2 bg-slate-950 rounded-full overflow-hidden border border-white/5">
                                                                    <div 
                                                                        class="h-full bg-gradient-to-r from-cyan-600 via-cyan-400 to-indigo-500 transition-all duration-150 shadow-[0_0_10px_rgba(6,182,212,0.5)]"
                                                                        style=move || format!("width: {}%", match_progress.get())
                                                                    />
                                                                </div>
                                                                <p class="text-[10px] text-slate-500 italic leading-tight text-center">
                                                                    "✨ Ground your posture, take a breath, and sustain a hum at " {target_pitch.to_string()} "Hz for 1.5 seconds to unlock."
                                                                </p>
                                                            </div>
                                                        </div>
                                                    }.into_any()
                                                } else {
                                                    view! { <span/> }.into_any()
                                                }}

                                                // Accessibility Bypass / Control buttons
                                                <div class="relative z-10 flex flex-col gap-2 pt-2">
                                                    {move || if is_locked {
                                                        view! {
                                                            <button
                                                                on:click=force_unlock_gate
                                                                class="w-full py-2.5 rounded-lg bg-slate-800 hover:bg-slate-755 border border-white/10 text-xs font-extrabold tracking-wider uppercase text-cyan-300 hover:text-white transition-all duration-200"
                                                            >
                                                                "Override Gate (Force Unification)"
                                                            </button>
                                                        }.into_any()
                                                    } else {
                                                        view! {
                                                            <div class="text-center py-2 bg-green-950/20 border border-green-500/20 rounded-lg text-green-300 text-xs font-bold uppercase tracking-wider animate-pulse">
                                                                "✓ Somatic Gate Open"
                                                            </div>
                                                        }.into_any()
                                                    }}
                                                </div>
                                        </div>
                                    </div>
                                }.into_any()
                            }
                        }}
                    </div>

            </div>
        </div>
    }
}
