use slab::Slab;
use crate::{
	Collection,
	WithCapacity,
	Len,
	Capacity,
	Reserve,
	Get,
	GetMut,
	Insert,
	Remove,
	Clear
};

impl<T> Collection for Slab<T> {
	type Item = T;
}

impl<T> WithCapacity for Slab<T> {
	fn with_capacity(capacity: usize) -> Self {
		Slab::with_capacity(capacity)
	}
}

impl<T> Len for Slab<T> {
	fn len(&self) -> usize {
		self.len()
	}
}

impl<T> Capacity for Slab<T> {
	fn capacity(&self) -> usize {
		self.capacity()
	}
}

impl<T> Reserve for Slab<T> {
	fn reserve(&mut self, additional: usize) {
		self.reserve(additional)
	}
}

impl<T> Get<usize> for Slab<T> {
	fn get(&self, key: usize) -> Option<&Self::Item> {
		self.get(key)
	}
}

impl<T> GetMut<usize> for Slab<T> {
	fn get_mut(&mut self, key: usize) -> Option<&mut Self::Item> {
		self.get_mut(key)
	}
}

impl<T> Insert for Slab<T> {
	type Output = usize;

	fn insert(&mut self, element: T) -> usize {
		self.insert(element)
	}
}

impl<T> Remove<usize> for Slab<T> {
	fn remove(&mut self, key: usize) -> Option<T> {
		if self.contains(key) {
			Some(self.remove(key))
		} else {
			None
		}
	}
}

impl<T> Clear for Slab<T> {
	fn clear(&mut self) {
		self.clear()
	}
}