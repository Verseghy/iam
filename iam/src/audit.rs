#[macro_export]
macro_rules! audit {
    ($($tt:tt)*) => {
        ::tracing::trace!(
            target: "audit",
            parent: None,
            $($tt)*
        )
    }
}
