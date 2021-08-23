use crate::{
	Collection,
	Keyed,
	Len,
	Get,
	GetMut,
	MapInsert,
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

impl Keyed for JsonObject {
	type Key = String;
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