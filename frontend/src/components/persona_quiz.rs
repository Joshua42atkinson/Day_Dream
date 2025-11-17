use leptos::*;
use common::Dilemma;

#[component]
pub fn PersonaQuiz() -> impl IntoView {
    let dilemmas = create_resource(|| (), |_| async move {
        match reqwest::get("http://127.0.0.1:3000/api/dilemmas").await {
            Ok(response) => response.json::<Vec<Dilemma>>().await.map_err(|e| e.to_string()),
            Err(e) => Err(e.to_string()),
        }
    });

    view! {
        <div class="max-w-2xl mx-auto p-4">
            <h1 class="text-3xl font-bold mb-4 text-cyan-400">"Discover Your Persona"</h1>
            <p class="text-gray-400 mb-6">"Your answers to the following dilemmas will reveal your primary archetype. This will shape your journey and unlock unique opportunities."</p>

            <Suspense fallback=move || view! { <p>"Loading Quiz..."</p> }>
                {move || match dilemmas.get() {
                    None => view! { <p>"Loading..."</p> }.into_view(),
                    Some(Err(e)) => view! { <p class="text-red-400">"Error: "{e}</p> }.into_view(),
                    Some(Ok(dilemmas_vec)) => view! {
                        <form>
                            <fieldset disabled=true>
                                <ul class="space-y-6">
                                    {dilemmas_vec.into_iter().map(|dilemma| {
                                        view! {
                                            <li class="leptos-panel border border-gray-700 rounded-lg p-4 bg-gray-800 shadow-md">
                                                <h2 class="text-xl font-semibold text-gray-200">{dilemma.title}</h2>
                                                <p class="text-gray-400 my-2">{dilemma.dilemma_text}</p>
                                                <ul class="space-y-2 mt-4">
                                                    {dilemma.choices.into_iter().map(|choice| {
                                                        view! {
                                                            <li>
                                                                <button
                                                                    type="button"
                                                                    class="w-full text-left p-3 rounded-md border border-gray-600 hover:bg-gray-700 text-gray-300"
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
                                    class="mt-6 w-full bg-green-600 text-white font-bold py-3 px-4 rounded-md disabled:bg-gray-500"
                                    disabled=true
                                >
                                    "Reveal My Persona"
                                </button>
                            </fieldset>
                        </form>
                    }.into_view(),
                }}
            </Suspense>
        </div>
    }
}
