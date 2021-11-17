/// Automatically defines the `CollectionRef::upcast_item_ref` function using the
/// covariance of the `ItemRef<'a>` type with regards to `'a`.
///
/// ## Example
///
/// ```
/// #![feature(generic_associated_types)]
/// use cc_traits::{Collection, CollectionRef, covariant_item_ref};
///
/// pub struct MyVec<T>(Vec<T>);
///
/// impl<T> Collection for MyVec<T> {
///   type Item = T;
/// }
///
/// impl<T> CollectionRef for MyVec<T> {
///   type ItemRef<'a>
///   where
///     Self: 'a,
///   = &'a T;
///
///   covariant_item_ref!();
/// }
/// ```
#[macro_export]
macro_rules! covariant_item_ref {
	() => {
		fn upcast_item_ref<'short, 'long: 'short>(
			r: Self::ItemRef<'long>,
		) -> Self::ItemRef<'short> {
			r
		}
	};
}

/// Automatically defines the `CollectionMut::upcast_item_mut` function using the
/// covariance of the `ItemMut<'a>` type with regards to `'a`.
///
/// ## Example
///
/// ```
/// #![feature(generic_associated_types)]
/// use cc_traits::{Collection, CollectionMut, covariant_item_mut};
///
/// pub struct MyVec<T>(Vec<T>);
///
/// impl<T> Collection for MyVec<T> {
///   type Item = T;
/// }
///
/// impl<T> CollectionMut for MyVec<T> {
///   type ItemMut<'a>
///   where
///     Self: 'a,
///   = &'a mut T;
///
///   covariant_item_mut!();
/// }
/// ```
#[macro_export]
macro_rules! covariant_item_mut {
	() => {
		fn upcast_item_mut<'short, 'long: 'short>(
			r: Self::ItemMut<'long>,
		) -> Self::ItemMut<'short> {
			r
		}
	};
}

/// Automatically defines the `KeyedRef::upcast_item_ref` function using the
/// covariance of the `KeyRef<'a>` type with regards to `'a`.
///
/// ## Example
///
/// ```
/// #![feature(generic_associated_types)]
/// use cc_traits::{Collection, Keyed, KeyedRef, covariant_key_ref};
///
/// pub struct MyMap<K, V>(std::collections::HashMap<K, V>);
///
/// impl<K, V> Collection for MyMap<K, V> {
///   type Item = V;
/// }
///
/// impl<K, V> Keyed for MyMap<K, V> {
///   type Key = K;
/// }
///
/// impl<K, V> KeyedRef for MyMap<K, V> {
///   type KeyRef<'a>
///   where
///     Self: 'a,
///   = &'a K;
///
///   covariant_key_ref!();
/// }
/// ```
#[macro_export]
macro_rules! covariant_key_ref {
	() => {
		fn upcast_key_ref<'short, 'long: 'short>(r: Self::KeyRef<'long>) -> Self::KeyRef<'short> {
			r
		}
	};
}
