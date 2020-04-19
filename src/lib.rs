pub use delete::*;
pub use init::*;
pub use navigate::*;

mod init;
mod navigate;

#[cfg(test)]
mod tests {
	use crate::StringEdit;

	mod insert {
		use crate::StringEdit;

		#[test]
		fn char() {
			let inserted = StringEdit::empty().insert_char('A');
			assert_eq!(inserted, StringEdit { chars: vec!['A'], cursor_pos: 1 })
		}
	}

	#[test]
	fn read() {
		let red = StringEdit::new("abc", 0).read();
		assert_eq!(red, "abc");
	}
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct StringEdit {
	pub chars: Vec<char>,
	pub cursor_pos: usize,
}

mod delete;

impl StringEdit {
	pub fn insert_char(&self, c: char) -> Self {
		if c.is_control() {
			self.clone()
		} else {
			let new = [c];
			let chars = [&self.chars[0..self.cursor_pos], &new, &self.chars[self.cursor_pos..]].concat();
			let cursor_pos = self.cursor_pos + 1;
			StringEdit { chars, cursor_pos }
		}
	}

	pub fn read(&self) -> String {
		self.chars.iter().collect()
	}
}
