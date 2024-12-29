use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <div class="flex justify-center py-4 w-full fancy-text">
            <div class="px-2 text-lg transition duration-200 ease-in-out hover:text-emerald-500">
                "© Github Template by Matthew McPeak"
            </div>
        </div>
    }
}
