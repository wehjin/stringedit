use crate::StringEdit;

#[cfg(test)]
mod tests {
	mod cursor_right {
		use crate::StringEdit;

		#[test]
		fn from_far_left() {
			let edit = StringEdit::new("a", 0).cursor_right();
			assert_eq!(edit, StringEdit { chars: vec!['a'], cursor_pos: 1 })
		}

		#[test]
		fn from_far_right() {
			let edit = StringEdit::new("a", 1).cursor_right();
			assert_eq!(edit, StringEdit { chars: vec!['a'], cursor_pos: 1 })
		}
	}

	mod cursor_left {
		use crate::StringEdit;

		#[test]
		fn from_far_left() {
			let edit = StringEdit::new("a", 0).cursor_left();
			assert_eq!(edit, StringEdit { chars: vec!['a'], cursor_pos: 0 })
		}

		#[test]
		fn from_far_right() {
			let edit = StringEdit::new("a", 1).cursor_left();
			assert_eq!(edit, StringEdit { chars: vec!['a'], cursor_pos: 0 })
		}
	}
}

impl StringEdit {
	pub fn cursor_right(&self) -> Self {
		let cursor_pos = (self.cursor_pos + 1).min(self.chars.len());
		StringEdit { chars: self.chars.to_vec(), cursor_pos }
	}

	pub fn cursor_left(&self) -> Self {
		let cursor_pos = if self.cursor_pos > 0 { self.cursor_pos - 1 } else { self.cursor_pos };
		StringEdit { chars: self.chars.to_vec(), cursor_pos }
	}
}
