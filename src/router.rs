use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router as LeptosRouter, Routes as LeptoRoutes},
    path,
};

use crate::pages::*;

#[component]
pub fn Router() -> impl IntoView {
    view! {
        <div class="flex-grow fancy-text">
            <div class="flex flex-col justify-center h-full transition duration-200 ease-in-out">
                <LeptosRouter>
                    <LeptoRoutes fallback=|| "Page not found.">
                        <Route path=path!("/") view=About />
                        <Route path=path!("/projects") view=Projects />
                        <Route path=path!("/contact") view=Contact />
                    </LeptoRoutes>
                </LeptosRouter>
            </div>
        </div>
    }
}
