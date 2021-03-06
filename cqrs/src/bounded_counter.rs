use super::*;

#[derive(Debug, Eq, PartialEq)]
pub struct Aggregate {
    limit: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct State {
    value: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Commands {
    Increment,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Events {
    Incremented,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Errors {
    OutOfBounds(u32),
}

impl Aggregate {
    pub fn new(limit: u32) -> Self {
        Aggregate { limit }
    }

    fn apply(&self, state: &mut State, event: Events) {
        match event {
            Events::Incremented => {
                if state.value == self.limit {
                    panic!("Invalid increment");
                }
                state.value += 1;
            }
        }
    }

    fn eval(&self, state: State, command: Commands) -> Result<Events, Errors> {
        match command {
            Commands::Increment => {
                if state.value == self.limit {
                    Err(Errors::OutOfBounds(self.limit))
                } else {
                    Ok(Events::Incremented)
                }
            }
        }
    }
}

#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn apply_valid_increment() {
        let a = Aggregate::new(1);
        let mut s = State { value: 0 };
        a.apply(&mut s, Events::Incremented);
        assert_eq!(1, s.value);
    }

    #[test]
    #[should_panic]
    fn panic_on_apply_invalid_increment() {
        let a = Aggregate::new(1);
        let mut s = State { value: 1 };
        a.apply(&mut s, Events::Incremented);
    }

    #[test]
    fn eval_valid_command() {
        let a = Aggregate::new(1);
        let s = State { value: 0 };
        let e = a.eval(s, Commands::Increment).unwrap();
        assert_eq!(Events::Incremented, e);
    }

    #[test]
    fn eval_invalid_command() {
        let a = Aggregate::new(1);
        let s = State { value: 1 };
        let err = a.eval(s, Commands::Increment).unwrap_err();
        assert_eq!(Errors::OutOfBounds(1), err);
    }
}

// TODO: Make Generated

impl IAggregateMeta for Aggregate {
    fn domain() -> String {
        "users_registered_bounded_counter".to_owned()
    }

    fn commands() -> Vec<String> {
        vec!["increment".to_owned()]
    }

    fn events() -> Vec<String> {
        vec!["incremented".to_owned()]
    }
}

impl IAggregate for Aggregate {
    type TState = State;
    type TCommands = Commands;
    type TEvents = Events;
    type TErrors = Errors;

    fn apply(&self, state: &mut State, event: Events) {
        (self as &Aggregate).apply(state, event)
    }

    fn eval(&self, state: State, command: Commands) -> Result<Events, Errors> {
        (self as &Aggregate).eval(state, command)
    }
}
