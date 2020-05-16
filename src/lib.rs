pub use delete::*;
pub use init::*;
pub use navigate::*;
pub use read::*;

mod init;
mod navigate;
mod delete;
mod read;

#[cfg(test)]
mod tests {
	use crate::StringEdit;

	mod insert {
		use crate::StringEdit;

		#[test]
		fn char() {
			let inserted = StringEdit::empty().insert_char('A');
			assert_eq!(inserted, StringEdit { chars: vec!['A'], cursor_index: 1 })
		}
	}

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

impl StringEdit {
	pub fn insert_char(&self, c: char) -> Self {
		if c.is_control() {
			self.clone()
		} else {
			let new = [c];
			let chars = [&self.chars[0..self.cursor_index], &new, &self.chars[self.cursor_index..]].concat();
			let cursor_pos = self.cursor_index + 1;
			StringEdit { chars, cursor_index: cursor_pos }
		}
	}
}
