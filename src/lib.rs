use log::{LevelFilter, Log, Metadata, Record, SetLoggerError};

#[derive(Default)]
pub struct CompositeLogger {
    loggers: Vec<Box<dyn Log>>,
}

impl CompositeLogger {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a logger to delegate the logs to
    pub fn with_logger(mut self, logger: impl Log + 'static) -> Self {
        self.loggers.push(Box::new(logger));
        self
    }

    /// Initializes the global logger with the built composite logger.
    ///
    /// This should be called early in the execution of a Rust program. Any log
    /// events that occur before initialization will be ignored.
    ///
    /// # Errors
    ///
    /// This function will fail if it is called more than once, or if another
    /// library has already initialized a global logger.
    pub fn try_init(self) -> Result<(), SetLoggerError> {
        let r = log::set_boxed_logger(Box::new(self));

        if r.is_ok() {
            log::set_max_level(LevelFilter::max());
        }

        r
    }

    /// Initializes the global logger with the built composite logger.
    ///
    /// This should be called early in the execution of a Rust program. Any log
    /// events that occur before initialization will be ignored.
    ///
    /// # Panics
    ///
    /// This function will panic if it is called more than once, or if another
    /// library has already initialized a global logger.
    pub fn init(self) {
        self.try_init()
            .expect("CompositeLogger::init should not be called after logger initialized");
    }
}

impl Log for CompositeLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.loggers.iter().any(|logger| logger.enabled(metadata))
    }

    fn log(&self, record: &Record) {
        self.loggers
            .iter()
            .filter(|logger| logger.enabled(record.metadata()))
            .for_each(|logger| logger.log(record));
    }

    fn flush(&self) {
        self.loggers.iter().for_each(|logger| logger.flush());
    }
}
