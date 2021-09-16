use crate::{
	Collection,
	CollectionRef,
	CollectionMut,
	Keyed,
	KeyedRef,
	Len,
	Get,
	GetMut,
	MapInsert,
	MapIter,
	MapIterMut,
	Remove,
	Clear
};
use json::{
	JsonValue,
	object::Object as JsonObject
};

impl Collection for JsonObject {
	type Item = JsonValue;
}

impl CollectionRef for JsonObject {
	type ItemRef<'r> = &'r JsonValue;
}

impl CollectionMut for JsonObject {
	type ItemMut<'r> = &'r mut JsonValue;
}

impl Keyed for JsonObject {
	type Key = String;
}

impl KeyedRef for JsonObject {
	type KeyRef<'r> = &'r str;
}

impl Len for JsonObject {
	fn len(&self) -> usize {
		self.len()
	}
}

impl<'a> Get<&'a str> for JsonObject {
	#[inline(always)]
	fn get(&self, key: &'a str) -> Option<&JsonValue> {
		self.get(key)
	}
}

impl<'a> GetMut<&'a str> for JsonObject {
	#[inline(always)]
	fn get_mut(&mut self, key: &'a str) -> Option<&mut JsonValue> {
		self.get_mut(key)
	}
}

impl<'a> MapInsert<&'a str> for JsonObject {
	type Output = ();

	#[inline(always)]
	fn insert(&mut self, key: &'a str, value: JsonValue) {
		self.insert(key, value)
	}
}

impl MapInsert<String> for JsonObject {
	type Output = ();

	#[inline(always)]
	fn insert(&mut self, key: String, value: JsonValue) {
		self.insert(key.as_str(), value)
	}
}

impl MapIter for JsonObject {
	type Iter<'a> = ::json::object::Iter<'a>;

	fn iter(&self) -> Self::Iter<'_> {
		self.iter()
	}
}

impl MapIterMut for JsonObject {
	type IterMut<'a> = ::json::object::IterMut<'a>;

	fn iter_mut(&mut self) -> Self::IterMut<'_> {
		self.iter_mut()
	}
}

impl<'a> Remove<&'a str> for JsonObject {
	#[inline(always)]
	fn remove(&mut self, key: &'a str) -> Option<JsonValue> {
		self.remove(key)
	}
}

impl Clear for JsonObject {
	#[inline(always)]
	fn clear(&mut self) {
		self.clear()
	}
}