extern crate failure;

#[macro_use]
extern crate failure_derive;

use std::fs::{File, OpenOptions};
use std::{io, result};

type Result<T> = result::Result<T, DocumentServiceError>;

// custome error-type
#[derive(Debug, Fail)]
pub enum DocumentServiceError {
    #[fail(display = "you have exceeded allowed number of rate per minute")]
    RateLimitExceeded,

    #[fail(display = "I/O error: {}", _0)]
    Io(io::Error),
}

// convert IO error to DocumentServiceError
impl From<io::Error> for DocumentServiceError {
    fn from(other: io::Error) -> Self {
        DocumentServiceError::Io(other)
    }
}

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
