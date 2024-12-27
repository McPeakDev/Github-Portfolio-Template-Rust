use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <div class="flex justify-center py-4 w-full fancy-text">
            <div class="px-2 text-lg">"© Github Template by Matthew McPeak"</div>
        </div>
    }
}
