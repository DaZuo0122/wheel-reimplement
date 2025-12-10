use crate::ast::{RegexNode, RepeatRange};

#[derive(Debug, Clone, PartialEq)]
pub enum NFAState {
    Start,
    Match,
    Transition(char, usize),
    EpsilonTransition(usize),
}

#[derive(Debug, Clone)]
pub struct NFA {
    pub states: Vec<NFAState>,
    pub start: usize,
    pub accept: usize,
}

impl NFA {
    pub fn new() -> Self {
        Self {
            states: vec![NFAState::Start],
            start: 0,
            accept: 0,
        }
    }

    pub fn from_regex(node: &RegexNode) -> Self {
        let mut nfa = Self::new();
        nfa.build_from_node(node, 0);
        nfa
    }

    fn build_from_node(&mut self, node: &RegexNode, current_state: usize) -> usize {
        match node {
            RegexNode::Char(ch) => {
                let new_state = self.states.len();
                self.states[current_state] = NFAState::Transition(*ch, new_state);
                self.states.push(NFAState::Match);
                new_state
            }
            RegexNode::AnyChar => {
                // Simplified: match any character
                let new_state = self.states.len();
                self.states[current_state] = NFAState::Transition('\0', new_state); // \0 means "any"
                self.states.push(NFAState::Match);
                new_state
            }
            RegexNode::Concat(nodes) => {
                let mut current = current_state;
                for node in nodes {
                    current = self.build_from_node(node, current);
                }
                current
            }
            RegexNode::Alternation(nodes) => {
                let start_state = self.states.len();
                self.states.push(NFAState::Start);

                let mut accept_state = None;
                for node in nodes {
                    let branch_accept = self.build_from_node(node, start_state);
                    if accept_state.is_none() {
                        accept_state = Some(branch_accept);
                    } else {
                        // Merge accept states
                        self.states[branch_accept] = NFAState::Match;
                    }
                }

                if let Some(accept) = accept_state {
                    accept
                } else {
                    start_state
                }
            }
            RegexNode::Star(node) => {
                let start_state = self.states.len();
                self.states.push(NFAState::Start);

                let branch_accept = self.build_from_node(node, start_state);
                self.states[branch_accept] = NFAState::EpsilonTransition(start_state);

                start_state
            }
            RegexNode::Plus(node) => {
                let start_state = self.states.len();
                self.states.push(NFAState::Start);

                let branch_accept = self.build_from_node(node, start_state);
                self.states[branch_accept] = NFAState::EpsilonTransition(start_state);

                start_state
            }
            RegexNode::Question(node) => {
                let start_state = self.states.len();
                self.states.push(NFAState::Start);

                let branch_accept = self.build_from_node(node, start_state);
                self.states[branch_accept] = NFAState::Match;

                start_state
            }
            RegexNode::Repeat(node, range) => {
                // Simplified repetition
                let mut current = current_state;

                // Minimum repetitions
                for _ in 0..range.min {
                    current = self.build_from_node(node, current);
                }

                // Optional extra repetitions
                if let Some(max) = range.max {
                    for _ in range.min..max {
                        current = self.build_from_node(node, current);
                    }
                } else {
                    // Handle unbounded repetition (simplified)
                    // In a full implementation, you'd add epsilon transitions for repetition
                }

                current
            }
            RegexNode::Group(node) => self.build_from_node(node, current_state),
            RegexNode::Digit => {
                // Simplified: match digits 0-9
                let new_state = self.states.len();
                self.states[current_state] = NFAState::Transition('\0', new_state);
                self.states.push(NFAState::Match);
                new_state
            }
            RegexNode::WordChar => {
                // Simplified: match word characters [a-zA-Z0-9_]
                let new_state = self.states.len();
                self.states[current_state] = NFAState::Transition('\0', new_state);
                self.states.push(NFAState::Match);
                new_state
            }
            RegexNode::Whitespace => {
                // Simplified: match whitespace
                let new_state = self.states.len();
                self.states[current_state] = NFAState::Transition('\0', new_state);
                self.states.push(NFAState::Match);
                new_state
            }
            RegexNode::StartLine
            | RegexNode::EndLine
            | RegexNode::StartInput
            | RegexNode::EndInput
            | RegexNode::WordBoundary => {
                // Simplified handling of anchors
                let new_state = self.states.len();
                self.states[current_state] = NFAState::Transition('\0', new_state);
                self.states.push(NFAState::Match);
                new_state
            }
        }
    }

    pub fn matches(&self, input: &str) -> bool {
        let mut current_states = vec![self.start];

        for ch in input.chars() {
            let mut next_states = Vec::new();

            for &state in &current_states {
                if let Some(new_states) = self.transition(state, ch) {
                    next_states.extend(new_states);
                }
            }

            if next_states.is_empty() {
                return false;
            }

            current_states = next_states;
        }

        current_states.contains(&self.accept)
    }

    fn transition(&self, state: usize, ch: char) -> Option<Vec<usize>> {
        match &self.states[state] {
            NFAState::Transition(expected, next) if *expected == '\0' || *expected == ch => {
                Some(vec![*next])
            }
            NFAState::EpsilonTransition(next) => Some(vec![*next]),
            _ => None,
        }
    }
}
