use std::path::Path;


#[derive(Clone)]
pub enum SmallStr {
	Stack([u8; 24], usize),
	Heap(String)
}

impl SmallStr {
	pub fn from_str(s: &str) -> Self {
		if s.len() <= 24 {
			let mut data = [0; 24];
			let bytes = s.as_bytes();
			(&mut data[0..bytes.len()]).copy_from_slice(bytes);
			Self::Stack(data, bytes.len())
		} else {
			Self::Heap(String::from(s))
		}
	}

	pub fn as_str(&self) -> &str {
		match self {
			Self::Stack(data, len) => unsafe { std::str::from_utf8_unchecked(&data[0..*len]) },
			Self::Heap(s) => &s
		}
	}

	pub fn as_mut_str(&mut self) -> &mut str {
		match self {
			Self::Stack(data, len) => unsafe { std::str::from_utf8_unchecked_mut(&mut data[0..*len]) },
			Self::Heap(s) => s.as_mut_str()
		}
	}
}

impl core::ops::Deref for SmallStr {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		self.as_str()
	}
}

impl core::ops::DerefMut for SmallStr {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.as_mut_str()
	}
}

impl From<String> for SmallStr {
	fn from(value: String) -> Self {
		Self::Heap(value)
	}
}

impl From<&str> for SmallStr {
	fn from(value: &str) -> Self {
		Self::from_str(value)
	}
}

impl AsRef<Path> for SmallStr {
	fn as_ref(&self) -> &Path {
		self.as_str().as_ref()
	}
}

impl core::fmt::Debug for SmallStr {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
		self.as_str().fmt(f)
	}
}

impl core::fmt::Display for SmallStr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.as_str().fmt(f)
	}
}

impl PartialEq for SmallStr {
	fn eq(&self, other: &Self) -> bool {
		self.as_str() == other.as_str()
	}

	fn ne(&self, other: &Self) -> bool {
		self.as_str() != other.as_str()
	}
}

impl PartialEq<&str> for SmallStr {
	fn eq(&self, other: &&str) -> bool {
		self.as_str() == *other
	}

	fn ne(&self, other: &&str) -> bool {
		self.as_str() != *other
	}
}

impl Eq for SmallStr {}

impl std::hash::Hash for SmallStr {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.as_str().hash(state)
	}
}


#[test]
fn test() {
	let s = "asdasdasdasdasdasdasdasdasdasdasdasdasdasdasdasdasdasdasdasdasdasdasd";
	assert_eq!(SmallStr::from_str(s), s);

	let s = "asd";
	assert_eq!(SmallStr::from_str(s), s);

	let s = "asdasdasdasdasdasdasdasd";
	assert_eq!(SmallStr::from_str(s), s);
}
