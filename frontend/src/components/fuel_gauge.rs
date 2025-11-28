use leptos::prelude::*;

#[component]
pub fn FuelGauge(
    /// The current "Steam Pressure" (0.0 to 1.0)
    #[prop(into)]
    pressure: Signal<f32>,
) -> impl IntoView {
    view! {
        <div class="relative w-full h-4 bg-slate-900 rounded-full overflow-hidden border border-slate-600 shadow-inner">
            // The Pressure Bar
            <div
                class="h-full transition-all duration-500 ease-out relative"
                style=move || format!("width: {}%", pressure.get() * 100.0)
            >
                // Dynamic Color based on Load (Green -> Yellow -> Red)
                <div class=move || {
                    let p = pressure.get();
                    let color = if p < 0.5 { "bg-green-500" }
                               else if p < 0.8 { "bg-yellow-500" }
                               else { "bg-red-600 animate-pulse" }; // Danger Zone!
                    format!("absolute inset-0 {}", color)
                }/>
            </div>

            // Ticks / Grid Lines
            <div class="absolute inset-0 flex justify-between px-2">
                <div class="w-px h-full bg-slate-800/50"></div>
                <div class="w-px h-full bg-slate-800/50"></div>
                <div class="w-px h-full bg-slate-800/50"></div>
                <div class="w-px h-full bg-slate-800/50"></div>
            </div>

            // Label
            <div class="absolute inset-0 flex items-center justify-center text-[10px] font-mono text-white/80 drop-shadow-md">
                "STEAM PRESSURE (COGNITIVE LOAD)"
            </div>
        </div>
    }
}
