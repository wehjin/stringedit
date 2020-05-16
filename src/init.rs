use crate::StringEdit;

#[cfg(test)]
mod tests {
	use crate::StringEdit;


	#[test]
	fn default() {
		let default: StringEdit = Default::default();
		assert_eq!(default, StringEdit::empty());
	}

	#[test]
	fn empty() {
		let empty = StringEdit::empty();
		assert_eq!(empty, StringEdit { chars: vec![], cursor_index: 0 });
	}
}

impl Default for StringEdit {
	fn default() -> Self {
		StringEdit::empty()
	}
}

impl StringEdit {
	pub fn empty() -> Self {
		StringEdit::new("", 0)
	}

	pub fn new(value: &str, cursor_pos: usize) -> Self {
		let chars = value.chars().collect::<Vec<_>>();
		assert!(cursor_pos <= chars.len());
		StringEdit { chars, cursor_index: cursor_pos }
	}
}
