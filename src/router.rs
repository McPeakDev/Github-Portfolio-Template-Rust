use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router as LeptosRouter, Routes as LeptoRoutes},
    path,
};

use crate::pages::*;

#[component]
pub fn Router() -> impl IntoView {
    view! {
        <div class="flex-grow regular-text">
            <div class="flex flex-col justify-center h-full">
                <LeptosRouter>
                    <LeptoRoutes fallback=|| "Page not found.">
                        <Route path=path!("/") view=About />
                        <Route path=path!("/projects") view=Projects />
                    </LeptoRoutes>
                </LeptosRouter>
            </div>
        </div>
    }
}
