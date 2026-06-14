//! Root component: loads the board (via a fake port for now) and renders it.

use application::BoardService;
use dioxus::prelude::*;
use domain::{RepoRef, Slice, SliceState};
use infrastructure::FakeGitHubPort;

use crate::components::{state_badge_class, state_label, BoardColumn};

/// daisyUI + Tailwind + Iconify, loaded into the webview head. Sourced from CDNs
/// for the walking skeleton; a local Tailwind/daisyUI build is a later slice.
const TAILWIND_CDN: &str = "https://cdn.tailwindcss.com";
const DAISYUI_CDN: &str = "https://cdn.jsdelivr.net/npm/daisyui@4.12.10/dist/full.min.css";
const ICONIFY_CDN: &str = "https://code.iconify.design/iconify-icon/2.1.0/iconify-icon.min.js";

#[component]
pub fn App() -> Element {
    let board = use_resource(|| async {
        let service = BoardService::new(FakeGitHubPort);
        let repo = RepoRef::new("funkode-io", "zfirot");
        service.load_board(&repo).await
    });

    rsx! {
        document::Title { "Zfirot" }
        document::Script { src: TAILWIND_CDN }
        document::Link { rel: "stylesheet", href: DAISYUI_CDN }
        document::Script { src: ICONIFY_CDN }

        div { class: "min-h-screen bg-base-200 p-6",
            header { class: "flex items-center gap-2 mb-6",
                div { dangerous_inner_html: r#"<iconify-icon icon="mdi:view-dashboard-outline" width="28" height="28"></iconify-icon>"# }
                h1 { class: "text-2xl font-bold", "Zfirot" }
            }

            match &*board.read_unchecked() {
                Some(Ok(slices)) => rsx! {
                    Board { slices: slices.clone() }
                },
                Some(Err(error)) => rsx! {
                    div { class: "alert alert-error", "{error}" }
                },
                None => rsx! {
                    span { class: "loading loading-spinner loading-lg" }
                },
            }
        }
    }
}

#[component]
fn Board(slices: Vec<Slice>) -> Element {
    rsx! {
        div { class: "grid grid-cols-1 md:grid-cols-3 gap-4",
            for state in SliceState::ALL {
                BoardColumn {
                    state,
                    label: state_label(state).to_string(),
                    badge_class: state_badge_class(state).to_string(),
                    slices: slices.iter().filter(|s| s.state == state).cloned().collect::<Vec<_>>(),
                    on_assign: move |_number| {} // Assign-self is wired in a later slice. No-op for now.,
                }
            }
        }
    }
}
