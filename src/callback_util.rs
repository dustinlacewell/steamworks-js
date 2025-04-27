use std::sync::mpsc::{channel, Receiver, Sender};

/// Generic utility to create a callback + channel pair for SWRS async-to-sync bridging for any owned result type.
///
/// - `T`: The type you want to receive in your task (e.g., WorkshopItemDetails)
/// - `Q`: The type received from the async result (must be Send + 'static)
/// - `convert`: A closure to convert from `Q` to `Option<T>`
///
/// Returns (rx, callback), where:
///   - `rx` is a std::sync::mpsc::Receiver<Option<T>>
///   - `callback` is a closure to pass directly to fetch
pub fn make_swrs_callback<T, Q, F>(
    convert: F,
) -> (
    std::sync::mpsc::Receiver<Option<T>>,
    impl FnOnce(Result<Q, steamworks::SteamError>),
)
where
    F: FnOnce(Q) -> Option<T> + Send + 'static,
    T: Send + 'static,
{
    let (tx, rx) = std::sync::mpsc::channel();
    let callback = move |result: Result<Q, steamworks::SteamError>| {
        let _ = tx.send(result.ok().and_then(|q| convert(q)));
    };
    (rx, callback)
}

pub fn make_swrs_callback2<T, E>() -> (std::sync::mpsc::Receiver<Option<T>>, impl FnOnce(Result<T, E>))
where
    T: Send + 'static,
    E: Send + Copy + 'static,
{
    let (tx, rx) = std::sync::mpsc::channel();
    let callback = move |result: Result<T, E>| {
        let _ = tx.send(result.ok());
    };
    (rx, callback)
}
