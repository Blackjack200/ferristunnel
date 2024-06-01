use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

pub struct StateMachine<S: Sized + Hash + Eq + Copy + Clone + Display> {
    m: HashMap<S, State<S>>,
    s: S,
}

struct State<S: Sized> {
    entry: Vec<fn(&S)>,
    permit: Vec<S>,
    exit: Vec<fn(&S)>,
}

impl<S: Sized> Default for State<S> {
    fn default() -> Self {
        Self {
            entry: vec![],
            permit: vec![],
            exit: vec![],
        }
    }
}

impl<S: Sized + Hash + Eq + Copy + Clone + Display> StateMachine<S> {
    pub fn new(init: S) -> Self {
        Self {
            m: HashMap::new(),
            s: init,
        }
    }
}

impl<S: Sized + Hash + Eq + Copy + Clone + Display> StateMachine<S> {
    fn lazy_init_state(&mut self, state: &S) -> &mut State<S> {
        if !self.m.contains_key(state) {
            self.m.insert(*state, State::default());
        }
        self.m.get_mut(state).unwrap()
    }

    pub fn entry(&mut self, state: S, on_entry: fn(&S)) {
        self.lazy_init_state(&state).entry.push(on_entry);
    }

    pub fn permit(&mut self, prev: S, next: Vec<S>) {
        self.lazy_init_state(&prev).permit.append(&mut next.clone());
    }

    pub fn exit(&mut self, state: S, on_exit: fn(&S)) {
        self.lazy_init_state(&state).exit.push(on_exit);
    }

    pub fn fire(&mut self, state: S) {
        let old_state = self.s;
        let old = self.m.get(&old_state).unwrap();
        let v = self.m.get(&state).unwrap();

        if !old.permit.contains(&state) {
            panic!("cannot fire {} in {}", &state, &old_state)
        }

        old.exit.iter().for_each(|x| x(&state));
        self.s = state;
        v.entry.iter().for_each(|x| x(&old_state));
    }
}
