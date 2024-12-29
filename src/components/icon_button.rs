use leptos::prelude::*;

#[component]
pub fn IconButton(href: String, svg: String) -> impl IntoView {
    let on_click = move |_| {
        let window = window();
        let _ = window.open_with_url_and_target(&href, "_blank");
    };

    view! {
        <button
            class="mx-4 mt-2 w-6 h-6 transition duration-200 ease-in-out hover:text-emerald-500"
            on:click=on_click
        >
            <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                stroke-width="1"
                stroke="currentColor"
                class="size-6"
            >
                <path stroke-linecap="round" stroke-linejoin="round" d=svg />
            </svg>
        </button>
    }
}
