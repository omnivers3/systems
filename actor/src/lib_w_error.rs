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
    type TErrors;
    type TResult;

    fn handle(&self,
        state: &mut Self::TState,
        command: Self::TCommands,
        events: &impl Sink<TInput=Self::TEvents, TResult=()>,
        errors: &impl Sink<TInput=Self::TErrors, TResult=()>
    ) -> Self::TResult;
}

#[derive(Clone)]
pub struct ActorSystem<'a, 'b, TState, TCommands, TResult, TEvents, TErrors, TActor, TEventSink, TErrorSink>
where
    TActor: Actor<TState=TState, TCommands=TCommands, TResult=TResult, TEvents=TEvents, TErrors=TErrors>,
    TState: ActorState<TActor>,
    TEventSink: Sink<TInput=TEvents, TResult=()>,
    TErrorSink: Sink<TInput=TErrors, TResult=()>,
{
    actor: TActor,
    state: RefCell<TState>,
    event_sink: &'a TEventSink,
    error_sink: &'b TErrorSink,
}

impl<'a, 'b, TState, TCommands, TResult, TEvents, TErrors, TActor, TEventSink, TErrorSink> ActorSystem<'a, 'b, TState, TCommands, TResult, TEvents, TErrors, TActor, TEventSink, TErrorSink>
where
    TActor: Actor<TState=TState, TCommands=TCommands, TResult=TResult, TEvents=TEvents, TErrors=TErrors>,
    TState: ActorState<TActor>,
    TEventSink: Sink<TInput=TEvents, TResult=()>,
    TErrorSink: Sink<TInput=TErrors, TResult=()>,
{
    pub fn new(
        actor: TActor,
        event_sink: &'a TEventSink,
        error_sink: &'b TErrorSink,
    ) -> Self {
        let state = TState::from(&actor);
        ActorSystem {
            actor,
            state: RefCell::new(state),
            event_sink,
            error_sink,
        }
    }
}

impl<'a, 'b, TState, TCommands, TResult, TEvents, TErrors, TActor, TEventSink, TErrorSink> Sink for ActorSystem<'a, 'b, TState, TCommands, TResult, TEvents, TErrors, TActor, TEventSink, TErrorSink>
where
    TActor: Actor<TState=TState, TCommands=TCommands, TResult=TResult, TEvents=TEvents, TErrors=TErrors>,
    TState: ActorState<TActor>,
    TEventSink: Sink<TInput=TEvents, TResult=()>,
    TErrorSink: Sink<TInput=TErrors, TResult=()>,
{
    type TInput = TCommands;
    type TResult = TResult;

    fn send(&self, input: Self::TInput) -> Self::TResult {
        let mut state = self.state.borrow_mut();
        self.actor.handle(&mut *state, input, self.event_sink, self.error_sink)
    }
}

pub trait IntoActorSystem<'a, 'b, TActor>
where
    TActor: Actor,
    TActor::TState: ActorState<TActor>,
{
    fn bind<TEventSink, TErrorSink>(self,
        events: &'a TEventSink,
        errors: &'b TErrorSink,
    ) -> ActorSystem<'a, 'b, TActor::TState, TActor::TCommands, TActor::TResult, TActor::TEvents, TActor::TErrors, TActor, TEventSink, TErrorSink>
    where
        TEventSink: Sink<TInput=TActor::TEvents, TResult=()>,
        TErrorSink: Sink<TInput=TActor::TErrors, TResult=()>;
}

impl<'a, 'b, TActor> IntoActorSystem<'a, 'b, TActor> for TActor
where
    TActor: Actor,
    TActor::TState: ActorState<TActor>,
{
    fn bind<TEventSink, TErrorSink>(self,
        events: &'a TEventSink,
        errors: &'b TErrorSink,
    ) -> ActorSystem<'a, 'b, TActor::TState, TActor::TCommands, TActor::TResult, TActor::TEvents, TActor::TErrors, TActor, TEventSink, TErrorSink>
    where
        TEventSink: Sink<TInput=TActor::TEvents, TResult=()>,
        TErrorSink: Sink<TInput=TActor::TErrors, TResult=()>,
    {
        ActorSystem::new(self, events, errors)
    }
}