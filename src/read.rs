use crate::StringEdit;

#[cfg(test)]
mod tests {
	use crate::StringEdit;

	#[test]
	fn read() {
		let red = StringEdit::new("abc", 0).read();
		assert_eq!(red, "abc")
	}

	mod read_spot {
		use crate::{Spot, StringEdit};

		#[test]
		fn at_cursor() {
			let position = StringEdit::new("abc", 0).read_spot(0);
			assert_eq!(position, Some(Spot { char: 'a', is_cursor: true }))
		}

		#[test]
		fn at_normal() {
			let position = StringEdit::new("abc", 0).read_spot(1);
			assert_eq!(position, Some(Spot { char: 'b', is_cursor: false }))
		}

		#[test]
		fn at_eos() {
			let position = StringEdit::new("abc", 3).read_spot(3);
			assert_eq!(position, Some(Spot { char: '\n', is_cursor: true }))
		}
	}
}

impl StringEdit {
	pub fn read(&self) -> String {
		self.chars.iter().collect()
	}

	pub fn read_spot(&self, index: usize) -> Option<Spot> {
		if index > self.chars.len() {
			None
		} else if index == self.chars.len() {
			Some(Spot {
				char: '\n',
				is_cursor: self.cursor_index == index,
			})
		} else {
			Some(Spot {
				char: self.chars[index],
				is_cursor: self.cursor_index == index,
			})
		}
	}
}


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Spot {
	pub char: char,
	pub is_cursor: bool,
}