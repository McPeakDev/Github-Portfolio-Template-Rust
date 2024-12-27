use leptos::prelude::*;

use crate::structs::{Config, User};

#[component]
pub fn About() -> impl IntoView {
    let user = use_context::<ReadSignal<Option<User>>>()
        .get()
        .unwrap()
        .unwrap();

    let config = use_context::<ReadSignal<Option<Config>>>()
        .get()
        .unwrap()
        .unwrap();

    view! {
        <div class="flex flex-col px-8 w-full">
            <div class="flex flex-row">
                <img class="rounded-lg grayscale drop-shadow-lg" src=user.avatar_url />
                <div class="flex flex-col justify-center pl-10">
                    <p class="text-3xl fancy-text drop-shadow-lg">
                        {user.bio} {config.extended_bio}
                    </p>
                </div>
            </div>
        </div>
    }
}
