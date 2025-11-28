use leptos::prelude::*;
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    *,
};

use crate::pages::ask_pete::AskPete;
use crate::pages::authoring::AuthoringPage;
use crate::pages::not_found::NotFound;
use crate::pages::research_dashboard::ResearchDashboard;

// Train Yard Architecture
use crate::pages::engine_cab_layout::EngineCabLayout;
use crate::pages::knowledge_library::KnowledgeLibrary;
use crate::pages::train_yard_layout::TrainYardLayout;
use crate::pages::weigh_station::WeighStation;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <crate::components::dispatcher_console::DispatcherConsole>
                <Routes fallback=|| "Not Found.">
                    // === TRAIN YARD ARCHITECTURE ===

                    // Instructor Mode: The Train Yard (IDE for curriculum authoring)
                    <ParentRoute path=path!("/yard") view=TrainYardLayout>
                        <Route path=path!("") view=move || view! { <crate::components::authoring::node_canvas::NodeCanvas/> }/>
                        // <Route path=path!("/weigh-station") view=move || view! { <WeighStation/> }/>
                        <Route path=path!("/library") view=move || view! { <KnowledgeLibrary/> }/>
                    </ParentRoute>

                    // Student Mode: The Journey (Immersive learning experience)
                    <Route path=path!("/journey/:quest_id") view=EngineCabLayout/>

                    // === LEGACY ROUTES (Backward Compatibility) ===
                    <Route path=path!("/") view=AskPete/>
                    <Route path=path!("/chat") view=crate::components::ai_mirror_chat::AiMirrorChat/>
                    <Route path=path!("/authoring") view=AuthoringPage/>
                    <Route path=path!("/settings") view=crate::pages::settings::SettingsPage/>

                    // Hidden developer routes
                    <Route path=path!("/research") view=ResearchDashboard/>
                    <Route path=path!("/ai-mirror") view=crate::components::ai_mirror_chat::AiMirrorChat/>
                    <Route path=path!("/ai-test") view=crate::components::ai_test_panel::AiTestPanel/>

                    <Route path=path!("/*any") view=NotFound/>
                </Routes>
            </crate::components::dispatcher_console::DispatcherConsole>
        </Router>
    }
}
