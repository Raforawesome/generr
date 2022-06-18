mod error;
pub use error::GenericError;

#[cfg(test)]
mod tests {
	use crate::GenericError;

    #[test]
    fn it_works() {
		let err: GenericError = GenericError::new("Test error message.");
		println!("{}", err);
    }
}
