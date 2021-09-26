use crate::{
	Back, BackMut, Capacity, Clear, Collection, CollectionMut, CollectionRef, Front, FrontMut,
	Iter, IterMut, Len, PopBack, PushBack, Remove, Reserve, WithCapacity,
};
use smallvec::{Array, SmallVec};

impl<A: Array> Collection for SmallVec<A> {
	type Item = A::Item;
}

impl<A: Array> CollectionRef for SmallVec<A> {
	type ItemRef<'a>
	where
		Self: 'a,
	= &'a A::Item;
}

impl<A: Array> CollectionMut for SmallVec<A> {
	type ItemMut<'a>
	where
		Self: 'a,
	= &'a mut A::Item;
}

impl<A: Array> WithCapacity for SmallVec<A> {
	#[inline(always)]
	fn with_capacity(capacity: usize) -> Self {
		SmallVec::with_capacity(capacity)
	}
}

impl<A: Array> Len for SmallVec<A> {
	#[inline(always)]
	fn len(&self) -> usize {
		self.len()
	}

	#[inline(always)]
	fn is_empty(&self) -> bool {
		self.is_empty()
	}
}

impl<A: Array> Capacity for SmallVec<A> {
	#[inline(always)]
	fn capacity(&self) -> usize {
		self.capacity()
	}
}

impl<A: Array> Reserve for SmallVec<A> {
	#[inline(always)]
	fn reserve(&mut self, additional: usize) {
		self.reserve(additional)
	}
}

impl<A: Array> Back for SmallVec<A> {
	#[inline(always)]
	fn back(&self) -> Option<&A::Item> {
		self.last()
	}
}

impl<A: Array> BackMut for SmallVec<A> {
	#[inline(always)]
	fn back_mut(&mut self) -> Option<&mut A::Item> {
		self.last_mut()
	}
}

impl<A: Array> Front for SmallVec<A> {
	#[inline(always)]
	fn front(&self) -> Option<&A::Item> {
		self.first()
	}
}

impl<A: Array> FrontMut for SmallVec<A> {
	#[inline(always)]
	fn front_mut(&mut self) -> Option<&mut A::Item> {
		self.first_mut()
	}
}

impl<A: Array> PushBack for SmallVec<A> {
	type Output = ();

	#[inline(always)]
	fn push_back(&mut self, t: A::Item) {
		self.push(t)
	}
}

impl<A: Array> PopBack for SmallVec<A> {
	#[inline(always)]
	fn pop_back(&mut self) -> Option<A::Item> {
		self.pop()
	}
}

impl<A: Array> Remove<usize> for SmallVec<A> {
	#[inline(always)]
	fn remove(&mut self, index: usize) -> Option<A::Item> {
		if index < self.len() {
			Some(self.remove(index))
		} else {
			None
		}
	}
}

impl<A: Array> Clear for SmallVec<A> {
	#[inline(always)]
	fn clear(&mut self) {
		self.clear()
	}
}

impl<A: Array> Iter for SmallVec<A> {
	type Iter<'a>
	where
		A: 'a,
	= std::slice::Iter<'a, A::Item>;

	#[inline(always)]
	fn iter(&self) -> Self::Iter<'_> {
		self.as_slice().iter()
	}
}

impl<A: Array> IterMut for SmallVec<A> {
	type IterMut<'a>
	where
		A: 'a,
	= std::slice::IterMut<'a, A::Item>;

	#[inline(always)]
	fn iter_mut(&mut self) -> Self::IterMut<'_> {
		self.as_mut_slice().iter_mut()
	}
}
