// use super::{ StateLoader, CommandHandler };
use crate::{ Configurable, EventLoader, CommandHandler };
use log::{ trace };

#[derive(Debug, Eq, PartialEq)]
pub struct Config {
    limit: u32,
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

#[derive(Debug, Default, Eq, PartialEq)]
pub struct State {
    pub value: u32,
}

impl Configurable for State {
    type Config = Config;

    fn init(config: &Config) -> Self {
        let state = State::default();
        trace!("Counter::Init({:?}) = {:?}", config, state);
        state
    }
}

impl EventLoader for State {
    type Config = Config;
    type Events = Events;

    fn apply(&mut self, config: &Config, event: &Events) {
        match event {
            Events::Incremented => {
                if self.value == config.limit {
                    panic!("Invalid increment");
                }
                self.value += 1;
            }
        }
    }
}

impl CommandHandler for State {
    type Config = Config;
    type Commands = Commands;
    type Events = Events;
    type Errors = Errors;

    fn eval(&self, config: &Config, command: &Commands) -> Result<Events, Errors> {
        match command {
            Commands::Increment => {
                if self.value == config.limit {
                    Err(Errors::OutOfBounds(config.limit))
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
        let config = Config {
            limit: 1
        };
        let mut state = State {
            value: 0,
        };
        state.apply(&config, &Events::Incremented);
        assert_eq!(1, state.value);
    }

    #[test]
    #[should_panic]
    fn panic_on_apply_invalid_increment() {
        let config = Config {
            limit: 1
        };
        let mut state = State {
            value: 1,
        };
        state.apply(&config, &Events::Incremented);
    }

    #[test]
    fn eval_valid_command() {
        let config = Config {
            limit: 1
        };
        let state = State {
            value: 0,
        };
        let event = state.eval(&config, &Commands::Increment).unwrap();
        assert_eq!(Events::Incremented, event);
    }

    #[test]
    fn eval_invalid_command() {
        let config = Config {
            limit: 1
        };
        let state = State {
            value: 1,
        };
        let error = state.eval(&config, &Commands::Increment).unwrap_err();
        assert_eq!(Errors::OutOfBounds(1), error);
    }
}