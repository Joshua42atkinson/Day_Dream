// (IMPROVEMENT) This is the new, interactive, multi-page frontend.

use leptos::*;
// (IMPROVEMENT) Import the router components
use leptos_router::*;
// (FIX) Import wasm_bindgen prelude
use wasm_bindgen::prelude::*;


// Import our shared data structures
use crate::components::persona_quiz::PersonaQuiz;
use crate::components::persistent_ui::PersistentUiPanel;
use common::{
    PlayerCharacter, ProfileData, JournalData,
    // (IMPROVEMENT) New structs for interactivity
    PlayerCommand, GameTurn
};
// (IMPROVEMENT) Import console logging

// --- API Fetch Functions ---
// These functions use `reqwest` to call our backend API.
// They are "client-side" functions that run in the browser.

async fn fetch_profile_data() -> Option<ProfileData> {
    let req_url = "/api/profile_data";
    // `gloo_net` is often used, but `reqwest` works well.
    reqwest::get(req_url).await.ok()?.json::<ProfileData>().await.ok()
}

async fn fetch_player_character() -> Option<PlayerCharacter> {
    let req_url = "/api/player_character";
    reqwest::get(req_url).await.ok()?.json::<PlayerCharacter>().await.ok()
}

async fn fetch_journal_data() -> Option<JournalData> {
    let req_url = "/api/journal_data";
    reqwest::get(req_url).await.ok()?.json::<JournalData>().await.ok()
}

// --- (IMPROVEMENT) New API POST Function ---
/// This function sends the player's command to the backend
// (FIX) Specify the error type for ServerFnError
async fn submit_command_to_api(command: PlayerCommand) -> Result<GameTurn, ServerFnError<String>> {
    let client = reqwest::Client::new();
    let res = client
        .post("/api/submit_command")
        .json(&command)
        .send()
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    if !res.status().is_success() {
        return Err(ServerFnError::ServerError("API request failed".to_string()));
    }

    res.json::<GameTurn>()
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}


// --- Main App Component (Router) ---
#[component]
pub fn App() -> impl IntoView {
    // (IMPROVEMENT) Replaced `match` with `leptos_router`
    view! {
        // The <Router> component wraps the entire app
        <Router>
            // The <Nav> component is our "base.html" header
            <Nav />

            <main class="min-h-screen p-4 md:p-8">
                // <Routes> defines all the possible pages
                <Routes>
                    // <Route> maps a URL path to a component
                    <Route path="/" view=ProfileView />
                    <Route path="/game" view=GameView />
                    <Route path="/journal/character" view=JournalCharQuestView />
                    <Route path="/journal/vocab" view=JournalVocabReportView />
                </Routes>
            </main>
        </Router>
    }
}

// --- Navigation Component ---
#[component]
fn Nav() -> impl IntoView {
    view! {
        <header class="bg-gray-800 text-gray-300 p-4 flex justify-between items-center shadow-md">
            <div class="flex items-center gap-4">
                <span class="text-2xl font-bold text-cyan-400">"Daydream"</span>
            </div>
            <nav class="flex gap-4">
                // (IMPROVEMENT) Replaced `on:click` with `<A>`
                // The <A> component automatically updates the URL
                // and triggers the <Router> to change pages.
                <A href="/" class="hover:text-cyan-400">"Profile"</A>
                <A href="/game" class="hover:text-cyan-400">"Game"</A>
                <A href="/journal/character" class="hover:text-cyan-400">"Character"</A>
                <A href="/journal/vocab" class="hover:text-cyan-400">"Journal"</A>
                <a href="#" class="hover:text-cyan-400">"Logout"</a>
            </nav>
        </header>
    }
}

