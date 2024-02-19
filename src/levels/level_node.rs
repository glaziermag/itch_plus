


// #[repr(transparent)]
// #[derive(Derivative)]
// #[derivative(Debug = "transparent")]
// pub struct RawNodePtr(pub mut Level);


// impl From<mut Level> for RawNodePtr {
//     fn from(value: mut Level) -> Self {
//         Self(value)
//     }
// }

// impl From<Level> for RawNodePtr {
//     fn from(value: Level) -> Self {
//         // Convert Level to raw pointer
//         let raw_ptr: mut Level = From::from(value);
//         // Convert raw pointer to RawNodePtr
//         From::from(raw_ptr)
//     }
// }

// impl From<RefMut<Level>> for RawNodePtr {
//     fn from(mut ref_mut: RefMut<Level>) -> Self {
//         unsafe {
//             let ptr: mut Level = &mut ref_mut as mut Level;
//             std::mem::forget(ref_mut); // Prevent RefMut from being dropped
//             RawNodePtr(ptr)
//         }
//     }
// }

// impl std::ops::Defor RawNodePtr {
//     type Target = Level;

//     fn deref(&self) -> &Self::Target {
//         unsafe { &self.0 }
//     }
// }

// impl Borrow<u64> for RawNodePtr {
//     fn borrow(&self) -> &u64 {
//         // SAFETY: Ensure this is safe, i.e., self.0 is a valid pointer
//         unsafe { &self.0).price }
//     }
// }

// impl Eq for RawNodePtr {}

// impl PartialEq for RawNodePtr {
//     fn eq(&self, other: &Self) -> bool {
//         let this = &self;
//         let other = &other;
//         this.price == other.price
//     }
// }

// impl PartialOrd for RawNodePtr {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         let this = &self;
//         let other = &other;
//         this.price.partial_cmp(&other.price)
//     }
// }

// impl Ord for RawNodePtr {
//     fn cmp(&self, other: &Self) -> Ordering {
//         let this = &self;
//         let other = &other;
//         this.price.cmp(&other.price)
//     }

//     fn max(self, other: Self) -> Self {
//         match self.partial_cmp(&other) {
//             Some(Ordering::Greater) | None => self,
//             _ => other,
//         }
//     }

//     fn min(self, other: Self) -> Self {
//         match self.partial_cmp(&other) {
//             Some(Ordering::Less) | None => self,
//             _ => other,
//         }
//     }

//     fn clamp(self, min: Self, max: Self) -> Self {
//         assert!(min.partial_cmp(&max).map_or(false, |ord| ord != Ordering::Greater));
//         if self.partial_cmp(&min).map_or(false, |ord| ord == Ordering::Less) {
//             min
//         } else if self.partial_cmp(&max).map_or(false, |ord| ord == Ordering::Greater) {
//             max
//         } else {
//             self
//         }
//     }
// }

// // Implement Drop if necessary
// impl Drop for RawNodePtr {
//     fn drop(&mut self) {
//         // Implement drop logic if needed
//     }
// }
