use leptos::*;
use common::{Dilemma, QuizSubmission, PlayerCharacter};
use std::collections::HashMap;

#[component]
pub fn PersonaQuiz() -> impl IntoView {
    // Resource to fetch the initial quiz data
    let dilemmas = create_resource(|| (), |_| async move {
        match reqwest::get("http://1227.0.0.1:3000/api/dilemmas").await {
            Ok(response) => response.json::<Vec<Dilemma>>().await.map_err(|e| e.to_string()),
            Err(e) => Err(e.to_string()),
        }
    });

    // Signal to store the user's answers
    let (answers, set_answers) = create_signal(HashMap::<i32, i32>::new());

    // Action to submit the quiz
    let submit_action = create_action(|submission: &QuizSubmission| {
        let submission = submission.clone();
        async move {
            let client = reqwest::Client::new();
            match client.post("http://127.0.0.1:3000/api/submit_quiz")
                .json(&submission)
                .send()
                .await {
                Ok(response) => response.json::<PlayerCharacter>().await.map_err(|e| e.to_string()),
                Err(e) => Err(e.to_string()),
            }
        }
    });

    let result = submit_action.value();

    view! {
        <div>
            <h1>"Persona Quiz"</h1>
            <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                {move || {
                    let submitted = result.get().is_some();
                    if submitted {
                        match result.get().unwrap() {
                            Ok(player) => view! {
                                <div class="leptos-panel border border-green-700 rounded-lg p-6">
                                    <h2 class="text-2xl font-semibold mb-4">"Your Persona has been Revealed!"</h2>
                                    <p class="text-lg">"Your primary archetype is: "{player.primary_archetype_id.unwrap_or(0)}</p>
                                </div>
                            }.into_view(),
                            Err(e) => view! { <p class="text-red-400">"Error submitting quiz: "{e}</p> }.into_view(),
                        }
                    } else {
                        dilemmas.get().map(|dilemmas_result| {
                            match dilemmas_result {
                                Ok(dilemmas_vec) => {
                                    let total_dilemmas = dilemmas_vec.len();
                                    view! {
                                        <form on:submit=|ev| {
                                            ev.prevent_default();
                                            let submission = QuizSubmission { answers: answers.get() };
                                            submit_action.dispatch(submission);
                                        }>
                                            <ul class="space-y-6">
                                                {dilemmas_vec.into_iter().map(|dilemma| {
                                                    let dilemma_id = dilemma.id;
                                                    view! {
                                                        <li class="leptos-panel border border-gray-700 rounded-lg p-4">
                                                            <h2 class="text-xl font-semibold">{dilemma.title}</h2>
                                                            <p class="text-gray-300 mb-3">{dilemma.dilemma_text}</p>
                                                            <ul class="space-y-2">
                                                                {dilemma.choices.into_iter().map(|choice| {
                                                                    let choice_id = choice.id;
                                                                    let is_selected = move || answers.get().get(&dilemma_id) == Some(&choice_id);
                                                                    view! {
                                                                        <li>
                                                                            <button
                                                                                type="button"
                                                                                on:click=move |_| {
                                                                                    set_answers.update(|ans| {
                                                                                        ans.insert(dilemma_id, choice_id);
                                                                                    });
                                                                                }
                                                                                class="w-full text-left p-3 rounded-md border"
                                                                                class:border-cyan-500=is_selected
                                                                                class:bg-cyan-900=is_selected
                                                                                class:border-gray-600=move || !is_selected()
                                                                                class:hover:bg-gray-700=move || !is_selected()
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
                                                class="mt-6 w-full bg-green-600 hover:bg-green-500 text-white font-bold py-3 px-4 rounded-md disabled:bg-gray-500"
                                                disabled=move || answers.get().len() != total_dilemmas
                                            >
                                                "Reveal My Persona"
                                            </button>
                                        </form>
                                    }.into_view()
                                },
                                Err(e) => view! { <p class="text-red-400">"Error: "{e}</p> }.into_view()
                            }
                        })
                    }
                }}
            </Suspense>
        </div>
    }
}
