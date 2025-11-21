use leptos::*;
use leptos_router::*;
use crate::api::submit_reflection; // Import the ServerFn
use leptos::form::ActionForm;      // CRITICAL IMPORT: The fix for E0425

#[component]
pub fn SystemsThinkingPage() -> impl IntoView {
    // Create the action that links to the ServerFn
    let submit_action = ServerAction::<SubmitReflection>::new();

    // Signals for UI feedback
    let pending = submit_action.pending();
    let value = submit_action.value();

    view! {
        // GlassPanel and AuroraBackground provide the requested aesthetic
        <div class="glass-panel p-6">
            <h1 class="text-2xl text-transparent bg-clip-text bg-gradient-to-r from-indigo-400 to-cyan-400">
                Systems Thinking: Archetype Construction
            </h1>
            <p class="text-gray-300 mb-4">
                "Ethics creates the narrative drive." Choose your path.
            </p>
            // THE FIX: Use ActionForm instead of ReflectionForm
            // The 'action' prop binds this form to the 'submit_reflection' ServerFn
            <ActionForm action=submit_action class="space-y-4">
                // Complex input naming for nested structs in Leptos 0.8
                // If the ServerFn arg is 'data', fields are 'data[field_name]'
                <div>
                    <label class="block text-sm font-medium text-gray-400">Chosen Archetype</label>
                    <select name="data[archetype]" class="w-full bg-gray-800 rounded border border-gray-700 text-white">
                        <option value="Sage">The Sage</option>
                        <option value="Hero">The Hero</option>
                        <option value="Jester">The Jester</option>
                    </select>
                </div>
                <div>
                    <label class="block text-sm font-medium text-gray-400">Primary Virtue</label>
                    <select name="data[virtue_focus]" class="w-full bg-gray-800 rounded border border-gray-700 text-white">
                        <option value="Honesty">Honesty</option>
                        <option value="Compassion">Compassion</option>
                        <option value="Valor">Valor</option>
                    </select>
                </div>
                // Hidden field for the dilemma logic context
                <input type="hidden" name="data[dilemma_choice]" value="A" />
                <button
                    type="submit"
                    class="px-4 py-2 bg-indigo-600 hover:bg-indigo-700 rounded text-white transition-all"
                    disabled=move || pending.get() // Disable while submitting
                >
                    {move || if pending.get() { "Constructing..." } else { "Confirm Persona" }}
                </button>
            </ActionForm>
            // Feedback Display using Result matching
            <div class="mt-4">
                {move || value.get().map(|result| match result {
                    Ok(_) => view! { <span class="text-green-400">"Persona Constructed Successfully."</span> },
                    Err(e) => view! { <span class="text-red-400">{format!("Error: {}", e)}</span> },
                })}
            </div>
        </div>
    }
}
