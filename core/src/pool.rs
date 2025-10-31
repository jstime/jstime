use std::cell::RefCell;
use std::rc::Rc;

/// A generic object pool for reusing allocations.
/// This reduces allocation overhead by recycling objects instead of dropping them.
///
/// The pool is designed for per-isolate use and uses interior mutability
/// via RefCell since V8 isolates are single-threaded.
pub(crate) struct Pool<T> {
    objects: RefCell<Vec<T>>,
    max_capacity: usize,
}

impl<T> Pool<T> {
    /// Create a new pool with a maximum capacity.
    ///
    /// The pool will not grow beyond `max_capacity` to avoid unbounded memory growth.
    /// Typical usage would set this to a reasonable limit (e.g., 100-200 objects).
    pub(crate) fn new(max_capacity: usize) -> Self {
        Self {
            objects: RefCell::new(Vec::with_capacity(std::cmp::min(max_capacity, 16))),
            max_capacity,
        }
    }

    /// Get an object from the pool, or create a new one using the provided factory.
    ///
    /// If the pool is empty, the factory function is called to create a new object.
    #[inline]
    pub(crate) fn get<F>(&self, factory: F) -> T
    where
        F: FnOnce() -> T,
    {
        self.objects.borrow_mut().pop().unwrap_or_else(factory)
    }

    /// Return an object to the pool for reuse.
    ///
    /// If the pool is at max capacity, the object is dropped instead.
    /// The caller should ensure the object is in a reusable state (e.g., cleared).
    #[inline]
    pub(crate) fn put(&self, object: T) {
        let mut objects = self.objects.borrow_mut();
        if objects.len() < self.max_capacity {
            objects.push(object);
        }
        // Otherwise drop the object
    }

    /// Get the current number of objects in the pool.
    #[allow(dead_code)]
    pub(crate) fn len(&self) -> usize {
        self.objects.borrow().len()
    }

    /// Check if the pool is empty.
    #[allow(dead_code)]
    pub(crate) fn is_empty(&self) -> bool {
        self.objects.borrow().is_empty()
    }
}

/// A handle to a pooled vector that automatically returns it to the pool when dropped.
#[allow(dead_code)]
pub(crate) struct PooledVec<T> {
    vec: Option<Vec<T>>,
    pool: Rc<Pool<Vec<T>>>,
}

#[allow(dead_code)]
impl<T> PooledVec<T> {
    /// Create a new pooled vector from a pool.
    pub(crate) fn new(pool: Rc<Pool<Vec<T>>>) -> Self {
        let mut vec = pool.get(Vec::new);
        vec.clear(); // Ensure it's empty
        Self {
            vec: Some(vec),
            pool,
        }
    }

    /// Get a mutable reference to the inner vector.
    #[inline]
    pub(crate) fn as_mut(&mut self) -> &mut Vec<T> {
        self.vec.as_mut().unwrap()
    }

    /// Take ownership of the inner vector, preventing it from being returned to the pool.
    pub(crate) fn take(mut self) -> Vec<T> {
        self.vec.take().unwrap()
    }
}

impl<T> Drop for PooledVec<T> {
    fn drop(&mut self) {
        if let Some(mut vec) = self.vec.take() {
            vec.clear();
            self.pool.put(vec);
        }
    }
}

impl<T> std::ops::Deref for PooledVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        self.vec.as_ref().unwrap()
    }
}

impl<T> std::ops::DerefMut for PooledVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.vec.as_mut().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_get_and_put() {
        let pool = Pool::new(10);

        // Get from empty pool creates new object
        let vec = pool.get(Vec::new);
        assert_eq!(vec.len(), 0);

        // Put back
        pool.put(vec);
        assert_eq!(pool.len(), 1);

        // Get returns the same object
        let mut vec = pool.get(Vec::new);
        vec.push(42);
        pool.put(vec);

        let vec = pool.get(Vec::new);
        assert_eq!(vec.len(), 1);
        assert_eq!(vec[0], 42);
    }

    #[test]
    fn test_pool_max_capacity() {
        let pool = Pool::new(2);

        pool.put(vec![1]);
        pool.put(vec![2]);
        pool.put(vec![3]); // Should be dropped

        assert_eq!(pool.len(), 2);
    }

    #[test]
    fn test_pooled_vec() {
        let pool = Rc::new(Pool::new(10));

        {
            let mut vec = PooledVec::new(pool.clone());
            vec.push(1);
            vec.push(2);
            assert_eq!(vec.len(), 2);
        } // vec returned to pool here

        assert_eq!(pool.len(), 1);

        // Next allocation reuses the vec
        let vec = PooledVec::new(pool.clone());
        assert_eq!(vec.len(), 0); // Should be cleared
    }

    #[test]
    fn test_pooled_vec_take() {
        let pool = Rc::new(Pool::new(10));

        let mut vec = PooledVec::new(pool.clone());
        vec.push(1);

        let inner = vec.take();
        assert_eq!(inner.len(), 1);

        // Pool should still be empty since we took ownership
        assert_eq!(pool.len(), 0);
    }

    #[test]
    fn test_pool_concurrent_operations() {
        let pool = Rc::new(Pool::new(5));

        // Simulate multiple allocations and deallocations
        for _ in 0..10 {
            let mut vec = PooledVec::new(pool.clone());
            vec.push(1);
            vec.push(2);
            vec.push(3);
        }

        // Pool should have some objects cached (up to max_capacity)
        assert!(pool.len() <= 5);
    }
}
