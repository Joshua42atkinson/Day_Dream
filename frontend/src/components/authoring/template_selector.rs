use common::expert::StoryNode;
use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct TemplateOption {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub icon: &'static str,
    pub passenger_load: u8,
}

#[component]
pub fn TemplateSelector(
    #[prop(into)] on_close: Callback<()>,
    #[prop(into)] on_generate: Callback<Vec<StoryNode>>,
) -> impl IntoView {
    let templates = vec![
        TemplateOption {
            id: "investigation",
            title: "Investigation Quest",
            description:
                "A scientific inquiry structure: Observe -> Hypothesize -> Experiment -> Conclude.",
            icon: "🔍",
            passenger_load: 3,
        },
        TemplateOption {
            id: "historical",
            title: "Historical Journey",
            description: "Travel through time: Context -> Event -> Reaction -> Consequence.",
            icon: "📜",
            passenger_load: 4,
        },
        TemplateOption {
            id: "skills",
            title: "Skills Workshop",
            description: "Procedural mastery: Concept -> Practice -> Challenge -> Mastery.",
            icon: "🛠️",
            passenger_load: 2,
        },
    ];

    let (selected_template, set_selected_template) = signal(templates[0].clone());

    let generate_nodes = move || {
        let template = selected_template.get();
        let mut nodes = Vec::new();
        let start_x = 100.0;
        let start_y = 100.0;
        let spacing = 350.0;

        match template.id {
            "investigation" => {
                nodes.push(create_node(
                    "Observation Deck",
                    "Students observe the phenomenon and record data.",
                    start_x,
                    start_y,
                    2,
                ));
                nodes.push(create_node(
                    "Hypothesis Hub",
                    "Formulate a testable hypothesis based on observations.",
                    start_x + spacing,
                    start_y,
                    3,
                ));
                nodes.push(create_node(
                    "Experiment Lab",
                    "Design and run an experiment to test the hypothesis.",
                    start_x + spacing * 2.0,
                    start_y,
                    4,
                ));
                nodes.push(create_node(
                    "Conclusion Center",
                    "Analyze results and draw conclusions.",
                    start_x + spacing * 3.0,
                    start_y,
                    3,
                ));
            }
            "historical" => {
                nodes.push(create_node(
                    "The Context",
                    "Understand the world before the event.",
                    start_x,
                    start_y,
                    3,
                ));
                nodes.push(create_node(
                    "The Event",
                    "Witness the turning point in history.",
                    start_x + spacing,
                    start_y,
                    2,
                ));
                nodes.push(create_node(
                    "The Reaction",
                    "How did different groups respond?",
                    start_x + spacing * 2.0,
                    start_y,
                    4,
                ));
                nodes.push(create_node(
                    "The Legacy",
                    "Long-term consequences we see today.",
                    start_x + spacing * 3.0,
                    start_y,
                    3,
                ));
            }
            "skills" => {
                nodes.push(create_node(
                    "Concept Intro",
                    "Learn the basic theory.",
                    start_x,
                    start_y,
                    2,
                ));
                nodes.push(create_node(
                    "Guided Practice",
                    "Try it with support.",
                    start_x + spacing,
                    start_y,
                    3,
                ));
                nodes.push(create_node(
                    "Challenge Mode",
                    "Apply without help.",
                    start_x + spacing * 2.0,
                    start_y,
                    4,
                ));
            }
            _ => {}
        }

        on_generate.run(nodes);
        on_close.run(());
    };

    view! {
        <div class="absolute inset-0 bg-black/80 z-50 flex items-center justify-center p-4">
            <div class="bg-slate-900 border-2 border-boilermaker-gold rounded-lg shadow-2xl max-w-4xl w-full flex flex-col max-h-[90vh]">
                // Header
                <div class="p-6 border-b border-white/10 flex justify-between items-center bg-railyard-dark">
                    <div class="flex items-center gap-3">
                        <span class="text-3xl">"🤖"</span>
                        <div>
                            <h2 class="text-2xl font-bold text-white">"Design with Pete"</h2>
                            <p class="text-slate-400">"Select a learning framework to generate a station network"</p>
                        </div>
                    </div>
                    <button
                        class="text-slate-400 hover:text-white text-2xl"
                        on:click=move |_| on_close.run(())
                    >
                        "✕"
                    </button>
                </div>

                // Content
                <div class="flex flex-1 overflow-hidden">
                    // Sidebar (Template List)
                    <div class="w-1/3 border-r border-white/10 overflow-y-auto p-4 space-y-2 bg-slate-950">
                        <For
                            each=move || templates.clone()
                            key=|t| t.id
                            children=move |t| {
                                let t_clone = t.clone();
                                let is_selected = move || selected_template.get().id == t_clone.id;
                                view! {
                                    <button
                                        class=move || format!(
                                            "w-full text-left p-4 rounded transition-all border-l-4 {}",
                                            if is_selected() { "bg-slate-800 border-boilermaker-gold" } else { "bg-transparent border-transparent hover:bg-slate-800/50" }
                                        )
                                        on:click=move |_| set_selected_template.set(t_clone.clone())
                                    >
                                        <div class="text-2xl mb-2">{t.icon}</div>
                                        <div class="font-bold text-white">{t.title}</div>
                                        <div class="text-xs text-slate-500 mt-1">"Avg Load: " {t.passenger_load} "/4"</div>
                                    </button>
                                }
                            }
                        />
                    </div>

                    // Preview Area
                    <div class="w-2/3 p-8 bg-slate-900 flex flex-col">
                        <div class="flex-1">
                            <h3 class="text-xl font-bold text-boilermaker-gold mb-4 flex items-center gap-2">
                                <span>{move || selected_template.get().icon}</span>
                                {move || selected_template.get().title}
                            </h3>
                            <p class="text-slate-300 text-lg mb-6 leading-relaxed">
                                {move || selected_template.get().description}
                            </p>

                            <div class="bg-slate-950 p-6 rounded border border-slate-800">
                                <h4 class="text-sm font-bold text-slate-500 uppercase tracking-wider mb-4">"Generated Stations"</h4>
                                <div class="space-y-4">
                                    {move || match selected_template.get().id {
                                        "investigation" => view! {
                                            <StationPreview title="Observation Deck" load=2 />
                                            <ArrowDown />
                                            <StationPreview title="Hypothesis Hub" load=3 />
                                            <ArrowDown />
                                            <StationPreview title="Experiment Lab" load=4 />
                                            <ArrowDown />
                                            <StationPreview title="Conclusion Center" load=3 />
                                        }.into_any(),
                                        "historical" => view! {
                                            <StationPreview title="The Context" load=3 />
                                            <ArrowDown />
                                            <StationPreview title="The Event" load=2 />
                                            <ArrowDown />
                                            <StationPreview title="The Reaction" load=4 />
                                            <ArrowDown />
                                            <StationPreview title="The Legacy" load=3 />
                                        }.into_any(),
                                        "skills" => view! {
                                            <StationPreview title="Concept Intro" load=2 />
                                            <ArrowDown />
                                            <StationPreview title="Guided Practice" load=3 />
                                            <ArrowDown />
                                            <StationPreview title="Challenge Mode" load=4 />
                                        }.into_any(),
                                        _ => view! {}.into_any()
                                    }}
                                </div>
                            </div>
                        </div>

                        <div class="mt-8 flex justify-end">
                            <button
                                class="px-8 py-3 bg-boilermaker-gold hover:bg-white text-black font-bold rounded shadow-lg text-lg transition-all transform hover:scale-105"
                                on:click=move |_| generate_nodes()
                            >
                                "Generate Route"
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn StationPreview(#[prop(into)] title: String, load: u8) -> impl IntoView {
    let load_color = if load <= 3 {
        "text-signal-green"
    } else {
        "text-signal-yellow"
    };
    view! {
        <div class="flex items-center justify-between bg-slate-900 p-3 rounded border border-slate-700">
            <div class="flex items-center gap-3">
                <span class="text-xl">"🚉"</span>
                <span class="font-bold text-slate-200">{title}</span>
            </div>
            <div class=move || format!("font-mono font-bold {}", load_color)>
                "📦 " {load} "/4"
            </div>
        </div>
    }
}

#[component]
fn ArrowDown() -> impl IntoView {
    view! {
        <div class="flex justify-center">
            <div class="text-slate-600 text-xl">"⬇️"</div>
        </div>
    }
}

fn create_node(title: &str, content: &str, x: f64, y: f64, load: u8) -> StoryNode {
    StoryNode {
        id: uuid::Uuid::new_v4().to_string(),
        title: title.to_string(),
        content: content.to_string(),
        x,
        y,
        passenger_count: load,
        complexity_level: 1,
        learner_profiles: vec![],
        gardens_active: vec![],
        required_stats: std::collections::HashMap::new(),
    }
}
