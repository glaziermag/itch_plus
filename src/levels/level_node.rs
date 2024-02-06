use std::{cmp::Ordering, cell::RefMut, borrow::Borrow};

use derivative::Derivative;

use super::{level::Level};


#[repr(transparent)]
#[derive(Derivative)]
#[derivative(Debug = "transparent")]
pub struct RawNodePtr<'a, R: Ref<'a>>(pub *mut Level<'a, R>);


impl<'a, R: Ref<'a>> From<*mut Level<'a, R>> for RawNodePtr<'a, R> {
    fn from(value: *mut Level<'a, R>) -> Self {
        Self(value)
    }
}

impl<'a, R: Ref<'a>> From<Level<'a, R>> for RawNodePtr<'a, R> {
    fn from(value: Level<'a, R>) -> Self {
        // Convert Level to raw pointer
        let raw_ptr: *mut Level<'a, R> = From::from(value);
        // Convert raw pointer to RawNodePtr
        From::from(raw_ptr)
    }
}

impl<'a, R: Ref<'a>> From<RefMut<'a, Level<'a, R>>> for RawNodePtr<'a, R> {
    fn from(mut ref_mut: RefMut<'a, Level<'a, R>>) -> Self {
        unsafe {
            let ptr: *mut Level<'a, R> = &mut *ref_mut as *mut Level<'a, R>;
            std::mem::forget(ref_mut); // Prevent RefMut from being dropped
            RawNodePtr(ptr)
        }
    }
}

impl<'a, R: Ref<'a>> std::ops::Deref for RawNodePtr<'a, R> {
    type Target = Level<'a, R>;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

impl<'a, R: Ref<'a>> Borrow<u64> for RawNodePtr<'a, R> {
    fn borrow(&self) -> &u64 {
        // SAFETY: Ensure this is safe, i.e., self.0 is a valid pointer
        unsafe { &(*self.0).price }
    }
}

impl<'a, R: Ref<'a>> Eq for RawNodePtr<'a, R> {}

impl<'a, R: Ref<'a>> PartialEq for RawNodePtr<'a, R> {
    fn eq(&self, other: &Self) -> bool {
        let this = &**self;
        let other = &**other;
        this.price == other.price
    }
}

impl<'a, R: Ref<'a>> PartialOrd for RawNodePtr<'a, R> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let this = &**self;
        let other = &**other;
        this.price.partial_cmp(&other.price)
    }
}

impl<'a, R: Ref<'a>> Ord for RawNodePtr<'a, R> {
    fn cmp(&self, other: &Self) -> Ordering {
        let this = &**self;
        let other = &**other;
        this.price.cmp(&other.price)
    }

    fn max(self, other: Self) -> Self {
        match self.partial_cmp(&other) {
            Some(Ordering::Greater) | None => self,
            _ => other,
        }
    }

    fn min(self, other: Self) -> Self {
        match self.partial_cmp(&other) {
            Some(Ordering::Less) | None => self,
            _ => other,
        }
    }

    fn clamp(self, min: Self, max: Self) -> Self {
        assert!(min.partial_cmp(&max).map_or(false, |ord| ord != Ordering::Greater));
        if self.partial_cmp(&min).map_or(false, |ord| ord == Ordering::Less) {
            min
        } else if self.partial_cmp(&max).map_or(false, |ord| ord == Ordering::Greater) {
            max
        } else {
            self
        }
    }
}

// Implement Drop if necessary
impl<'a, R: Ref<'a>> Drop for RawNodePtr<'a, R> {
    fn drop(&mut self) {
        // Implement drop logic if needed
    }
}
