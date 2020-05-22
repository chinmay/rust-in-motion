#[macro_use]
extern crate quick_error;

use std::fs::{File, OpenOptions};
use std::{io, result};

type Result<T> = result::Result<T, DocumentServiceError>;

// custome error-type
quick_error! {
    #[derive(Debug)]
    pub enum DocumentServiceError {
        RateLimitExceeded {
            display("you have exceeded allowed number of rate per minute")
        }
        Io(cause: io::Error) {
            display("I/O error: {}", cause)
            from()
        }
    }
}

// quick_error implements Error trait, so this can be removed.
// easier way to implement Display
// easier implementation of from()

const MAX_DOCS_CREATED_PER_MINUTE: u8 = 100;

fn num_documents_created_in_last_minute() -> u8 {
    2
}

pub fn create_document(filename: &str) -> Result<File> {
    if num_documents_created_in_last_minute() > MAX_DOCS_CREATED_PER_MINUTE {
        return Err(DocumentServiceError::RateLimitExceeded);
    }

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename)?;

    Ok(file)
}

fn create_project(project_name: &str) -> Result<()> {
    create_document(&format!("{}-draft1", project_name))?;
    create_document(&format!("{}-draft2", project_name))?;
    create_document(&format!("{}-revision1", project_name))?;
    create_document(&format!("{}-revision2", project_name))?;

    Ok(())
}

fn main() {
    match create_project("my-project") {
        Ok(()) => println!("project created successfully"),
        Err(e) => println!("project creation failure {}", e),
    }
}
