use crate::token::Handle;

/// Backing storage for resources
///
/// Since all resources in a value_store should have the same lifetime (think tokens from a source
/// file), we don't need to support individual resource removal, only full cleanup.
///
/// It is assumed that all handles are valid, since the only way to create a handle is by calling
/// `insert()`, and individual resources are never freed. Therefore `get()` doesn't return an
/// `Option<V>`, but a `V` directly.
///
/// `ValueStore`s only need to store data up to `HANDLE_MAX`, if there are exactly `HANDLE_MAX`
/// elements, then `.insert()` will panic.
pub trait ValueStore<K: Handle<V> + Copy, V> {
    /// Initialize a `ValueStore` with default capacity
    fn new() -> Self;

    /// Initialize a `ValueStore` with provided capacity.
    fn with_capacity(cap: usize) -> Self;

    /// Insert a `Value` and get a `Handle` to it back
    fn insert(&mut self, value: V) -> K;

    /// Get a `Value` from a handle. If handle is invalid this function panics
    fn get(&mut self, key: &K) -> &V;

    fn values(&self) -> impl Iterator;

    /// Clear everything. This `ValueStore` can then be reused with new `Value`s.
    fn clear(&mut self);
}
