use {Logger, MessageType};

/// Write log to multiple loggers.
pub struct MultiLogger {
  loggers: Vec<Box<Logger>>
}

impl MultiLogger {
  pub fn new(loggers: Vec<Box<Logger>>) -> MultiLogger {
    MultiLogger { loggers: loggers }
  }

  pub fn stdout_and_filelogger(file_path: &str) -> MultiLogger {
    use {StdoutLogger, FileLogger};
    let l1 = Box::new(StdoutLogger);
    let l2 = Box::new(FileLogger::new(file_path));
    MultiLogger::new(vec![l1, l2])
  }
}

impl Logger for MultiLogger {
  fn log(&self, msg_type:MessageType, message:&str) {
    for logger in &self.loggers {
      logger.log(msg_type, message);
    }
  }
}

#[test]
fn multi_logger_works() {
  let logger = MultiLogger::stdout_and_filelogger("test2.log");
  logger.info("Test info message.");
  logger.warn("Test warn message.");
  logger.error("Test error message.");
}
