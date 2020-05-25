use crate::StringEdit;

#[cfg(test)]
mod tests {
	mod move_cursor {
		mod right {
			use crate::{StringEdit, Validity};

			#[test]
			fn from_far_left() {
				let edit = StringEdit::new("a", 0, Validity::NotEmpty).move_cursor_right();
				assert_eq!(edit, StringEdit { chars: vec!['a'], cursor_index: 1, validity: Validity::NotEmpty })
			}

			#[test]
			fn from_far_right() {
				let edit = StringEdit::new("a", 1, Validity::NotEmpty).move_cursor_right();
				assert_eq!(edit, StringEdit { chars: vec!['a'], cursor_index: 1, validity: Validity::NotEmpty })
			}
		}

		mod left {
			use crate::{StringEdit, Validity};

			#[test]
			fn from_far_left() {
				let edit = StringEdit::new("a", 0, Validity::NotEmpty).move_cursor_left();
				assert_eq!(edit, StringEdit { chars: vec!['a'], cursor_index: 0, validity: Validity::NotEmpty })
			}

			#[test]
			fn from_far_right() {
				let edit = StringEdit::new("a", 1, Validity::NotEmpty).move_cursor_left();
				assert_eq!(edit, StringEdit { chars: vec!['a'], cursor_index: 0, validity: Validity::NotEmpty })
			}
		}
	}
}

impl StringEdit {
	pub fn move_cursor_right(&self) -> Self {
		let cursor_pos = (self.cursor_index + 1).min(self.chars.len());
		StringEdit { chars: self.chars.to_vec(), cursor_index: cursor_pos, validity: self.validity }
	}

	pub fn move_cursor_left(&self) -> Self {
		let cursor_pos = if self.cursor_index > 0 { self.cursor_index - 1 } else { self.cursor_index };
		StringEdit { chars: self.chars.to_vec(), cursor_index: cursor_pos, validity: self.validity }
	}
}
