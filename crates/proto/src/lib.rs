pub mod customer;
pub mod manager;
pub mod order;
pub mod organization;
pub mod product;
pub mod storage;
pub mod warehouse;

pub(crate) fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    t == &T::default()
}
