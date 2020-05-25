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
	use crate::{Action, StringEdit};

	#[test]
	fn to_string() {
		let edit = StringEdit::new("abc", 0);
		assert_eq!(edit.to_string(), String::from("abc"))
	}

	#[test]
	fn value_len() {
		let edit = StringEdit::new("abc", 0);
		assert_eq!(edit.chars.len(), 3)
	}

	#[test]
	fn action() {
		let edit = StringEdit::empty().edit(Action::InsertChar('a'));
		assert_eq!(edit, StringEdit::new("a", 1));
	}
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct StringEdit {
	pub chars: Vec<char>,
	pub cursor_index: usize,
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
	pub fn to_string(&self) -> String { self.chars.iter().collect() }
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