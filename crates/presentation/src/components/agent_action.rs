use domain::AgentRef;

/// The adaptive Agent action a Ready Slice card should render, derived purely
/// from the board's Assignable Agents.
///
/// The shape adapts to how many Agents can take the work:
/// - [`AgentAction::None`] — no Assignable Agent, so the card shows only
///   "Assign me";
/// - [`AgentAction::Single`] — exactly one, rendered as a single button labelled
///   with that Agent's name (e.g. "Assign Copilot");
/// - [`AgentAction::Picker`] — two or more, rendered as a picker so the user
///   chooses which Agent does the work.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentAction {
    None,
    Single(AgentRef),
    Picker(Vec<AgentRef>),
}

/// Derive the adaptive Agent action for a Ready card from the Assignable Agents.
///
/// Pure and offline — the primary test seam for the card's branching, so the UI
/// component itself stays a thin renderer over this decision.
pub fn agent_action(agents: &[AgentRef]) -> AgentAction {
    match agents {
        [] => AgentAction::None,
        [one] => AgentAction::Single(one.clone()),
        many => AgentAction::Picker(many.to_vec()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn agent(name: &str) -> AgentRef {
        AgentRef {
            name: name.to_string(),
            node_id: format!("{name}_id"),
        }
    }

    #[test]
    fn no_agents_is_none() {
        assert_eq!(agent_action(&[]), AgentAction::None);
    }

    #[test]
    fn one_agent_is_single() {
        let copilot = agent("Copilot");

        assert_eq!(
            agent_action(std::slice::from_ref(&copilot)),
            AgentAction::Single(copilot)
        );
    }

    #[test]
    fn two_or_more_agents_is_a_picker_preserving_order() {
        let agents = vec![agent("Copilot"), agent("Devin"), agent("Cursor")];

        assert_eq!(
            agent_action(&agents),
            AgentAction::Picker(agents.clone()),
            "the picker keeps the board's Agent order"
        );
    }
}
