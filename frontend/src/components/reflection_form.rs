use leptos::*;

#[cfg(feature = "ssr")]
use crate::server_functions::save_reflection;

#[cfg(feature = "ssr")]
#[component]
pub fn ReflectionForm() -> impl IntoView {
    let save_reflection_action = create_server_action::<SaveReflection>();
    let pending = save_reflection_action.pending();

    view! {
        <div class="bg-gray-800 bg-opacity-70 p-6 rounded-lg shadow-lg backdrop-blur-sm border border-gray-700">
            <h3 class="text-xl font-semibold text-pink-400 mb-4">Submit Your Reflection</h3>
            <ActionForm action=save_reflection_action>
                <textarea
                    name="content"
                    rows="4"
                    class="w-full bg-gray-900 text-gray-300 border border-gray-600 rounded-md p-2 focus:ring-2 focus:ring-teal-400 focus:border-teal-400 transition"
                    placeholder="Reflect on how this concept applies to your own projects..."
                ></textarea>
                <button
                    type="submit"
                    disabled=pending
                    class="mt-4 px-4 py-2 bg-teal-500 text-white font-semibold rounded-md hover:bg-teal-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-gray-900 focus:ring-teal-400 disabled:bg-gray-600 disabled:cursor-not-allowed transition"
                >
                    {move || if pending.get() { "Submitting..." } else { "Submit" }}
                </button>
            </ActionForm>
        </div>
    }
}

#[cfg(not(feature = "ssr"))]
#[component]
pub fn ReflectionForm() -> impl IntoView {
    // This component is only rendered on the server.
    // Return a placeholder or an empty view for the client.
    view! { <></> }
}

#[server(SaveReflection, "/api")]
pub async fn save_reflection(content: String) -> Result<(), ServerFnError> {
    // Here you would typically save the reflection to a database.
    // For this example, we'll just print it to the server's console.
    println!("Received reflection: {}", content);
    Ok(())
}
