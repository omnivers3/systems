extern crate sink;

use std::cell::RefCell;

use sink::*;

pub trait ActorState<TConfig> {
    fn from(config: &TConfig) -> Self;
}

pub trait Actor {
    type TState;
    type TCommands;
    type TEvents;
    type TResult;

    fn handle(&self,
        state: &mut Self::TState,
        command: Self::TCommands,
        events: &impl Sink<TInput=Self::TEvents, TResult=()>
    ) -> Self::TResult;
}

#[derive(Clone)]
pub struct ActorSystem<'a, TState, TCommands, TResult, TEvents, TActor, TEventSink>
where
    TActor: Actor<TState=TState, TCommands=TCommands, TResult=TResult, TEvents=TEvents>,
    TState: ActorState<TActor>,
    TEventSink: Sink<TInput=TEvents, TResult=()>,
{
    actor: TActor,
    state: RefCell<TState>,
    event_sink: &'a TEventSink,
}

impl<'a, TState, TCommands, TResult, TEvents, TActor, TEventSink> ActorSystem<'a, TState, TCommands, TResult, TEvents, TActor, TEventSink>
where
    TActor: Actor<TState=TState, TCommands=TCommands, TResult=TResult, TEvents=TEvents>,
    TState: ActorState<TActor>,
    TEventSink: Sink<TInput=TEvents, TResult=()>,
{
    pub fn new(
        actor: TActor,
        event_sink: &'a TEventSink,
    ) -> Self {
        let state = TState::from(&actor);
        ActorSystem {
            actor,
            state: RefCell::new(state),
            event_sink,
        }
    }
}

impl<'a, TState, TCommands, TResult, TEvents, TActor, TEventSink> Sink for ActorSystem<'a, TState, TCommands, TResult, TEvents, TActor, TEventSink>
where
    TActor: Actor<TState=TState, TCommands=TCommands, TResult=TResult, TEvents=TEvents>,
    TState: ActorState<TActor>,
    TEventSink: Sink<TInput=TEvents, TResult=()>,
{
    type TInput = TCommands;
    type TResult = TResult;

    fn send(&self, input: Self::TInput) -> Self::TResult {
        let mut state = self.state.borrow_mut();
        self.actor.handle(&mut *state, input, self.event_sink)
    }
}

pub trait IntoActorSystem<'a, TActor>
where
    TActor: Actor,
    TActor::TState: ActorState<TActor>,
{
    fn bind<TEventSink>(self,
        events: &'a TEventSink,
    ) -> ActorSystem<'a, TActor::TState, TActor::TCommands, TActor::TResult, TActor::TEvents, TActor, TEventSink>
    where
        TEventSink: Sink<TInput=TActor::TEvents, TResult=()>;
}

impl<'a, TActor> IntoActorSystem<'a, TActor> for TActor
where
    TActor: Actor,
    TActor::TState: ActorState<TActor>,
{
    fn bind<TEventSink>(self,
        events: &'a TEventSink,
    ) -> ActorSystem<'a, TActor::TState, TActor::TCommands, TActor::TResult, TActor::TEvents, TActor, TEventSink>
    where
        TEventSink: Sink<TInput=TActor::TEvents, TResult=()>,
    {
        ActorSystem::new(self, events)
    }
}