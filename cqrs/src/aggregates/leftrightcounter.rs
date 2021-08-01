// Example composite aggregate which borrows functionality from other primitives

use crate::{ Configurable, EventLoader, CommandHandler };
use super::counter;
use log::{ trace };

#[derive(Debug, Eq, PartialEq)]
pub struct Config {
    left: counter::Config,
    right: counter::Config,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Commands {
    Left (counter::Commands),
    Right (counter::Commands),
}

#[derive(Debug, Eq, PartialEq)]
pub enum Events {
    Left (counter::Events),
    Right (counter::Events),
}

#[derive(Debug, Eq, PartialEq)]
pub enum Errors {
    Left (counter::Errors),
    Right (counter::Errors),
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct State {
    left: counter::State,
    right: counter::State,
}

impl Config {
    pub fn new(left: counter::Config, right: counter::Config) -> Self {

        Config { left, right }
    }
}

impl Configurable for State {
    type Config = Config;

    fn init(config: &Config) -> Self {
        trace!("LeftRightCounter::Init({:?})", config);
        let mut state = State::default();
        state.left = counter::State::init(&config.left);
        state.right = counter::State::init(&config.right);
        state
    }
}

impl EventLoader for State {
    type Config = Config;
    type Events = Events;

    fn apply(&mut self, config: &Config, event: &Events) {
        match event {
            Events::Left (event) => self.left.apply(&config.left, event),
            Events::Right (event) => self.right.apply(&config.right, event),
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
            Commands::Left (command) => self.left.eval(&config.left, command).map(Events::Left).map_err(Errors::Left),
            Commands::Right (command) => self.right.eval(&config.right, command).map(Events::Right).map_err(Errors::Right),
        }
    }
}
