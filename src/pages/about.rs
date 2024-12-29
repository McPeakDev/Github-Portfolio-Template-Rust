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
            <div class="flex flex-row flex-wrap justify-center">
                <div class="flex flex-row justify-center shadow-xl rounded-lg basis-1/3 border-2 transition duration-200 ease-in-out hover:border-emerald-700 border-zinc-800 flex-grow">
                    <img class="rounded-lg grayscale" src=user.avatar_url />
                </div>
                <div class="flex flex-col justify-center p-5 lg:basis-2/3 basis-full">
                    <p class="text-3xl fancy-text transition duration-200 ease-in-out hover:text-emerald-500">
                        {user.bio} {config.extended_bio}
                    </p>
                </div>
            </div>
        </div>
    }
}
