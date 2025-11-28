use crate::components::weigh_station::WeighStation as WeighStationComponent;
use leptos::prelude::*;

/// "Weigh Station" page wrapper
#[component]
pub fn WeighStation() -> impl IntoView {
    view! {
        <WeighStationComponent />
    }
}
