mod one;

#[cfg(feature = "sync")]
pub use one::sync::func_one as sync_one;

#[cfg(feature = "tokio")]
pub use one::tokio::func_one as async_one;
