use leptos::*;
use std::collections::HashMap;
use common::{Dilemma, Archetype, QuizSubmission};

// --- API Function ---
/// Submits the quiz and returns the resulting archetype
async fn submit_quiz(answers: HashMap<i32, i32>) -> Result<Archetype, String> {
    let client = reqwest::Client::new();
    let submission = QuizSubmission { answers };

    match client.post("http://127.0.0.1:3000/api/submit_quiz")
        .json(&submission)
        .send()
        .await {
            Ok(response) => {
                if response.status().is_success() {
                    response.json::<Archetype>().await.map_err(|e| e.to_string())
                } else {
                    Err(format!("Server error: {}", response.status()))
                }
            },
            Err(e) => Err(e.to_string()),
        }
}

#[component]
pub fn PersonaQuiz() -> impl IntoView {
    // --- Resources and State ---

    // Resource to fetch the initial list of dilemmas
    let dilemmas = create_resource(|| (), |_| async move {
        match reqwest::get("http://127.0.0.1:3000/api/dilemmas").await {
            Ok(response) => response.json::<Vec<Dilemma>>().await.map_err(|e| e.to_string()),
            Err(e) => Err(e.to_string()),
        }
    });

    // Signal to store the user's selected answers (dilemma_id -> choice_id)
    let (selected_answers, set_selected_answers) = create_signal(HashMap::<i32, i32>::new());

    // Action to handle the quiz submission process
    let submission_action = create_action(|answers: &HashMap<i32, i32>| {
        let answers = answers.clone();
        async move { submit_quiz(answers).await }
    });

    // --- Derived Signals ---

    // A signal that tells us if the form is currently being submitted
    let is_submitting = submission_action.pending();

    // A signal that holds the final result from the server
    let quiz_result = submission_action.value();

    // --- The View ---
    view! {
        <div class="max-w-2xl mx-auto p-4">
            <h1 class="text-3xl font-bold mb-4 text-cyan-400">"Discover Your Persona"</h1>

            // --- Part 1: The Quiz Form ---
            // This view is shown when the quiz result is None (i.e., not yet submitted)
            <Show when=move || quiz_result.get().is_none()>
                <p class="text-gray-400 mb-6">"Your answers to the following dilemmas will reveal your primary archetype. This will shape your journey and unlock unique opportunities."</p>

                <Suspense fallback=move || view! { <p>"Loading Quiz..."</p> }>
                    {move || match dilemmas.get() {
                        None => view! { <p>"Loading..."</p> }.into_view(),
                        Some(Err(e)) => view! { <p class="text-red-400">"Error loading quiz: "{e}</p> }.into_view(),
                        Some(Ok(dilemmas_vec)) => {
                            // We need to know the total number of questions to enable the submit button
                            let total_dilemmas = dilemmas_vec.len();
                            let is_quiz_complete = move || selected_answers.get().len() == total_dilemmas;

                            view! {
                                <form on:submit=move |ev| {
                                    ev.prevent_default();
                                    submission_action.dispatch(selected_answers.get());
                                }>
                                    <fieldset disabled=is_submitting>
                                        <ul class="space-y-6">
                                            {dilemmas_vec.into_iter().map(|dilemma| {
                                                let dilemma_id = dilemma.id;
                                                view! {
                                                    <li class="leptos-panel border border-gray-700 rounded-lg p-4 bg-gray-800 shadow-md">
                                                        <h2 class="text-xl font-semibold text-gray-200">{dilemma.title}</h2>
                                                        <p class="text-gray-400 my-2">{dilemma.dilemma_text}</p>
                                                        <ul class="space-y-2 mt-4">
                                                            {dilemma.choices.into_iter().map(|choice| {
                                                                let choice_id = choice.id;
                                                                // Check if this choice is the selected one for this dilemma
                                                                let is_selected = move || selected_answers.get().get(&dilemma_id) == Some(&choice_id);

                                                                view! {
                                                                    <li>
                                                                        <button
                                                                            type="button"
                                                                            on:click=move |_| {
                                                                                set_selected_answers.update(|answers| {
                                                                                    answers.insert(dilemma_id, choice_id);
                                                                                });
                                                                            }
                                                                            // Dynamically set the class based on selection
                                                                            class="w-full text-left p-3 rounded-md border transition-colors"
                                                                            class:border-cyan-500=is_selected
                                                                            class:bg-cyan-900=is_selected
                                                                            class:text-white=is_selected
                                                                            class:border-gray-600=move || !is_selected()
                                                                            class:hover:bg-gray-700=move || !is_selected()
                                                                            class:text-gray-300=move || !is_selected()
                                                                        >
                                                                            {choice.choice_text}
                                                                        </button>
                                                                    </li>
                                                                }
                                                            }).collect_view()}
                                                        </ul>
                                                    </li>
                                                }
                                            }).collect_view()}
                                        </ul>
                                        <button
                                            type="submit"
                                            class="mt-6 w-full bg-green-600 text-white font-bold py-3 px-4 rounded-md disabled:bg-gray-500 hover:bg-green-500 transition-colors"
                                            disabled=move || !is_quiz_complete() || is_submitting.get()
                                        >
                                            {move || if is_submitting.get() { "Revealing..." } else { "Reveal My Persona" }}
                                        </button>
                                    </fieldset>
                                </form>
                            }.into_view()
                        }
                    }}
                </Suspense>
            </Show>

            // --- Part 2: The Result View ---
            // This view is shown after the submission action completes
            <Show when=move || quiz_result.get().is_some()>
                {move || match quiz_result.get().unwrap() {
                    Ok(archetype) => view! {
                        <div class="leptos-panel border border-cyan-500 rounded-lg p-6 bg-gray-800 shadow-lg text-center">
                            <h2 class="text-2xl font-semibold text-gray-200">"Your Persona is: " <span class="text-cyan-400">{archetype.name}</span></h2>
                            <p class="text-gray-400 mt-4">{archetype.description}</p>
                            // TODO: Add a button to continue to the game or character creation
                        </div>
                    }.into_view(),
                    Err(e) => view! {
                        <div class="leptos-panel border border-red-500 rounded-lg p-6 bg-gray-800 text-center">
                             <p class="text-red-400">"An error occurred: "{e}</p>
                             // TODO: Add a button to try again
                        </div>
                    }.into_view(),
                }}
            </Show>
        </div>
    }
}
