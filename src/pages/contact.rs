use leptos::prelude::*;

use crate::structs::{Config, User};

#[component]
pub fn Contact() -> impl IntoView {
    let user = use_context::<ReadSignal<Option<User>>>()
        .get()
        .unwrap()
        .unwrap();

    let config = use_context::<ReadSignal<Option<Config>>>()
        .get()
        .unwrap()
        .unwrap();

    view! {
    <div
        class="p-2 my-2 mx-4 rounded-lg border shadow-xl transition duration-200 ease-in-out hover:border-emerald-700 border-zinc-800 bg-zinc-700 flex flex-col h-96 justify-center"
    >
        <div class="flex flex-row justify-center">
            <div class="shadow-xl rounded-lg border-2 transition duration-200 ease-in-out hover:border-emerald-700 border-zinc-700">
                <img class="rounded-lg grayscale h-48 w-48" src=user.avatar_url />
            </div>
        </div>
        <p class="text-3xl text-center transition duration-200 ease-in-out hover:text-emerald-500">
            "Matthew McPeak"
        </p>

       <p class="text-xl text-center transition duration-200 ease-in-out hover:text-emerald-500">
            "Full-Stack Developer"
        </p>

        <p class="text-lg text-center transition duration-200 ease-in-out hover:text-emerald-500">
            {config.email}
        </p>
        <p class="text-lg text-center transition duration-200 ease-in-out hover:text-emerald-500">
            {config.phone_number}
        </p>

    </div> }
}
