//! Zfirot desktop application entry point.

mod app;
mod components;

fn main() {
    dioxus::launch(app::App);
}
