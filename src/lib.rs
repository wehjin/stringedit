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
	use crate::{Action, StringEdit, Validity};

	#[test]
	fn action() {
		let edit = StringEdit::empty(Validity::Always).edit(Action::InsertChar('a'));
		assert_eq!(edit, StringEdit::new("a", 1, Validity::Always));
	}

	#[test]
	fn valid() {
		let tests = vec![
			(Validity::Always, ""),
			(Validity::NotEmpty, "a"),
			(Validity::Int, "1"),
			(Validity::Int, "0"),
			(Validity::Int, "-1"),
			(Validity::UnsignedInt, "1"),
			(Validity::UnsignedInt, "0"),
			(Validity::Double, "1.1"),
			(Validity::Double, "0"),
			(Validity::Double, "-1.1"),
		];
		for (valid, text) in tests {
			let edit = StringEdit::new(text, 0, valid);
			assert_eq!(edit.is_valid(), true, "{:?} should be valid for {:?}", valid, text)
		}
	}

	#[test]
	fn invalid() {
		let tests = vec![
			(Validity::NotEmpty, ""),
			(Validity::Int, "a"),
			(Validity::UnsignedInt, "a"),
			(Validity::UnsignedInt, "-1"),
			(Validity::Double, "a"),
		];
		for (valid, text) in tests {
			let edit = StringEdit::new(text, 0, valid);
			assert_eq!(edit.is_valid(), false, "{:?} should be invalid for {:?}", valid, text)
		}
	}
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct StringEdit {
	pub chars: Vec<char>,
	pub cursor_index: usize,
	pub validity: Validity,
}

impl StringEdit {
	pub fn edit(&self, action: Action) -> Self {
		match action {
			Action::InsertChar(c) => self.insert_char(c),
			Action::Delete => self.delete_chars(),
			Action::DeleteCharBeforeCursor => self.delete_char_before_cursor(),
			Action::DeleteCharAtCursor => self.delete_char_at_cursor(),
			Action::MoveCursorLeft => self.move_cursor_left(),
			Action::MoveCursorRight => self.move_cursor_right()
		}
	}
	pub fn is_valid(&self) -> bool {
		match self.validity {
			Validity::Always => true,
			Validity::NotEmpty => !self.read().trim().is_empty(),
			Validity::Int => self.read().parse::<i64>().is_ok(),
			Validity::UnsignedInt => self.read().parse::<u64>().is_ok(),
			Validity::Double => self.read().parse::<f64>().is_ok(),
		}
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Validity {
	Always,
	NotEmpty,
	Int,
	UnsignedInt,
	Double,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Action {
	InsertChar(char),
	Delete,
	DeleteCharBeforeCursor,
	DeleteCharAtCursor,
	MoveCursorLeft,
	MoveCursorRight,
}