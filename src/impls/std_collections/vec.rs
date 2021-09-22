use crate::{
	Capacity, Clear, Collection, CollectionMut, CollectionRef, Get, GetMut, Iter, IterMut, Len,
	PopBack, PushBack, Remove, Reserve, WithCapacity,
};

impl<T> Collection for Vec<T> {
	type Item = T;
}

impl<T> CollectionRef for Vec<T> {
	type ItemRef<'a>
	where
		Self: 'a,
	= &'a T;
}

impl<T> CollectionMut for Vec<T> {
	type ItemMut<'a>
	where
		Self: 'a,
	= &'a mut T;
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

impl<T> Get<usize> for Vec<T> {
	#[inline(always)]
	fn get(&self, index: usize) -> Option<&T> {
		self.as_slice().get(index)
	}
}

impl<T> GetMut<usize> for Vec<T> {
	#[inline(always)]
	fn get_mut(&mut self, index: usize) -> Option<&mut T> {
		self.as_mut_slice().get_mut(index)
	}
}

impl<T> Capacity for Vec<T> {
	#[inline(always)]
	fn capacity(&self) -> usize {
		self.capacity()
	}
}

impl<T> Reserve for Vec<T> {
	#[inline(always)]
	fn reserve(&mut self, additional: usize) {
		self.reserve(additional)
	}
}

impl<T> PushBack for Vec<T> {
	type Output = ();

	#[inline(always)]
	fn push_back(&mut self, t: T) {
		self.push(t)
	}
}

impl<T> PopBack for Vec<T> {
	#[inline(always)]
	fn pop_back(&mut self) -> Option<T> {
		self.pop()
	}
}

impl<T> Remove<usize> for Vec<T> {
	#[inline(always)]
	fn remove(&mut self, index: usize) -> Option<T> {
		if index < self.len() {
			Some(self.remove(index))
		} else {
			None
		}
	}
}

impl<T> Clear for Vec<T> {
	#[inline(always)]
	fn clear(&mut self) {
		self.clear()
	}
}

impl<T> Iter for Vec<T> {
	type Iter<'a>
	where
		T: 'a,
	= std::slice::Iter<'a, T>;

	#[inline(always)]
	fn iter(&self) -> Self::Iter<'_> {
		self.as_slice().iter()
	}
}

impl<T> IterMut for Vec<T> {
	type IterMut<'a>
	where
		T: 'a,
	= std::slice::IterMut<'a, T>;

	#[inline(always)]
	fn iter_mut(&mut self) -> Self::IterMut<'_> {
		self.as_mut_slice().iter_mut()
	}
}
