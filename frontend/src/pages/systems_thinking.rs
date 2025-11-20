use leptos::prelude::*;
use leptos::form::ActionForm; // 1. Add this import

#[component]
pub fn SystemsThinkingPage() -> impl IntoView {
    // Assuming you have a server action defined
    let submit_action = ServerAction::<SubmitReflection>::new();

    view! {
        <div class="glass-panel p-6">
            <h1>"Archetype Construction"</h1>

            // 2. Change <ReflectionForm> to <ActionForm>
            <ActionForm action=submit_action class="space-y-4">

                <select name="archetype">
                    <option value="Sage">"The Sage"</option>
                    <option value="Hero">"The Hero"</option>
                </select>

                <button type="submit" class="btn-primary">
                    "Confirm Persona"
                </button>
            </ActionForm>
        </div>
    }
}
