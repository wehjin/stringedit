use crate::StringEdit;

#[cfg(test)]
mod tests {
	use crate::{StringEdit, Validity};

	#[test]
	fn all() {
		let edit = StringEdit::new("abc", 2, Validity::NotEmpty).delete_chars();
		assert_eq!(edit, StringEdit { chars: vec![], cursor_index: 0, validity: Validity::NotEmpty });
	}

	mod previous_char {
		use crate::{StringEdit, Validity};

		#[test]
		fn from_far_left() {
			let edit = StringEdit::new("abc", 0, Validity::NotEmpty).delete_char_before_cursor();
			assert_eq!(edit, StringEdit { chars: vec!['a', 'b', 'c'], cursor_index: 0, validity: Validity::NotEmpty })
		}

		#[test]
		fn from_far_right() {
			let edit = StringEdit::new("abc", 3, Validity::NotEmpty).delete_char_before_cursor();
			assert_eq!(edit, StringEdit { chars: vec!['a', 'b'], cursor_index: 2, validity: Validity::NotEmpty })
		}
	}

	mod cursor_char {
		use crate::{StringEdit, Validity};

		#[test]
		fn from_far_left() {
			let edit = StringEdit::new("abc", 0, Validity::NotEmpty).delete_char_at_cursor();
			assert_eq!(edit, StringEdit { chars: vec!['b', 'c'], cursor_index: 0, validity: Validity::NotEmpty })
		}

		#[test]
		fn from_far_right() {
			let edit = StringEdit::new("abc", 3, Validity::NotEmpty).delete_char_at_cursor();
			assert_eq!(edit, StringEdit { chars: vec!['a', 'b', 'c'], cursor_index: 3, validity: Validity::NotEmpty })
		}
	}
}

impl StringEdit {
	pub fn delete_chars(&self) -> Self {
		StringEdit { chars: vec![], cursor_index: 0, validity: self.validity }
	}

	pub fn delete_char_before_cursor(&self) -> Self {
		if self.cursor_index == 0 {
			self.clone()
		} else {
			let chars = [&self.chars[0..self.cursor_index - 1], &self.chars[self.cursor_index..]].concat();
			let cursor_pos = self.cursor_index - 1;
			StringEdit { chars, cursor_index: cursor_pos, validity: self.validity }
		}
	}

	pub fn delete_char_at_cursor(&self) -> Self {
		if self.cursor_index >= self.chars.len() {
			self.clone()
		} else {
			let chars = [&self.chars[0..self.cursor_index], &self.chars[self.cursor_index + 1..]].concat();
			let cursor_pos = self.cursor_index;
			StringEdit { chars, cursor_index: cursor_pos, validity: self.validity }
		}
	}
}
