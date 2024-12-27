mod app;
mod components;
mod pages;
mod router;
mod structs;
mod utilities;

use app::App;
use leptos::mount::mount_to_body;

fn main() {
    mount_to_body(App);
}
