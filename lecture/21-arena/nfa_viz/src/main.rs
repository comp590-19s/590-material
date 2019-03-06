#![allow(unused)]

/**
 * This file demonstrates many of the key concepts needed to represent and construct
 * a simple NFA appropriate for understanding the fundamental building blocks needed
 * to implement Thompson's Regex -> NFA algorithm.
 */

fn main() {
    let mut m = NFA::new();

    // Let's make an NFA for: a(b|c)*d
    // In an homage to Thompson, let's step through RPN where · is catenation.
    // Complete input we'll process char-by-char.
    // abc|*·d·
    
    // a
    let a = m.add(Match(Char::Literal('a'), None));
    // b
    
    // c
    
    // |
    
    // *
    
    // ·
    
    // d
    
    // ·
    
    // Finalize by connecting start and end
    m.join(m.start, a);
    let end = m.add(End);
    m.join(a, end);

    // We're printing a dump to stderr and dot representation to stdout
    // so that we can see the dump contents printed in our terminal
    // and pipe stdout to produce dot graphical visuals simultaneously.
    // Command: cargo run | dot -Tsvg -o/vagrant/nfa.svg
    eprintln!("{}", nfa_dump(&m));
    println!("{}", nfa_dot(&m));
}

/**
 * An NFA is represented by an arena Vec of States
 * and a start state.
 */
#[derive(Debug)]
struct NFA {
    states: Vec<State>,
    start: StateId,
}

impl NFA {
    fn new() -> NFA {
        NFA {
            states: vec![Start(None)],
            start: 0,
        }
    }

    /**
     * Add a state to the NFA and get its arena ID back
     */
    fn add(&mut self, state: State) -> StateId {
        let idx = self.states.len();
        self.states.push(state);
        idx
    }

    /**
     * Join a loose end of one state to another by IDs.
     * Note in the Split case, only the 2nd ID (rhs) is being bound.
     * It is assumed when building an NFA with these constructs
     * that the lhs of an Split state will always be known and bound.
     */
    fn join(&mut self, from: StateId, to: StateId) {
        match self.states[from] {
            Start(ref mut next) => *next = Some(to),
            Match(_, ref mut next) => *next = Some(to),
            Split(_, ref mut next) => *next = Some(to),
            End => {}
        }
    }
}

/**
 * States are the elements of our NFA Graph
 * Start is starting state
 * Match is a state with a single matching transition out
 * Split is a state with two epsilon transitions out
 * End is the final accepting state
 */
type StateId = usize;

#[derive(Debug)]
enum State {
    Start(Option<StateId>),
    Match(Char, Option<StateId>),
    Split(Option<StateId>, Option<StateId>),
    End,
}
use self::State::*;

/**
 * Chars are the matching label of a non-epsilon edge in the
 * transition diagram representation of the NFA.
 */
#[derive(Debug)]
enum Char {
    Literal(char),
    Any,
}

impl std::fmt::Display for Char {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Char::Literal(c) => write!(f, "{}", c),
            Char::Any => write!(f, "ANY"),
        }
    }
}

/**
 * Helper functions for visualizing our NFA
 * Both at the internal representation level and in dot format
 * to generate a graphical representation.
 */
fn nfa_dump(nfa: &NFA) -> String {
    let mut s = String::new();
    for (id, state) in nfa.states.iter().enumerate() {
        s.push_str(&format!("{:03} | {:?}\n", id, state));
    }
    s
}

fn nfa_dot(nfa: &NFA) -> String {
    let mut dot = String::from("digraph nfa {\n\tnode [shape = circle];\n");
    for (id, state) in nfa.states.iter().enumerate() {
        dot.push_str(&match state {
            Start(Some(next)) => format!("\tstart [shape=\"none\"]\n\tstart -> {}\n", next),
            Match(c, Some(next)) => format!("\t{} -> {} [label=\"{}\"]\n", id, next, c),
            Split(Some(lhs), Some(rhs)) => format!(
                "\t{0} -> {1} [label=\"ε\"]\n\t{0} -> {2} [label=\"ε\"]",
                id, rhs, lhs
            ),
            End => format!("\t{} [shape=\"doublecircle\"]\n", id),
            _ => String::new(),
        });
    }
    dot += "}";
    dot
}
