use crate::{
    Back, BackMut, Capacity, Clear, Collection, CollectionMut, CollectionRef, Front, FrontMut,
    Len, PopBack, PopFront, PushBack, PushFront, Reserve, WithCapacity,
};
use std::collections::VecDeque;

impl<T> Collection for VecDeque<T> {
    type Item = T;
}

impl<T> CollectionRef for VecDeque<T> {
    type ItemRef<'a>
    where
        Self: 'a,
    = &'a T;

    crate::covariant_item_ref!();
}

impl<T> CollectionMut for VecDeque<T> {
    type ItemMut<'a>
    where
        Self: 'a,
    = &'a mut T;

    crate::covariant_item_mut!();
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
    #[inline(always)]
    fn capacity(&self) -> usize {
        self.capacity()
    }
}

impl<T> Reserve for VecDeque<T> {
    #[inline(always)]
    fn reserve(&mut self, additional: usize) {
        self.reserve(additional)
    }
}

impl<T> Front for VecDeque<T> {
    #[inline(always)]
    fn front(&self) -> Option<&T> {
        self.front()
    }
}

impl<T> FrontMut for VecDeque<T> {
    #[inline(always)]
    fn front_mut(&mut self) -> Option<&mut T> {
        self.front_mut()
    }
}

impl<T> Back for VecDeque<T> {
    #[inline(always)]
    fn back(&self) -> Option<&T> {
        self.back()
    }
}

impl<T> BackMut for VecDeque<T> {
    #[inline(always)]
    fn back_mut(&mut self) -> Option<&mut T> {
        self.back_mut()
    }
}

impl<T> PushFront for VecDeque<T> {
    type Output = ();

    #[inline(always)]
    fn push_front(&mut self, t: T) {
        self.push_front(t)
    }
}

impl<T> PushBack for VecDeque<T> {
    type Output = ();

    #[inline(always)]
    fn push_back(&mut self, t: T) {
        self.push_back(t)
    }
}

impl<T> PopFront for VecDeque<T> {
    #[inline(always)]
    fn pop_front(&mut self) -> Option<T> {
        self.pop_front()
    }
}

impl<T> PopBack for VecDeque<T> {
    #[inline(always)]
    fn pop_back(&mut self) -> Option<T> {
        self.pop_back()
    }
}

impl<T> Clear for VecDeque<T> {
    #[inline(always)]
    fn clear(&mut self) {
        self.clear()
    }
}
