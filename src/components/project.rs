use crate::structs::Project;
use leptos::prelude::*;

#[component]
pub fn Project(project: Project) -> impl IntoView {
    view! { <p>{project.name} " "{project.language}</p> }
}
