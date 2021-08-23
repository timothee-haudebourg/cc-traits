use crate::{
	Collection,
	CollectionRef,
	CollectionMut,
	WithCapacity,
	Len,
	Capacity,
	Reserve,
	Back,
	BackMut,
	Front,
	FrontMut,
	PushBack,
	PopBack,
	Remove,
	Clear
};

impl<T> Collection for Vec<T> {
	type Item = T;
}

impl<T> CollectionRef for Vec<T> {
	type ItemRef<'a> where Self: 'a = &'a T;
}

impl<T> CollectionMut for Vec<T> {
	type ItemMut<'a> where Self: 'a = &'a mut T;
}

impl<T> WithCapacity for Vec<T> {
	#[inline(always)]
	fn with_capacity(capacity: usize) -> Self {
		Vec::with_capacity(capacity)
	}
}

impl<T> Len for Vec<T> {
	#[inline(always)]
	fn len(&self) -> usize {
		self.len()
	}

	#[inline(always)]
	fn is_empty(&self) -> bool {
		self.is_empty()
	}
}

impl<T> Capacity for Vec<T> {
	fn capacity(&self) -> usize {
		self.capacity()
	}
}

impl<T> Reserve for Vec<T> {
	fn reserve(&mut self, additional: usize) {
		self.reserve(additional)
	}
}

impl<T> Back for Vec<T> {
	fn back(&self) -> Option<&T> {
		self.last()
	}
}

impl<T> BackMut for Vec<T> {
	fn back_mut(&mut self) -> Option<&mut T> {
		self.last_mut()
	}
}

impl<T> Front for Vec<T> {
	fn front(&self) -> Option<&T> {
		self.first()
	}
}

impl<T> FrontMut for Vec<T> {
	fn front_mut(&mut self) -> Option<&mut T> {
		self.first_mut()
	}
}

impl<T> PushBack for Vec<T> {
	type Output<'a> where Self: 'a = ();

	fn push_back(&mut self, t: T) {
		self.push(t)
	}
}

impl<T> PopBack for Vec<T> {
	fn pop_back(&mut self) -> Option<T> {
		self.pop()
	}
}

impl<T> Remove<usize> for Vec<T> {
	fn remove(&mut self, index: usize) -> Option<T> {
		if index < self.len() {
			Some(self.remove(index))
		} else {
			None
		}
	}
}

impl<T> Clear for Vec<T> {
	fn clear(&mut self) {
		self.clear()
	}
}