// --- Profile Page Component ---
#[component]
fn ProfileView() -> impl IntoView {
    // This is unchanged from our previous version.
    // It creates a "resource" that automatically calls `fetch_profile_data`
    let profile_data = create_resource(|| (), |_| async { fetch_profile_data().await });

    view! {
        <div class="max-w-4xl mx-auto">
            <h1 class="text-3xl font-bold mb-6">"Your Profile"</h1>
            // <Suspense> shows a fallback UI while the data is loading
            <Suspense fallback=move || view! { <p>"Loading profile..."</p> }>
                {move || profile_data.get().map(|data| {
                    match data {
                        Some(profile) => view! {
                            <div class="leptos-panel border border-gray-700 rounded-lg p-6 mb-6">
                                <p><strong>"Email: "</strong>{profile.email}</p>
                                <p><strong>"Premium Status: "</strong>{if profile.has_premium { "Active" } else { "Inactive" }}</p>
                            </div>

                            <div class="leptos-panel border border-gray-700 rounded-lg p-6 mb-6">
                                <h2 class="text-2xl font-semibold mb-4">"Your Characters"</h2>
                                <ul class="space-y-2">
                                    {profile.characters.into_iter().map(|char| view! {
                                        <li class="flex justify-between items-center">
                                            <span>{char.name} " (" {char.race} " " {char.class_name} ") "</span>
                                            <div>
                                                <button class="bg-cyan-600 hover:bg-cyan-500 text-white py-1 px-3 rounded-md mr-2">"Load"</button>
                                                <button class="bg-red-600 hover:bg-red-500 text-white py-1 px-3 rounded-md">"Delete"</button>
                                            </div>
                                        </li>
                                    }).collect_view()}
                                </ul>
                            </div>

                            <div class="leptos-panel border border-gray-700 rounded-lg p-6">
                                <h2 class="text-2xl font-semibold mb-4">"Create New Character"</h2>
                                <select class="w-full p-2 bg-gray-800 border border-gray-600 rounded-md">
                                    <option>"-- Select a Template --"</option>
                                    {profile.premade_characters.into_iter().map(|premade| view!{
                                        <option value=premade.id>{premade.name} " (" {premade.race_name} " " {premade.class_name} ")"</option>
                                    }).collect_view()}
                                </select>
                            </div>

                            // Add the PersonaQuiz component here
                            <PersonaQuiz />

                        }.into_view(),
                        None => view! { <p class="text-red-400">"Failed to load profile data."</p> }.into_view()
                    }
                })}
            </Suspense>
        </div>
    }
}


// --- (IMPROVEMENT) Game Page Component (Now Interactive!) ---
#[component]
fn GameView() -> impl IntoView {
    // --- 1. State for the Conversation Log ---
    // We create a "signal" to hold the list of narrative messages.
    // A signal is a reactive piece of state.
    let (conversation_log, set_conversation_log) = create_signal(Vec::<String>::new());

    // --- 2. State for the Character Sheet ---
    // We also need a signal to hold the *current* character state,
    // which will be updated by the game logic.
    let (character_state, set_character_state) = create_signal(None::<PlayerCharacter>);

    // --- 3. Resource to Load Initial Data ---
    // This `create_resource` will run once to fetch the *initial* character
    // and set up the game.
    let initial_loader = create_resource(
        || (),
        |_| async { fetch_player_character().await }
    );

    // --- 4. Action to Submit Commands ---
    // This is the "improvement" that handles interactivity.
    // `create_action` wraps our async API call.
    let submit_action = create_action(
        move |input: &String| {
            let input = input.clone();
            // (FIX) This `async move` block is now the single return path
            // for the action closure, resolving the mismatched types error.
            async move {
                // Get the current character state to send with the command
                let char_opt = character_state.get();
                if char_opt.is_none() {
                    // Should not happen if data is loaded
                    return Err(ServerFnError::ServerError("Character not loaded".to_string()));
                }

                // Create the `PlayerCommand` payload
                let command = PlayerCommand {
                    command_text: input.clone(),
                    current_character: char_opt.unwrap(),
                };

                // Call our async API function
                submit_command_to_api(command).await
            }
        }
    );

    // --- 5. Effect to Update UI on Load ---
    // This `create_effect` runs when `initial_loader` finishes.
    // It updates our character state and conversation log.
    create_effect(move |_| {
        if let Some(Some(player)) = initial_loader.get() {
            // Set the initial character state
            set_character_state.set(Some(player.clone()));

            // Set the initial narrative message
            set_conversation_log.set(vec![
                "The wind howls... Ahead, the way is shrouded in mist.".to_string(),
                "What do you do?".to_string()
            ]);
        }
    });

    // --- 6. Effect to Update UI on Action Return ---
    // This `create_effect` runs when `submit_action` finishes.
    create_effect(move |_| {
        if let Some(Ok(game_turn)) = submit_action.value().get() {
            // Update the character state with the new one from the server
            set_character_state.set(Some(game_turn.updated_character.clone()));

            // Update the conversation log
            set_conversation_log.update(|log| {
                // Add the player's command
                log.push(format!("> {}", game_turn.player_command));
                // Add the AI's narrative
                log.push(game_turn.ai_narrative.clone());
                // Add the system message, if one exists
                if let Some(msg) = game_turn.system_message.clone() {
                    log.push(format!("[SYSTEM: {}]", msg));
                }
            });
        }
    });

    // --- 7. The UI (View) ---
    view! {
        // The PersistentUiPanel is fixed, so it will float on the side.
        <PersistentUiPanel />

        // We adjust the main grid to leave space for the sidebar.
        <div class="max-w-4xl mx-auto grid grid-cols-1 md:grid-cols-3 gap-6 pr-80"> // Added pr-80 for sidebar

            // Left Column: Character & Quest Info
            <div class="md:col-span-1 flex flex-col gap-6">
                <section class="leptos-panel border border-gray-700 rounded-lg p-6">
                    <h2 class="text-2xl font-semibold mb-4">"Player Status"</h2>
                    // This <Suspense> now wraps the signal, not the resource
                    <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                        {move || match character_state.get() {
                            Some(player) => view! {
                                <div class="space-y-3">
                                    <div><p class="text-sm text-gray-400">"Name"</p><p class="text-lg font-medium">{player.name}</p></div>
                                    <div><p class="text-sm text-gray-400">"Identity"</p><p class="text-lg">{format!("{} {} ({})", player.race_name, player.class_name, player.philosophy_name)}</p></div>
                                    <div><p class="text-sm text-gray-400">"Location"</p><p class="text-lg font-medium">{player.current_location}</p></div>
                                    <div><p class="text-sm text-gray-400">"Fate Points"</p><p class="text-lg font-medium">{player.fate_points}</p></div>
                                </div>
                            }.into_view(),
                            None => view! { <p class="text-red-400">"Failed to load player data."</p> }.into_view(),
                        }}
                    </Suspense>
                </section>

                <section class="leptos-panel border border-gray-700 rounded-lg p-6">
                    <h2 class="text-2xl font-semibold mb-4">"Current Quest"</h2>
                    <Suspense fallback=move || view! { <p>"..."</p> }>
                        {move || match character_state.get() {
                            Some(player) => view! {
                                <div class="space-y-2">
                                    <h3 class="text-lg font-medium text-cyan-400">{player.current_quest_title}</h3>
                                    <p class="text-gray-300">{player.current_step_description}</p>
                                </div>
                            }.into_view(),
                            None => view! { <p class="text-gray-400">"No quest data."</p> }.into_view(),
                        }}
                    </Suspense>
                </section>
            </div>

            // Right Column: Narrative & Input
            <div class="md:col-span-2">
                <section class="leptos-panel border border-gray-700 rounded-lg p-6">
                    <h2 class="text-2xl font-semibold mb-4">"Narrative"</h2>

                    // (IMPROVEMENT) The narrative log is now reactive
                    <div class="narrative-log bg-gray-800 rounded-md p-4 mb-4 text-gray-300 text-lg leading-relaxed">
                        // We reverse the log so new messages are at the top,
                        // which works with `flex-direction: column-reverse`
                        <For
                            each=move || conversation_log.get().into_iter().rev()
                            key=|msg| msg.clone() // Use the message as its own key (simple)
                            children=move |msg| view! { <p class="mb-2">{msg}</p> }
                        />
                    </div>

                    // (FIX) This is now a standard <form>
                    <form on:submit=move |ev| {
                        ev.prevent_default(); // Stop the browser from reloading
                        let form = ev.target().unwrap().dyn_into::<web_sys::HtmlFormElement>().unwrap();
                        let command = form.get_with_name("command").unwrap().as_string().unwrap();
                        submit_action.dispatch(command); // Dispatch the action
                        form.reset(); // Clear the input field
                    }>
                        <input
                            type="text"
                            name="command"
                            placeholder="Type your command..."
                            class="w-full p-3 text-lg rounded-md bg-gray-900 border border-gray-700 focus:outline-none focus:border-cyan-500 focus:ring-1 focus:ring-cyan-500"
                        />
                    </form>
                </section>
            </div>
        </div>
    }
}


