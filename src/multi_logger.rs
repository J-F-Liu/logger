use {Logger, MessageType};

/// Write log to multiple loggers.
pub struct MultiLogger<'a> {
  loggers: Vec<&'a Logger>
}

impl<'a> MultiLogger<'a> {
  pub fn new(loggers: Vec<&Logger>) -> MultiLogger {
    MultiLogger { loggers: loggers }
  }
}

impl<'a> Logger for MultiLogger<'a> {
  fn log(&self, msg_type:MessageType, message:&str) {
    for logger in &self.loggers {
      logger.log(msg_type, message);
    }
  }
}

#[test]
fn multi_logger_works() {
  use {StdoutLogger, FileLogger};
  let l1 = StdoutLogger;
  let l2 = FileLogger::new("test2.log");
  let logger = MultiLogger::new(vec![&l1, &l2]);
  logger.info("Test info message.");
  logger.warn("Test warn message.");
  logger.error("Test error message.");
}
