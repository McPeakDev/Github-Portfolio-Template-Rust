use leptos::prelude::*;

#[component]
pub fn Link(href: String, name: String) -> impl IntoView {
    view! {
        <a class="mr-6 text-center transition duration-200 ease-in-out hover:text-emerald-500" href=href>
            {{ name }}
        </a>
    }
}
