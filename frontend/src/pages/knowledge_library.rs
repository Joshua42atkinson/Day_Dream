use leptos::prelude::*;

/// Placeholder "Knowledge Library" page for RAG document management
#[component]
pub fn KnowledgeLibrary() -> impl IntoView {
    view! {
        <div class="p-8">
            <h2 class="text-3xl mb-4 text-[#CFB991]">"📚 The Knowledge Library"</h2>
            <p class="text-slate-400 mb-6">
                "Upload course materials (PDFs, textbooks) that Pete will use to answer questions. "
                "Documents are chunked and embedded for semantic search."
            </p>

            <div class="bg-slate-950 border border-[#CFB991]/30 rounded-lg p-6">
                <p class="text-center text-slate-500">
                    "RAG document upload coming in Phase 2..."
                </p>
            </div>
        </div>
    }
}
