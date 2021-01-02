#![feature(trait_alias)]

pub trait WithCapacity {
	fn with_capacity(capacity: usize) -> Self;
}

pub trait Len {
	fn len(&self) -> usize;

	fn is_empty(&self) -> bool {
		self.len() == 0
	}
}

pub trait Capacity {
	fn capacity(&self) -> usize;
}

pub trait Reserve {
	fn reserve(&mut self, additional: usize);
}

pub trait CapacityMut = Capacity + Reserve;

pub trait Get<T> {
	type Output;

	fn get(&self, index: T) -> Option<&Self::Output>;

	fn contains(&self, element: T) -> bool {
		self.get(element).is_some()
	}
}

pub trait GetMut<T> {
	type Output;

	fn get_mut(&mut self, index: T) -> Option<&mut Self::Output>;
}

pub trait Front {
	type Output;

	fn front(&self) -> &Self::Output;
}

pub trait Back {
	type Output;

	fn back(&self) -> &Self::Output;
}

pub trait FrontMut {
	type Output;

	fn front(&mut self) -> &mut Self::Output;
}

pub trait BackMut {
	type Output;

	fn back(&mut self) -> &mut Self::Output;
}

pub trait Insert<T> {
	type Output;

	fn insert(&mut self, element: T) -> Self::Output;
}

pub trait PushFront<T> {
	type Output;

	fn push_front(&mut self, element: T) -> Self::Output;
}

pub trait PushBack<T> {
	type Output;

	fn push_back(&mut self, element: T) -> Self::Output;
}

pub trait Remove<T> {
	type Output;

	fn remove(&mut self, element: T) -> Self::Output;
}

pub trait PopFront<T> {
	fn pop_front(&mut self) -> Option<T>;
}

pub trait PopBack<T> {
	fn pop_back(&mut self) -> Option<T>;
}

pub trait Clear {
	fn clear(&mut self);
}

pub trait Stack<T> = Len + Back<Output=T>;
pub trait StackMut<T> = Len + BackMut<Output=T> + PushBack<T> + PopBack<T>;

pub trait Vec<T> = Stack<T> + Get<usize, Output=T>;
pub trait VecMut<T> = Vec<T> + StackMut<T> + GetMut<usize, Output=T>;

pub trait Deque<T> = Stack<T> + Front<Output=T>;
pub trait DequeMut<T> = StackMut<T> + FrontMut<Output=T> + PushFront<T> + PopFront<T>;

pub trait VecDeque<T> = Deque<T> + Get<usize, Output=T>;
pub trait VecDequeMut<T> = VecDeque<T> + DequeMut<T> + GetMut<usize, Output=T>;

pub trait Set<T> = Len + for<'a> Get<&'a T>;
pub trait SetMut<T> = Set<T> + Insert<T, Output=bool> + for<'a> Remove<&'a T, Output=bool>;

pub trait Map<K, V> = Len + for<'a> Get<&'a K, Output=V>;
pub trait MapMut<K, V> = Map<K, V> + for<'a> GetMut<&'a K, Output=V>;

pub trait Slab<T> = Len + Get<usize, Output=T>;
pub trait SlabMut<T> = Slab<T> + Insert<T, Output=usize> + Remove<usize, Output=Option<T>>;