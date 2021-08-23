use std::borrow::Borrow;
use std::hash::Hash;
use crate::{
	Collection,
	Keyed,
	WithCapacity,
	Len,
	Get,
	GetMut,
	MapInsert,
	Remove,
	Clear
};

impl Collection for serde_json::Map<String, serde_json::Value> {
	type Item = serde_json::Value;
}

impl Keyed for serde_json::Map<String, serde_json::Value> {
	type Key = String;
}

impl WithCapacity for serde_json::Map<String, serde_json::Value> {
	fn with_capacity(capacity: usize) -> Self {
		Self::with_capacity(capacity)
	}
}

impl Len for serde_json::Map<String, serde_json::Value> {
	fn len(&self) -> usize {
		self.len()
	}
}

impl<'a, Q> Get<&'a Q> for serde_json::Map<String, serde_json::Value> where String: Borrow<Q>, Q: Eq + Ord + Hash + ?Sized {
	#[inline(always)]
	fn get(&self, key: &'a Q) -> Option<&serde_json::Value> {
		self.get(key)
	}
}

impl<'a, Q> GetMut<&'a Q> for serde_json::Map<String, serde_json::Value> where String: Borrow<Q>, Q: Eq + Ord + Hash + ?Sized {
	#[inline(always)]
	fn get_mut(&mut self, key: &'a Q) -> Option<&mut serde_json::Value> {
		self.get_mut(key)
	}
}

impl MapInsert<String> for serde_json::Map<String, serde_json::Value> {
	type Output = Option<serde_json::Value>;

	#[inline(always)]
	fn insert(&mut self, key: String, value: serde_json::Value) -> Option<serde_json::Value> {
		self.insert(key, value)
	}
}

impl<'a, Q> Remove<&'a Q> for serde_json::Map<String, serde_json::Value> where String: Borrow<Q>, Q: Eq + Ord + Hash + ?Sized {
	#[inline(always)]
	fn remove(&mut self, key: &'a Q) -> Option<serde_json::Value> {
		self.remove(key)
	}
}

impl Clear for serde_json::Map<String, serde_json::Value> {
	#[inline(always)]
	fn clear(&mut self) {
		self.clear()
	}
}