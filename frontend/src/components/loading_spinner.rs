use leptos::prelude::*;

#[component]
pub fn LoadingSpinner(
    /// Message to display below the spinner
    #[prop(into)]
    message: String,
    /// Size of the spinner (sm, md, lg)
    #[prop(into)]
    size: String,
) -> impl IntoView {
    let (spinner_size, message_class) = match size.as_str() {
        "sm" => ("w-8 h-8", "text-xs"),
        "lg" => ("w-20 h-20", "text-lg"),
        _ => ("w-12 h-12", "text-sm"), // md is default
    };

    view! {
        <div class="flex flex-col items-center justify-center gap-4">
            <div class=format!("relative {}", spinner_size)>
                // Outer chamfered ring (static)
                <div class="absolute inset-0 chamfered-corners border-2 border-purdue-gold/30"></div>

                // Spinning inner gear
                <div class="absolute inset-1 chamfered-corners border-2 border-transparent border-t-purdue-gold border-r-dust-gold animate-spin"></div>

                // Center glow
                <div class="absolute inset-3 chamfered-corners bg-gradient-to-br from-purdue-gold/20 to-aged-gold/20 animate-pulse-gold"></div>

                // Corner accents
                <div class="absolute top-0 left-0 w-1 h-1 bg-purdue-prime"></div>
                <div class="absolute top-0 right-0 w-1 h-1 bg-purdue-prime"></div>
                <div class="absolute bottom-0 left-0 w-1 h-1 bg-purdue-prime"></div>
                <div class="absolute bottom-0 right-0 w-1 h-1 bg-purdue-prime"></div>
            </div>

            <p class=format!("text-purdue-gold font-mono font-bold uppercase tracking-wider {}", message_class)>
                {message}
            </p>
        </div>
    }
}
