/// Automatically defines the `CollectionRef::upcast_item_ref` function using the
/// covariance of the `ItemRef<'a>` type with regards to `'a`.
///
/// ## Example
///
/// ```
/// use cc_traits::{CollectionRef, covariant_item_ref};
///
/// impl<T> CollectionRef for Vec<T> {
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
/// use cc_traits::{CollectionMut, covariant_item_mut};
///
/// impl<T> CollectionMut for Vec<T> {
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
/// use cc_traits::{KeyedRef, covariant_key_ref};
///
/// impl<T> KeyedRef for HashMap<K, V> {
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
