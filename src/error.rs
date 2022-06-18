#![allow(unused, dead_code)]
use std::error::Error;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct GenericError {
	pub message: String
}

impl GenericError {
	pub fn new<T: ToString>(m: T) -> Self {
		Self { message: m.to_string() }
	}
}

impl Display for GenericError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut new_string: String = String::from("Error: ");
		new_string.push_str(&self.message);
        f.write_str(&new_string)
    }
}

impl Error for GenericError {}