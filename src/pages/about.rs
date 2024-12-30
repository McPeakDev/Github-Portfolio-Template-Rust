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
                <div class="flex flex-row flex-grow justify-center rounded-lg border-2 shadow-xl transition duration-200 ease-in-out hover:border-emerald-700 basis-1/3 border-zinc-800">
                    <img class="rounded-lg grayscale" src=user.avatar_url />
                </div>
                <div class="flex flex-col justify-center p-5 basis-full lg:basis-2/3">
                    <p class="text-3xl transition duration-200 ease-in-out hover:text-emerald-500 fancy-text">
                        {user.bio}" "{config.extended_bio}
                    </p>
                </div>
            </div>
        </div>
    }
}
