/// Different message types.
#[derive(Copy, Debug)]
pub enum MessageType {
    /// Designates useful information.
    Info,

    /// Designates hazardous situations.
    Warn,

    /// Designates very serious errors.
    Error,
}

impl Clone for MessageType {
    #[inline]
    fn clone(&self) -> MessageType {
        *self
    }
}

/// A common logger api.
pub trait Logger {
	fn log(&self, msg_type:MessageType, message:&str);

	fn info(&self, message:&str) {
		self.log(MessageType::Info, message);
	}

	fn warn(&self, message:&str) {
		self.log(MessageType::Warn, message);
	}

	fn error(&self, message:&str) {
		self.log(MessageType::Error, message);
	}
}

extern crate time;

/// Format the message to be logged with time prefixed.
pub fn format_message(msg_type:MessageType, message:&str) -> String {
  let type_indicator = match msg_type {
    MessageType::Info => String::new(),
    _ => format!("[{:?}] ", msg_type)
  };
  format!("{} {}{}\n",
    time::now().strftime("%Y-%m-%d %H:%M:%S").unwrap(),
    type_indicator,
    message)
}

mod stdout_logger;
pub use stdout_logger::*;

mod file_logger;
pub use file_logger::*;

mod multi_logger;
pub use multi_logger::*;
