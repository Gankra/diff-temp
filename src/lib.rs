#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T: ?Sized> !Send for MutexGuard<'a, T> { }
