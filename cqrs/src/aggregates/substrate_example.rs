use crate::{ DefaultConfigurable, CommandHandler, EventLoader };

#[derive(Debug)]
#[allow(dead_code)]
pub enum Commands {
    DoSomething(u32),
    CauseError(),
}

#[derive(Clone, Copy, Debug)]
pub enum Events {
    SomethingStored(u32)
}

#[allow(unused_imports)]
use log::{info, trace, warn};

#[derive(Debug)]
pub enum Errors {
    NoneValue,
    StorageOverflow,
}

#[derive(Debug, Default)]
pub struct State {
    something: Option<u32>,
}

impl DefaultConfigurable for State {}

impl EventLoader for State {
    type Events = Events;

    fn apply(&mut self, _: &(), event: &Events) {
        trace!("SubstrateExample::Apply({:?})", event);
        match *event {
            Events::SomethingStored(something) => {
                self.something = Some(something);
            }
        }
    }
}

impl CommandHandler for State {
    type Commands = Commands;
    type Events = Events;
    type Errors = Errors;

    fn eval(&self, _: &(), command: &Commands) -> Result<Events, Errors> {
        trace!("SubstrateExample::Eval({:?})", command);
        match *command {
            Commands::DoSomething(something) => {
                Ok(Events::SomethingStored(something))
            },
            Commands::CauseError() => {
                match self.something {
                    None => Err(Errors::NoneValue),
                    Some(old) => {
                        // Increment the value read from storage; will error in the event of overflow.
                        let new = old.checked_add(1).ok_or(Errors::StorageOverflow)?;
                        Ok(Events::SomethingStored(new))
                    }
                }
            }
        }
    }
}

// impl Aggregate for Domain {
//     type Events = Events;
//     type Commands = Commands;
//     type Errors = Errors;

//     fn eval(&self, command: &Commands) -> Result<Events, Errors> {
//         trace!("Domain::Eval({:?})", command);
//         match *command {
//             Commands::DoSomething(something) => {
//                 Ok(Events::SomethingStored(something))
//             },
//             Commands::CauseError() => {
//                 match self.something {
//                     None => Err(Errors::NoneValue),
//                     Some(old) => {
//                         // Increment the value read from storage; will error in the event of overflow.
//                         let new = old.checked_add(1).ok_or(Errors::StorageOverflow)?;
//                         Ok(Events::SomethingStored(new))
//                     }
//                 }
//             }
//         }
//     }

//     fn apply(&mut self, event: &Events) {
//         trace!("Domain::Apply({:?})", event);
//         match *event {
//             Events::SomethingStored(something) => {
//                 self.something = Some(something);
//             }
//         }
//     }
// }

mod test {
    #[allow(unused_imports)]
    use super::{ Commands, Events, Errors, State };
    #[allow(unused_imports)]
    use crate::{ CommandHandler, EventLoader, Runtime };
    #[allow(unused_imports)]
    use crate::runtimes::in_memory_runtime::{ InMemoryRuntime };
    #[allow(unused_imports)]
    use crate::stores::in_memory_store::{ InMemoryEventStore };
    // #[allow(unused_imports)]
    // use super::temp;

    // trait RootAggregate: Aggregate<Context=(), Commands=Commands, Events=Events, Errors=Errors> {}

    #[test]
    fn test_001() {
        let store = InMemoryEventStore::<Events>::init();
        let runtime = InMemoryRuntime::<'_, (), State>::init(&store, &());

        let result = runtime.handle(&Commands::DoSomething(10));

        // assert!(result.is_ok(), "should test behaviors of domain");
        match result {
            Ok(event) => {
                println!("Store Event {:?}", event);
            },
            Err(error) => {
                println!("Announce Error: {:?}", error);
            },
        }
        assert!(false, "nope");
    }

    #[test]
    fn test_101() {
        let state = State::default();

        match state.eval(&(), &Commands::DoSomething(10)) {
            Ok(event) => {
                println!("Store Event {:?}", event);
            },
            Err(error) => {
                println!("Announce Error: {:?}", error);
            },
        }
        assert!(false, "nope");
    }
}