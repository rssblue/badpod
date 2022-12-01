use std::{
    sync::mpsc::{self, RecvTimeoutError},
    thread,
    time::Duration,
};

pub fn from_str_exact<T: strum::IntoEnumIterator + std::fmt::Display>(s: &str) -> Option<T>
where
{
    T::iter().find(|variant| format!("{variant}") == s)
}

pub fn from_str_case_insensitive<T: strum::IntoEnumIterator + std::fmt::Display>(
    s: &str,
) -> Option<T>
where
{
    T::iter().find(|variant| format!("{variant}").to_lowercase() == s.to_lowercase())
}

#[derive(Debug)]
pub struct TimeoutError;

// https://stackoverflow.com/a/74234262/7027465
pub fn run_with_timeout<F, T>(f: F, timeout: Duration) -> Result<T, TimeoutError>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    let (tx, rx) = mpsc::channel();
    let _ = thread::spawn(move || {
        let result = f();
        if let Ok(()) = tx.send(result) {}
    });

    match rx.recv_timeout(timeout) {
        Ok(result) => Ok(result),
        Err(RecvTimeoutError::Timeout) => Err(TimeoutError),
        Err(RecvTimeoutError::Disconnected) => unreachable!(),
    }
}
