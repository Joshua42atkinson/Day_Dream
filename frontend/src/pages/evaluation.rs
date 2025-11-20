use leptos::*;
use crate::components::formative_eval::FormativeEvalChallenge;
#[component]
pub fn Evaluation() -> impl IntoView {
view! {
<div class="max-w-5xl mx-auto w-full space-y-20 animate-fade-in p-6">
<div class="text-center space-y-6">
<h1 class="text-4xl font-bold text-white">"Evaluation & Implementation"</h1>
<p class="text-slate-400">"Formative/summative evaluation and implementation planning."</p>
</div>
<FormativeEvalChallenge />
</div>
}
}