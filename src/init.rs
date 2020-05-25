use crate::{StringEdit, Validity};

#[cfg(test)]
mod tests {
	use crate::{StringEdit, Validity};

	#[test]
	fn empty() {
		let empty = StringEdit::empty(Validity::NotEmpty);
		assert_eq!(empty, StringEdit { chars: vec![], cursor_index: 0, validity: Validity::NotEmpty });
	}
}

impl StringEdit {
	pub fn empty(validity: Validity) -> Self {
		StringEdit::new("", 0, validity)
	}

	pub fn new<S: AsRef<str>>(value: S, cursor_pos: usize, validity: Validity) -> Self {
		let chars = value.as_ref().chars().collect::<Vec<_>>();
		assert!(cursor_pos <= chars.len());
		StringEdit { chars, cursor_index: cursor_pos, validity }
	}
}
