#[cfg(feature = "chrono")]
pub(crate) fn current_time() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc::now()
}

#[cfg(all(not(target_family = "wasm"), not(feature = "chrono")))]
pub(crate) fn current_time() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

#[cfg(all(target_family = "wasm", not(feature = "chrono")))]
pub(crate) fn current_time() -> i64 {
    js_sys::Date::now() as i64
}
