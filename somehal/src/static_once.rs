use core::{cell::UnsafeCell, ops::Deref, sync::atomic::AtomicBool};

pub struct StaticOnce<T> {
    value: UnsafeCell<Option<T>>,
    initialized: AtomicBool,
}

unsafe impl<T> Send for StaticOnce<T> {}
unsafe impl<T> Sync for StaticOnce<T> {}

impl<T> StaticOnce<T> {
    pub const fn new() -> Self {
        Self {
            value: UnsafeCell::new(None),
            initialized: AtomicBool::new(false),
        }
    }

    pub const fn with_default(value: T) -> Self {
        Self {
            value: UnsafeCell::new(Some(value)),
            initialized: AtomicBool::new(false),
        }
    }

    pub fn init(&self, value: T) {
        if self
            .initialized
            .swap(true, core::sync::atomic::Ordering::Release)
        {
            panic!("StaticOnce has already been initialized");
        }
        unsafe {
            *self.value.get() = Some(value);
        }
    }

    pub unsafe fn get(&self) -> *mut T {
        let ptr = self.value.get();
        unsafe {
            let value = &mut *ptr;
            value.as_mut().expect("StaticOnce not initialized")
        }
    }
}

impl<T> Deref for StaticOnce<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let r = unsafe { &*self.value.get() };
        r.as_ref().expect("StaticOnce not initialized")
    }
}
