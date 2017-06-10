#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T: ?Sized> !Send for MutexGuard<'a, T> { }
#[stable(feature = "mutexguard", since = "1.18.0")]
unsafe impl<'a, T: ?Sized + Sync> Sync for MutexGuard<'a, T> { }
