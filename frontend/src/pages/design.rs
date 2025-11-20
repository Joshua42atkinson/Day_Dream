use leptos::*;
use crate::components::visual_design::VisualDesignChallenge;

#[component]
pub fn Design() -> impl IntoView {
    view! {
        <div class="container mx-auto py-8">
            <VisualDesignChallenge />
        </div>
    }
}