use std::collections::VecDeque;
use crate::{
	Collection,
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
	Clear
};

impl<T> Collection for VecDeque<T> {
	type Item = T;
}

impl<T> WithCapacity for VecDeque<T> {
	#[inline(always)]
	fn with_capacity(capacity: usize) -> Self {
		VecDeque::with_capacity(capacity)
	}
}

impl<T> Len for VecDeque<T> {
	#[inline(always)]
	fn len(&self) -> usize {
		self.len()
	}

	#[inline(always)]
	fn is_empty(&self) -> bool {
		self.is_empty()
	}
}

impl<T> Capacity for VecDeque<T> {
	fn capacity(&self) -> usize {
		self.capacity()
	}
}

impl<T> Reserve for VecDeque<T> {
	fn reserve(&mut self, additional: usize) {
		self.reserve(additional)
	}
}

impl<T> Back for VecDeque<T> {
	fn back(&self) -> Option<&T> {
		self.back()
	}
}

impl<T> BackMut for VecDeque<T> {
	fn back_mut(&mut self) -> Option<&mut T> {
		self.back_mut()
	}
}

impl<T> Front for VecDeque<T> {
	fn front(&self) -> Option<&T> {
		self.front()
	}
}

impl<T> FrontMut for VecDeque<T> {
	fn front_mut(&mut self) -> Option<&mut T> {
		self.front_mut()
	}
}

impl<T> PushBack for VecDeque<T> {
	type Output = ();

	fn push_back(&mut self, t: T) {
		self.push_back(t)
	}
}

impl<T> PopBack for VecDeque<T> {
	fn pop_back(&mut self) -> Option<T> {
		self.pop_back()
	}
}

impl<T> Clear for VecDeque<T> {
	fn clear(&mut self) {
		self.clear()
	}
}