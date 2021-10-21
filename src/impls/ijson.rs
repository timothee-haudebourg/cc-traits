use crate::{
	Capacity, Clear, Collection, CollectionMut, CollectionRef, Get, GetMut, Iter, IterMut, Keyed,
	KeyedRef, Len, MapInsert, MapIter, MapIterMut, PopBack, PushBack, Remove, Reserve,
	WithCapacity,
};
use ijson::{IArray, IObject, IString, IValue};

impl Collection for IObject {
	type Item = IValue;
}

impl CollectionRef for IObject {
	type ItemRef<'a> = &'a IValue;

	crate::covariant_item_ref!();
}

impl CollectionMut for IObject {
	type ItemMut<'a> = &'a mut IValue;

	crate::covariant_item_mut!();
}

impl Keyed for IObject {
	type Key = IString;
}

impl KeyedRef for IObject {
	type KeyRef<'a> = &'a IString;

	crate::covariant_key_ref!();
}

impl Len for IObject {
	#[inline(always)]
	fn len(&self) -> usize {
		self.len()
	}

	#[inline(always)]
	fn is_empty(&self) -> bool {
		self.is_empty()
	}
}

impl MapIter for IObject {
	type Iter<'a> = ijson::object::Iter<'a>;

	#[inline(always)]
	fn iter(&self) -> Self::Iter<'_> {
		self.iter()
	}
}

impl MapIterMut for IObject {
	type IterMut<'a> = ijson::object::IterMut<'a>;

	#[inline(always)]
	fn iter_mut(&mut self) -> Self::IterMut<'_> {
		self.iter_mut()
	}
}

impl<Q: ijson::object::ObjectIndex> Get<Q> for IObject {
	#[inline(always)]
	fn get(&self, q: Q) -> Option<&IValue> {
		self.get(q)
	}
}

impl<Q: ijson::object::ObjectIndex> GetMut<Q> for IObject {
	#[inline(always)]
	fn get_mut(&mut self, q: Q) -> Option<&mut IValue> {
		self.get_mut(q)
	}
}

impl MapInsert<IString> for IObject {
	type Output = Option<IValue>;

	#[inline(always)]
	fn insert(&mut self, key: IString, value: IValue) -> Option<IValue> {
		self.insert(key, value)
	}
}

impl<Q: ijson::object::ObjectIndex> Remove<Q> for IObject {
	#[inline(always)]
	fn remove(&mut self, key: Q) -> Option<IValue> {
		self.remove(key)
	}
}

impl Clear for IObject {
	#[inline(always)]
	fn clear(&mut self) {
		self.clear()
	}
}

impl Collection for IArray {
	type Item = IValue;
}

impl CollectionRef for IArray {
	type ItemRef<'a>
	where
		Self: 'a,
	= &'a IValue;

	crate::covariant_item_ref!();
}

impl CollectionMut for IArray {
	type ItemMut<'a>
	where
		Self: 'a,
	= &'a mut IValue;

	crate::covariant_item_mut!();
}

impl WithCapacity for IArray {
	#[inline(always)]
	fn with_capacity(capacity: usize) -> Self {
		Self::with_capacity(capacity)
	}
}

impl Len for IArray {
	#[inline(always)]
	fn len(&self) -> usize {
		self.len()
	}

	#[inline(always)]
	fn is_empty(&self) -> bool {
		self.is_empty()
	}
}

impl Get<usize> for IArray {
	#[inline(always)]
	fn get(&self, index: usize) -> Option<&IValue> {
		self.as_slice().get(index)
	}
}

impl GetMut<usize> for IArray {
	#[inline(always)]
	fn get_mut(&mut self, index: usize) -> Option<&mut IValue> {
		self.as_mut_slice().get_mut(index)
	}
}

impl Capacity for IArray {
	#[inline(always)]
	fn capacity(&self) -> usize {
		self.capacity()
	}
}

impl Reserve for IArray {
	#[inline(always)]
	fn reserve(&mut self, additional: usize) {
		self.reserve(additional)
	}
}

impl PushBack for IArray {
	type Output = ();

	#[inline(always)]
	fn push_back(&mut self, t: IValue) {
		self.push(t)
	}
}

impl PopBack for IArray {
	#[inline(always)]
	fn pop_back(&mut self) -> Option<IValue> {
		self.pop()
	}
}

impl Remove<usize> for IArray {
	#[inline(always)]
	fn remove(&mut self, index: usize) -> Option<IValue> {
		if index < self.len() {
			self.remove(index)
		} else {
			None
		}
	}
}

impl Clear for IArray {
	#[inline(always)]
	fn clear(&mut self) {
		self.clear()
	}
}

impl Iter for IArray {
	type Iter<'a> = std::slice::Iter<'a, IValue>;

	#[inline(always)]
	fn iter(&self) -> Self::Iter<'_> {
		self.as_slice().iter()
	}
}

impl IterMut for IArray {
	type IterMut<'a> = std::slice::IterMut<'a, IValue>;

	#[inline(always)]
	fn iter_mut(&mut self) -> Self::IterMut<'_> {
		self.as_mut_slice().iter_mut()
	}
}
