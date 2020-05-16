use crate::StringEdit;

#[cfg(test)]
mod tests {
	use crate::StringEdit;

	#[test]
	fn char() {
		let inserted = StringEdit::empty().insert_char('A');
		assert_eq!(inserted, StringEdit { chars: vec!['A'], cursor_index: 1 })
	}
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
