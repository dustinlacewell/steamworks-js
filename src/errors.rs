use napi::bindgen_prelude::*;
use steamworks::CreateQueryError;

// No need for a custom struct, just map to standard JS Error with .message

// Converts a steamworks::SteamError into a napi::Error (JS exception)
pub fn steam_error_to_napi(err: steamworks::SteamError) -> napi::Error {
    let msg = match err {
        steamworks::SteamError::Generic => "Steamworks: Generic error".to_string(),
        steamworks::SteamError::NoConnection => "Steamworks: No connection to Steam".to_string(),
        steamworks::SteamError::InvalidPassword => "Steamworks: Invalid password or ticket".to_string(),
        steamworks::SteamError::InvalidParameter => "Steamworks: Invalid parameter".to_string(),
        steamworks::SteamError::FileNotFound => "Steamworks: File not found".to_string(),
        steamworks::SteamError::Busy => "Steamworks: Method busy".to_string(),
        steamworks::SteamError::InvalidState => "Steamworks: Invalid state".to_string(),
        steamworks::SteamError::InvalidName => "Steamworks: Invalid name".to_string(),
        steamworks::SteamError::InvalidEmail => "Steamworks: Invalid email".to_string(),
        // Add more variants as needed for coverage
        _ => format!("Steamworks: Unknown error: {:?}", err),
    };
    napi::Error::from_reason(msg)
}

// Converts a CreateQueryError into a napi::Error (JS exception)
pub fn create_query_error_to_napi(err: CreateQueryError) -> napi::Error {
    napi::Error::from_reason(err.to_string())
}
