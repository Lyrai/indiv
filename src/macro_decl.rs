#[macro_export]
macro_rules! mutex {
    ($mutex: expr) => { $mutex.lock().unwrap() };
}