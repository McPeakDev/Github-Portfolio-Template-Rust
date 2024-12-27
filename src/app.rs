use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_meta::*;

use crate::components::*;
use crate::router::*;
use crate::structs::Config;
use crate::structs::User;
use crate::utilities::API;

#[component]
pub fn App() -> impl IntoView {
    let api = API::new();

    let (user_option, set_user) = signal::<Option<User>>(None);
    let (config_option, set_config) = signal::<Option<Config>>(None);

    let api_clone = api.clone();

    spawn_local(async move {
        let config = api_clone.get_github_username().await;

        if let Ok(config) = config {
            *set_config.write() = Some(config.clone());

            let user = api_clone.get_github_user(config.github_username).await;

            if let Ok(user) = user {
                *set_user.write() = Some(user.clone());
            }
        }
    });

    let user = move || user_option.get();

    provide_meta_context();

    provide_context(api);
    provide_context(user_option);
    provide_context(config_option);

    move || {
        if user().is_some() {
            view! {
                <div class="flex flex-col mr-2 ml-4 h-screen">
                    <Header />
                    <Router />
                    <Footer />
                </div>
            }
            .into_any()
        } else {
            view! {}.into_any()
        }
    }
}
