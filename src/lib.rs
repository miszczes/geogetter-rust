//! # geogetter
//!
//! `geogetter` is a simple library that connects to the OpenStreetMap API to retrive all the information about the
//! given string

mod url_encoder;

pub use reqwest::header::USER_AGENT;
pub use serde::{Deserialize, Serialize};

/// Represents the address structure containing administrative, town, county, state, postcode, country, and country code.
#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    pub administrative: Option<String>,
    pub town: Option<String>,
    pub county: Option<String>,
    pub state: Option<String>,
    pub postcode: Option<String>,
    pub country: Option<String>,
    pub country_code: Option<String>,
}

/// Represents the location structure containing latitude, longitude, address type, name, display name, address, and bounding box.
#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub lat: String,
    pub lon: String,
    pub addresstype: Option<String>,
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub address: Address,
    pub boundingbox: Vec<String>,
}

/// Asynchronously retrieves location data from OpenStreetMap based on the input string.
///
/// # Arguments
///
/// * `input_str` - A string representing the location to search for.
///
/// # Returns
///
/// A Result containing a vector of Location structs if successful, or a Box<dyn std::error::Error> if an error occurs.
///
/// # Examples
///
/// ```
/// #[tokio::main]
/// async fn main() {
///     let input_str = "1600 Amphitheatre Parkway, Mountain View, CA";
///     
///     match geogetter::get_location(input_str).await {
///         Ok(locations) => {
///             if locations.is_empty() {
///                 println!("No location found for the input.");
///             } else {
///                 println!("Location found:");
///                 for location in locations {
///                     println!("Name: {}", location.name.unwrap_or_else(|| "Unknown".to_string()));
///                     println!("Address: {}", location.display_name.unwrap_or_else(|| "Unknown".to_string()));
///                     // Print other relevant information if needed
///                 }
///             }
///         }
///         Err(err) => {
///             eprintln!("Error occurred: {}", err);
///         }
///     }
/// }
/// ```

pub async fn get_location(input_str: &str) -> Result<Vec<Location>, Box<dyn std::error::Error>> {
    let url = format!(
        "https://nominatim.openstreetmap.org/search?addressdetails=1&q={}&format=jsonv2&limit=1",
        url_encoder::url_encoder::encode_str(input_str, true)
    );

    let http_resp = reqwest::Client::new()
        .get(&url)
        .header(USER_AGENT, "Geogetter Rust library")
        .send()
        .await?
        .text()
        .await?;
    let res_json: Vec<Location> = serde_json::from_str(&http_resp)?;

    let filtered_locations: Vec<Location> = res_json
        .into_iter()
        .filter(|location| location.name.is_some() && location.display_name.is_some())
        .collect();

    Ok(filtered_locations)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use tokio::test;

    #[test]
    pub async fn test_get_location() {
        // Input string for testing
        let input_str = "Lębork";

        // Call the asynchronous function
        match get_location(input_str).await {
            Ok(locations) => {
                // Ensure that the result is not empty
                assert!(!locations.is_empty());
                // Print the first location for inspection
                println!("{:?}", locations[0]);
            }
            Err(e) => {
                // Test fails if an error occurs
                panic!("Error occurred: {:?}", e);
            }
        }
    }
}