// --- Journal (Character & Quest) Page Component ---
#[component]
fn JournalCharQuestView() -> impl IntoView {
    // This component is unchanged
    let player_character = create_resource(
        || (),
        |_| async { fetch_player_character().await }
    );

    view! {
        <div class="max-w-4xl mx-auto grid grid-cols-1 md:grid-cols-2 gap-6">
            // Left Column: Character Sheet
            <section class="leptos-panel border border-gray-700 rounded-lg p-6">
                <h2 class="text-2xl font-semibold mb-4">"Character Sheet"</h2>
                <Suspense fallback=move || view! { <p>"Loading character..."</p> }>
                    {move || player_character.get().map(|data| {
                        match data {
                            Some(player) => view! {
                                <div class="space-y-3">
                                    <p><strong>"Name: "</strong>{player.name}</p>
                                    <p><strong>"Race: "</strong>{player.race_name}</p>
                                    <p><strong>"Class: "</strong>{player.class_name}</p>
                                    <p><strong>"Philosophy: "</strong>{player.philosophy_name}</p>
                                    <hr class="border-gray-600" />
                                    <p><strong>"Fate Points: "</strong>{player.fate_points}</p>
                                    <hr class="border-gray-600" />
                                    <p><strong>"Boon: "</strong>{player.boon}</p>
                                    <p><strong>"Backstory: "</strong><span class="whitespace-pre-wrap">{player.backstory}</span></p>
                                    <hr class="border-gray-600" />
                                    <div>
                                        <h3 class="text-lg font-medium text-cyan-400 mb-2">"Abilities & Aspects"</h3>
                                        <ul class="list-disc list-inside text-gray-300">
                                            {player.abilities.into_iter().map(|a| view!{ <li>{a}</li> }).collect_view()}
                                            {player.aspects.into_iter().map(|a| view!{ <li class="italic text-gray-400">{a}</li> }).collect_view()}
                                        </ul>
                                    </div>
                                    <div>
                                        <h3 class="text-lg font-medium text-cyan-400 mb-2">"Inventory"</h3>
                                        <ul class="list-disc list-inside text-gray-300">
                                            {player.inventory.into_iter().map(|i| view!{ <li>{i}</li> }).collect_view()}
                                        </ul>
                                    </div>
                                </div>
                            }.into_view(),
                            None => view! { <p class="text-red-400">"Failed to load character."</p> }.into_view(),
                        }
                    })}
                </Suspense>
            </section>

            // Right Column: Quest Log
            <section class="leptos-panel border border-gray-700 rounded-lg p-6">
                <h2 class="text-2xl font-semibold mb-4">"Current Quest"</h2>
                <Suspense fallback=move || view! { <p>"Loading quest..."</p> }>
                    {move || player_character.get().map(|data| {
                        match data {
                            Some(player) => view! {
                                <div class="space-y-2">
                                    <h3 class="text-lg font-medium text-cyan-400">{player.current_quest_title}</h3>
                                    <p class="text-gray-300">{player.current_step_description}</p>
                                </div>
                            }.into_view(),
                            None => view! { <p class="text-gray-400">"No quest data."</p> }.into_view(),
                        }
                    })}
                </Suspense>
            </section>
        </div>
    }
}


