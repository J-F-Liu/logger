use std::io::{stdout, Write};
use {Logger, MessageType, format_message};

/// Write log to standand output.
pub struct StdoutLogger;

impl Logger for StdoutLogger {
  fn log(&self, msg_type:MessageType, message:&str) {
    stdout().write(format_message(msg_type, message).as_bytes()).unwrap();
    stdout().flush().unwrap();
  }
}

#[test]
fn stdout_logger_works() {
  let logger = StdoutLogger;
  logger.info("Test info message.");
  logger.warn("Test warn message.");
  logger.error("Test error message.");
}
