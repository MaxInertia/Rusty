use std::collections::HashMap;

// Deterministic Finite Automata (DFA)
// also known as a Finite-state Machine
pub type StateMachine = State;

#[derive(Debug)]
pub struct State {
    transitions: HashMap<char, State>,
    terminal: bool,
}

impl StateMachine {
    pub fn new() -> Self {
        State {
            transitions: HashMap::new(),
            terminal: false,
        }
    }

    // accepts checks if a word is accepted by this state machine.
    // returns true if word is accepted, else false.
    pub fn accepts(&self, word: &str) -> bool {
        let mut state = self;

        for c in word.chars() {
            match state.trans().get(&c) {
                Some(next_state) => state = next_state,
                None => return false,
            }
        }

        return state.terminal;
    }

    // include modifies the state machine so that it accepts the input word.
    // post-conditions:
    //      - All accepted words prior to applying `include` are accepted after
    //      - This now accepts the input word. (`self.accepts(word)` will return true).
    pub fn include(&mut self, word: &str) {
        match word.chars().nth(0) {
            // Reached end of word, make current state terminal.
            None => self.terminal = true,

            // Characters remaining in word.
            Some(current_char) => {
                let remaining = &word[1..];

                match self.trans_mut().get_mut(&current_char) {
                    // Transition from self on current_char exists.
                    Some(ref mut state) => {
                        state.include(remaining)
                    },
                    // Transition from self on current_char must be created.
                    None => {
                        let mut new_state = State {
                            transitions: HashMap::new(),
                            terminal: remaining.len() == 0,
                        };

                        new_state.include(remaining);
                        self.trans_mut().insert(current_char, new_state);
                    }
                }
            }
        }
    }
}

impl State {
    // trans is a getter for the state transitions that is immutable.
    fn trans(&self) -> &HashMap<char, State> {
        &self.transitions
    }

    // trans_mut is a getter for the state transitions that allows mutation.
    fn trans_mut(&mut self) -> &mut HashMap<char, State> {
        &mut self.transitions
    }
}

#[cfg(test)]
mod tests {
    use super::StateMachine;

    #[test]
    fn sm_include_post_condition_1() {
        let mut sm = StateMachine::new();
        sm.include("abc");
        assert!(sm.accepts("abc"));
    }

    #[test]
    fn sm_include_and_no_accept() {
        let mut sm = StateMachine::new();
        sm.include("abc");
        assert_eq!(sm.accepts(""), false, "word: \"\"");
        assert_eq!(sm.accepts("bc"), false, "word: \"bc\"");
        assert_eq!(sm.accepts("abz"), false, "word: \"abz\"");
        assert_eq!(sm.accepts("abcd"), false, "word: \"abcd\"");
        assert_eq!(sm.accepts("Thor"), false, "word: \"Thor\"");
    }

    #[test]
    fn sm_include_post_condition_2() {
        let mut sm = StateMachine::new();
        sm.include("abc");
        assert!(sm.accepts("abc"));
        assert_eq!(sm.accepts("a"), false, "expected fail on word: \"a\"");

        sm.include("a");
        assert!(sm.accepts("abc"));
        assert_eq!(sm.accepts("a"), true, "expected acceptance of word: \"a\"");

        sm.include("Loki");
        assert!(sm.accepts("abc"));
        assert_eq!(sm.accepts("a"), true, "expected acceptance of word: \"a\"");
        assert_eq!(sm.accepts("Loki"), true, "expected acceptance of word: \"Loki\"");
    }

    #[test]
    fn sm_include_empty_string() {
        let mut sm = StateMachine::new();
        sm.include("");
        assert!(sm.accepts(""));
    }
}
