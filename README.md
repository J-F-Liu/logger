# A logger crate for Rust.

This crate includes a `Logger` trait and the following implementations:
- `FileLogger`	Write log to text file.
- `MultiLogger`	Write log to multiple loggers.
- `StdoutLogger`	Write log to standand output.

## Usage

Cargo.toml
```
[dependencies.logger]
git = "https://github.com/J-F-Liu/logger.git"
```

Rust code.
```Rust
extern crate logger;

use logger::{Logger, MultiLogger};

fn main() {
  let logger = MultiLogger::stdout_and_filelogger("backup_folder.log");
  logger.info("Start backup");
}
```
