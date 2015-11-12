use std::io::Write;
use std::fs::{File, OpenOptions};
use {Logger, MessageType, format_message};

/// Write log to text file.
pub struct FileLogger {
  log_file: String
}

impl FileLogger {
  pub fn new(file_path: &str) -> FileLogger {
    File::create(file_path).unwrap();
    FileLogger { log_file: file_path.to_string() }
  }
}

impl Logger for FileLogger {
  fn log(&self, msg_type:MessageType, message:&str) {
    let mut file = OpenOptions::new()
            .append(true)
            .write(true)
            .open(&self.log_file).unwrap();
    file.write(format_message(msg_type, message).as_bytes()).unwrap();
  }
}

#[test]
fn file_logger_works() {
  let logger = FileLogger::new("test.log");
  logger.info("Test info message.");
  logger.warn("Test warn message.");
  logger.error("Test error message.");
}
