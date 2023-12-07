mod app;
mod nav;
mod profile;

use leptos::*;

use crate::app::App;

fn main() {
    mount_to_body(|| view! {<App/>})
}
