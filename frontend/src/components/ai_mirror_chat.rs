use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Message {
    speaker: String,
    content: String,
    timestamp: String,
}

#[component]
pub fn AiMirrorChat() -> impl IntoView {
    let (messages, set_messages) = signal(Vec::<Message>::new());
    let (input, set_input) = signal(String::new());
    let (is_loading, set_is_loading) = signal(false);
    let (error, set_error) = signal(None::<String>);

    // No session creation needed for now

    let send_message_action = move || {
        let msg = input.get().trim().to_string();
        if msg.is_empty() {
            return;
        }

        set_is_loading.set(true);
        set_error.set(None);

        // Add user message to UI
        set_messages.update(|msgs| {
            msgs.push(Message {
                speaker: "user".to_string(),
                content: msg.clone(),
                timestamp: get_current_time(),
            });
        });

        set_input.set(String::new());

        spawn_local(async move {
            match send_ai_message(msg).await {
                Ok(response) => {
                    set_messages.update(|msgs| {
                        msgs.push(Message {
                            speaker: "ai".to_string(),
                            content: response,
                            timestamp: get_current_time(),
                        });
                    });
                }
                Err(e) => set_error.set(Some(format!("Error: {}", e))),
            }
            set_is_loading.set(false);
        });
    };

    let new_conversation = move |_| {
        set_messages.set(Vec::new());
        set_input.set(String::new());
        set_error.set(None);
        // No session to reset
    };

    view! {
        <div class="h-[calc(100vh-8rem)] flex flex-col space-y-4 animate-fade-in">
            // Chat Container
            <crate::components::boilermaker::BoilermakerPanel class="flex-grow flex flex-col overflow-hidden bg-industrial-surface/90 backdrop-blur-sm p-0">
                // Header / Toolbar
                <div class="flex items-center justify-between p-4 border-b border-purdue-gold/20 bg-black/20">
                    <div class="flex items-center gap-3">
                        <div class="w-2 h-2 rounded-full bg-purdue-prime animate-pulse shadow-[0_0_10px_#FFD700]"></div>
                        <span class="text-xs font-mono text-purdue-gold uppercase tracking-widest">
                            "Secure Channel // Socratic Mode"
                        </span>
                    </div>
                    <crate::components::boilermaker::MechanicalButton
                        on_click=Callback::new(move |_| new_conversation(()))
                        class="text-xs py-2 px-4"
                    >
                        "Reset Session"
                    </crate::components::boilermaker::MechanicalButton>
                </div>

                // Messages Area
                <div class="flex-grow overflow-y-auto p-6 space-y-6 scrollbar-thin scrollbar-thumb-purdue-gold/20 scrollbar-track-transparent">
                    {move || {
                        let msgs = messages.get();
                        if msgs.is_empty() {
                            view! {
                                <div class="flex flex-col items-center justify-center h-full text-purdue-gold/30 space-y-4">
                                    <div class="text-6xl opacity-20">"⚡"</div>
                                    <p class="text-sm font-mono uppercase tracking-widest">"System Ready"</p>
                                    <p class="text-xs opacity-50">"Awaiting Input..."</p>
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <div class="space-y-6">
                                    {msgs.iter().map(|msg| {
                                        let is_user = msg.speaker == "user";
                                        view! {
                                            <div class={format!("flex {}", if is_user { "justify-end" } else { "justify-start" })}>
                                                <div class={format!(
                                                    "max-w-[80%] p-4 border-l-2 shadow-lg {}",
                                                    if is_user {
                                                        "bg-purdue-gold/10 border-purdue-gold text-steam-white"
                                                    } else {
                                                        "bg-black/40 border-purdue-prime/50 text-purdue-prime"
                                                    }
                                                )}>
                                                    <div class="flex items-center justify-between gap-4 mb-2 opacity-50 text-[10px] font-mono uppercase tracking-wider">
                                                        <span>{if is_user { "Operator" } else { "System" }}</span>
                                                        <span>{msg.timestamp.clone()}</span>
                                                    </div>
                                                    <div class="whitespace-pre-wrap leading-relaxed font-ui text-sm">{msg.content.clone()}</div>
                                                </div>
                                            </div>
                                        }
                                    }).collect::<Vec<_>>()}

                                    {move || is_loading.get().then(|| view! {
                                        <div class="flex justify-start">
                                            <div class="p-4 bg-black/40 border-l-2 border-purdue-prime/50">
                                                <div class="flex items-center space-x-2">
                                                    <div class="w-1.5 h-1.5 bg-purdue-prime animate-bounce"></div>
                                                    <div class="w-1.5 h-1.5 bg-purdue-prime animate-bounce delay-75"></div>
                                                    <div class="w-1.5 h-1.5 bg-purdue-prime animate-bounce delay-150"></div>
                                                </div>
                                            </div>
                                        </div>
                                    })}
                                </div>
                            }.into_any()
                        }
                    }}
                </div>

                // Error Display
                {move || error.get().map(|err| view! {
                    <div class="px-6 py-2 bg-signal-red/10 border-t border-signal-red text-signal-red text-xs font-mono flex items-center gap-2">
                        <span>"⚠️ ERROR:"</span>
                        {err}
                    </div>
                })}

                // Input Area
                <div class="p-4 bg-black/20 border-t border-purdue-gold/20">
                    <div class="flex gap-3">
                        <input
                            type="text"
                            placeholder="Enter command or query..."
                            class="flex-1 px-4 py-3 bg-black/50 border border-purdue-gold/30 text-steam-white placeholder-purdue-gold/30 focus:outline-none focus:border-purdue-prime transition-colors font-mono text-sm"
                            prop:value=move || input.get()
                            on:input=move |ev| set_input.set(event_target_value(&ev))
                            on:keypress=move |ev| {
                                if ev.key() == "Enter" && !is_loading.get() {
                                    send_message_action();
                                }
                            }
                            prop:disabled=move || is_loading.get()
                        />
                        <crate::components::boilermaker::MechanicalButton
                            primary=true
                            on_click=Callback::new(move |_| send_message_action())
                            class="px-8"
                        >
                            {move || if is_loading.get() { "PROCESSING" } else { "TRANSMIT" }}
                        </crate::components::boilermaker::MechanicalButton>
                    </div>
                </div>
            </crate::components::boilermaker::BoilermakerPanel>
        </div>
    }
}

fn get_current_time() -> String {
    // Simple timestamp - in production you'd use proper time formatting
    let now = js_sys::Date::new_0();
    format!("{:02}:{:02}", now.get_hours(), now.get_minutes())
}

async fn send_ai_message(message: String) -> Result<String, String> {
    #[derive(Serialize)]
    struct ChatRequest {
        message: String,
    }

    #[derive(Deserialize)]
    struct ChatResponseWrapper {
        status: String,
        data: Option<SocraticResponseData>,
        message: Option<String>,
    }

    #[derive(Deserialize)]
    struct SocraticResponseData {
        text: String,
    }

    let request_body = ChatRequest { message };

    let response = gloo_net::http::Request::post("http://localhost:3000/api/pete/chat")
        .header("Content-Type", "application/json")
        .json(&request_body)
        .map_err(|e| format!("Failed to serialize request: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.ok() {
        return Err(format!("Server error: {}", response.status()));
    }

    let data: ChatResponseWrapper = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    if data.status == "success" {
        if let Some(inner) = data.data {
            Ok(inner.text)
        } else {
            Err("No data in response".to_string())
        }
    } else {
        Err(data.message.unwrap_or_else(|| "Unknown error".to_string()))
    }
}
