use crate::structs::Project;
use chrono::prelude::*;
use leptos::prelude::*;

#[component]
pub fn Project(project: Project) -> impl IntoView {
    let on_click = move |_| {
        let window = window();
        let _ = window.open_with_url_and_target(&project.html_url, "_blank");
    };

    let published_date = DateTime::parse_from_rfc3339(&project.created_at)
        .unwrap()
        .format("%b %e %Y")
        .to_string();

    view! {
        <div
            on:click=on_click
            class="p-2 my-2 mx-4 rounded-lg border shadow-xl transition duration-200 ease-in-out cursor-pointer hover:border-emerald-700 border-zinc-800 bg-zinc-700 basis-1/2"
        >
            <p class="transition duration-200 ease-in-out hover:text-emerald-500">
                "Name: "{project.name}
            </p>
            <p class="transition duration-200 ease-in-out hover:text-emerald-500">
                "Language: " {project.language}
            </p>

            <p class="transition duration-200 ease-in-out hover:text-emerald-500">
                "Publish Date: " {published_date}
            </p>
            <p class="overflow-y-auto break-words transition duration-200 ease-in-out hover:text-emerald-500 h-fit text-wrap">
                {project.description}
            </p>
        </div>
    }
}
