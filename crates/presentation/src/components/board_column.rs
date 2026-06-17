use dioxus::prelude::*;
use domain::{AgentRef, Slice, SliceState};

use super::SliceCard;

/// A single board column for one [`SliceState`], listing its Slices.
///
/// `highlighted` is the shared "highlighted issue" the whole board coordinates;
/// each card highlights itself when it matches and re-emits hover intents via
/// `on_highlight`. `agents` are the board's Assignable Agents handed to each
/// Ready card's adaptive Agent action; `delegating` is the issue number whose
/// delegate is currently in flight.
#[component]
pub fn BoardColumn(
    state: SliceState,
    label: String,
    badge_class: String,
    slices: Vec<Slice>,
    agents: Vec<AgentRef>,
    on_assign: EventHandler<u64>,
    on_assign_agent: EventHandler<(u64, AgentRef)>,
    delegating: Option<u64>,
    highlighted: Option<u64>,
    on_highlight: EventHandler<Option<u64>>,
) -> Element {
    rsx! {
        div { class: "bg-base-100 rounded-box p-3",
            div { class: "flex items-center justify-between mb-3",
                h2 { class: "font-semibold", "{label}" }
                span { class: "badge {badge_class}", "{slices.len()}" }
            }
            div { class: "flex flex-col gap-2",
                for slice in slices {
                    SliceCard {
                        key: "{slice.number}",
                        slice: slice.clone(),
                        agents: agents.clone(),
                        on_assign,
                        on_assign_agent,
                        delegating,
                        highlighted,
                        on_highlight,
                    }
                }
            }
        }
    }
}