// --- Journal (Vocab & Report) Page Component ---
#[component]
fn JournalVocabReportView() -> impl IntoView {
    // This component is unchanged
    let journal_data = create_resource(
        || (),
        |_| async { fetch_journal_data().await }
    );

    view! {
        <div class="max-w-4xl mx-auto grid grid-cols-1 md:grid-cols-2 gap-6">
            // Left Column: Word Journal
            <section class="leptos-panel border border-gray-700 rounded-lg p-6">
                <h2 class="text-2xl font-semibold mb-4">"Word Journal"</h2>
                <Suspense fallback=move || view! { <p>"Loading journal..."</p> }>
                    {move || journal_data.get().map(|data| {
                        match data {
                            Some(journal) => view! {
                                <div class="space-y-4">
                                    <div>
                                        <h3 class="text-lg font-medium text-cyan-400 mb-2">"Academic Word List (AWL)"</h3>
                                        <ul class="list-disc list-inside">
                                            {journal.awl_words.into_iter().map(|w| view! {
                                                <li><strong>{w.word}": "</strong>{w.definition}</li>
                                            }).collect_view()}
                                        </ul>
                                    </div>
                                    <div>
                                        <h3 class="text-lg font-medium text-cyan-400 mb-2">"AI Generated Lists"</h3>
                                        {journal.ai_word_lists.into_iter().map(|(list_name, words)| view! {
                                            <div class="mt-2">
                                                <h4 class="font-semibold text-gray-300">{list_name}</h4>
                                                <ul class="list-disc list-inside ml-4">
                                                    {words.into_iter().map(|w| view! {
                                                        <li><strong>{w.word}": "</strong>{w.definition}</li>
                                                    }).collect_view()}
                                                </ul>
                                            </div>
                                        }).collect_view()}
                                    </div>
                                </div>
                            }.into_view(),
                            None => view! { <p class="text-red-400">"Failed to load journal."</p> }.into_view(),
                        }
                    })}
                </Suspense>
            </section>

            // Right Column: Chapter Reports
            <section class="leptos-panel border border-gray-700 rounded-lg p-6">
                <h2 class="text-2xl font-semibold mb-4">"Chapter Reports"</h2>
                <Suspense fallback=move || view! { <p>"Loading reports..."</p> }>
                    {move || journal_data.get().map(|data| {
                        match data {
                            Some(journal) => view! {
                                <div class="space-y-4">
                                    {journal.report_summaries.into_iter().map(|report| view! {
                                        <div>
                                            <h3 class="text-lg font-medium text-cyan-400">"Chapter "{report.chapter}</h3>
                                            <p class="whitespace-pre-wrap text-gray-300">{report.summary}</p>
                                        </div>
                                    }).collect_view()}
                                </div>
                            }.into_view(),
                            None => view! { <p class="text-gray-400">"No reports found."</p> }.into_view(),
                        }
                    })}
                </Suspense>
            </section>
        </div>
    }
}


// --- Main entry point for Wasm ---
#[wasm_bindgen]
pub fn run_app() {
    // Initialize logging
    _ = console_log::init_with_level(log::Level::Debug);

    // Mount the <App/> component to the `data-leptos-mount` div
    mount_to_body(App);
}