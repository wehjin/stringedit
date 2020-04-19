use crate::StringEdit;

#[cfg(test)]
mod tests {
	mod previous_char {
		use crate::StringEdit;

		#[test]
		fn from_far_left() {
			let edit = StringEdit::new("abc", 0).delete_char_before_cursor();
			assert_eq!(edit, StringEdit { chars: vec!['a', 'b', 'c'], cursor_pos: 0 })
		}

		#[test]
		fn from_far_right() {
			let edit = StringEdit::new("abc", 3).delete_char_before_cursor();
			assert_eq!(edit, StringEdit { chars: vec!['a', 'b'], cursor_pos: 2 })
		}
	}

	mod cursor_char {
		use crate::StringEdit;

		#[test]
		fn from_far_left() {
			let edit = StringEdit::new("abc", 0).delete_char_at_cursor();
			assert_eq!(edit, StringEdit { chars: vec!['b', 'c'], cursor_pos: 0 })
		}

		#[test]
		fn from_far_right() {
			let edit = StringEdit::new("abc", 3).delete_char_at_cursor();
			assert_eq!(edit, StringEdit { chars: vec!['a', 'b', 'c'], cursor_pos: 3 })
		}
	}
}

impl StringEdit {
	pub fn delete_char_before_cursor(&self) -> Self {
		if self.cursor_pos == 0 {
			self.clone()
		} else {
			let chars = [&self.chars[0..self.cursor_pos - 1], &self.chars[self.cursor_pos..]].concat();
			let cursor_pos = self.cursor_pos - 1;
			StringEdit { chars, cursor_pos }
		}
	}

	pub fn delete_char_at_cursor(&self) -> Self {
		if self.cursor_pos >= self.chars.len() {
			self.clone()
		} else {
			let chars = [&self.chars[0..self.cursor_pos], &self.chars[self.cursor_pos + 1..]].concat();
			let cursor_pos = self.cursor_pos;
			StringEdit { chars, cursor_pos }
		}
	}
}
