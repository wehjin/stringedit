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
			(Validity::IfNotEmpty, "a"),
			(Validity::IfInt, "1"),
			(Validity::IfInt, "0"),
			(Validity::IfInt, "-1"),
			(Validity::IfUnsignedInt, "1"),
			(Validity::IfUnsignedInt, "0"),
			(Validity::IfDouble, "1.1"),
			(Validity::IfDouble, "0"),
			(Validity::IfDouble, "-1.1"),
		];
		for (valid, text) in tests {
			let edit = StringEdit::new(text, 0, valid);
			assert_eq!(edit.is_valid(), true, "{:?} should be valid for {:?}", valid, text)
		}
	}

	#[test]
	fn invalid() {
		let tests = vec![
			(Validity::IfNotEmpty, ""),
			(Validity::IfInt, "a"),
			(Validity::IfUnsignedInt, "a"),
			(Validity::IfUnsignedInt, "-1"),
			(Validity::IfDouble, "a"),
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
			Validity::IfNotEmpty => !self.read().trim().is_empty(),
			Validity::IfInt => self.read().parse::<i64>().is_ok(),
			Validity::IfUnsignedInt => self.read().parse::<u64>().is_ok(),
			Validity::IfDouble => self.read().parse::<f64>().is_ok(),
		}
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Validity {
	Always,
	IfNotEmpty,
	IfInt,
	IfUnsignedInt,
	IfDouble,
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