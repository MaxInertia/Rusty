use std::collections::HashMap;

// Deterministic Finite Automata (DFA)
// also known as a Finite-state Machine
#[derive(Debug)]
pub struct StateMachine {
    transitions: HashMap<char, StateMachine>,
    terminal: bool,
}

impl StateMachine {
    pub fn new() -> Self {
        StateMachine {
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
        if let Some(current_char) = word.chars().nth(0) {
            let remaining = &word[1..];

            match self.trans_mut().get_mut(&current_char) {
                Some(ref mut state) => {
                    if remaining.len() == 0 {
                        state.terminal = true;
                    } else {
                        state.include(remaining)
                    }
                },
                None => {
                    let mut new_state = StateMachine {
                        transitions: HashMap::new(),
                        terminal: remaining.len() == 0,
                    };

                    println!("At \"{}\", rem.len = {}. new_state.terminal = {}", current_char, remaining.len(), new_state.terminal);

                    new_state.include(remaining);
                    self.trans_mut().insert(current_char, new_state);
                }
            }
        }
    }

    // trans is a getter for the state transitions that is immutable.
    fn trans(&self) -> &HashMap<char, StateMachine> {
        &self.transitions
    }

    // trans_mut is a getter for the state transitions that allows mutation.
    fn trans_mut(&mut self) -> &mut HashMap<char, StateMachine> {
        &mut self.transitions
    }

}

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
