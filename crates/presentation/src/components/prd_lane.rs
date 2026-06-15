use dioxus::prelude::*;
use domain::{PrdRef, Slice, SliceState};

use super::{state_badge_class, state_label, BoardColumn};

/// One swimlane: a PRD header above the Ready / WIP / Blocked columns holding
/// that PRD's Slices. A lane with no PRD (`prd` is `None`) renders a plain
/// "No PRD" header for issues that are not linked to any PRD.
#[component]
pub fn PrdLane(prd: Option<PrdRef>, slices: Vec<Slice>, on_assign: EventHandler<u64>) -> Element {
    rsx! {
        section { class: "bg-base-200 rounded-box p-4",
            div { class: "mb-3",
                match prd {
                    Some(prd) => rsx! {
                        a { class: "link link-hover font-semibold", href: "{prd.url}", "#{prd.number} {prd.title}" }
                    },
                    None => rsx! {
                        span { class: "font-semibold opacity-70", "No PRD" }
                    },
                }
            }
            div { class: "grid grid-cols-1 md:grid-cols-3 gap-4",
                for state in SliceState::BOARD {
                    BoardColumn {
                        state,
                        label: state_label(state).to_string(),
                        badge_class: state_badge_class(state).to_string(),
                        slices: slices.iter().filter(|s| s.state == state).cloned().collect::<Vec<_>>(),
                        on_assign: move |number| on_assign.call(number),
                    }
                }
            }
        }
    }
}
