pub use delete::*;
pub use init::*;
pub use insert::*;
pub use navigate::*;
pub use read::*;

mod delete;
mod init;
mod insert;
mod navigate;
mod read;

#[cfg(test)]
mod tests {
	use crate::StringEdit;

	#[test]
	fn value_len() {
		let edit = StringEdit::new("abc", 0);
		assert_eq!(edit.chars.len(), 3)
	}
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct StringEdit {
	pub chars: Vec<char>,
	pub cursor_index: usize,
}

