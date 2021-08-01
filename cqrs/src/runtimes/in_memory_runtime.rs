use crate::{ Configurable, CommandHandler, EventLoader, EventStore, Runtime };

pub struct InMemoryRuntime<'a, Config, T: Configurable<Config=Config> + CommandHandler<Config=Config> + ?Sized> {
    store: &'a dyn EventStore<T::Events>,
    config: &'a Config,
}

impl<'a, Config, T: Configurable<Config=Config> + CommandHandler<Config=Config>> InMemoryRuntime<'a, Config, T> {
    #[allow(dead_code)]
    pub fn init(store: &'a dyn EventStore<T::Events>, config: &'a Config) -> Self {
        InMemoryRuntime {
            store,
            config,
        }
    }
}

impl<'a, Config, Events, T: Configurable<Config=Config> + CommandHandler<Config=Config, Events=Events> + EventLoader<Config=Config, Events=Events>> Runtime<T> for InMemoryRuntime<'a, Config, T>
where
    <T as CommandHandler>::Events: Sized + Copy,
    T::Errors: Sized,
{
    fn handle(&self, command: &T::Commands) -> Result<<T as CommandHandler>::Events, T::Errors> {
        let mut state = T::init(self.config);
        self.store.load(&mut |e| state.apply(self.config, e));
        state.eval(self.config, command)
    }
}