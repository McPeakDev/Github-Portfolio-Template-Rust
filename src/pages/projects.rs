use leptos::{prelude::*, task::spawn_local};

use crate::{
    components::Project as ProjectComponent,
    structs::{Config, Project},
    utilities::API,
};

#[component]
pub fn Projects() -> impl IntoView {
    let api = use_context::<API>().unwrap();
    let config = use_context::<ReadSignal<Option<Config>>>()
        .get()
        .unwrap()
        .unwrap();

    let (projects_ref, set_projects) = signal::<Vec<Project>>(vec![]);

    spawn_local(async move {
        let projects_result = api.get_github_repos(config.github_username).await;

        if let Ok(projects) = projects_result {
            *set_projects.write() = projects
                .into_iter()
                .filter(|project| project.fork == false && project.language.is_some())
                .collect::<Vec<Project>>();
        }
    });

    let projects_view = move || {
        let projects = projects_ref.get();
        if projects.is_empty() {
            view! {}.into_any()
        } else {
            projects
                .iter()
                .map(|project| view! { <ProjectComponent project=project.clone() /> })
                .collect_view()
                .into_any()
        }
    };

    view! { <p>{projects_view}</p> }
}
