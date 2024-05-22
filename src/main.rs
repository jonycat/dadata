use serde::{Deserialize, Serialize};
use serde_json::Value; // Import Value for generic JSON handling
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
struct Suggestion {
    value: String,
    unrestricted_value: String,
    data: Datalist,
}
#[derive(Debug, Serialize, Deserialize)]
struct Datalist {
    postal_code: Option<String>,
    country: String,
    country_iso_code: String,
    region_fias_id: Option<String>,
    region_kladr_id: Option<String>,
    region_iso_code: Option<String>,
    region_with_type: Option<String>,
    region_type: Option<String>,
    region_type_full: Option<String>,
    region: Option<String>,
    area_fias_id: Option<String>,
    area_kladr_id: Option<String>,
    area_with_type: Option<String>,
    area_type: Option<String>,
    area_type_full: Option<String>,
    area: Option<String>,
    city_fias_id: Option<String>,
    city_kladr_id: Option<String>,
    city_with_type: Option<String>,
    city_type: Option<String>,
    city_type_full: Option<String>,
    city: Option<String>,
    city_area: Option<String>,
    city_district_fias_id: Option<String>,
    city_district_kladr_id: Option<String>,
    city_district_with_type: Option<String>,
    city_district_type: Option<String>,
    city_district_type_full: Option<String>,
    city_district: Option<String>,
    settlement_fias_id: Option<String>,
    settlement_kladr_id: Option<String>,
    settlement_with_type: Option<String>,
    settlement_type: Option<String>,
    settlement_type_full: Option<String>,
    settlement: Option<String>,
    street_fias_id: Option<String>,
    street_kladr_id: Option<String>,
    street_with_type: Option<String>,
    street_type: Option<String>,
    street_type_full: Option<String>,
    street: Option<String>,
    stead_fias_id: Option<String>,
    stead_cadnum: Option<String>,
    stead_type: Option<String>,
    stead_type_full: Option<String>,
    stead: Option<String>,
    house_fias_id: Option<String>,
    house_kladr_id: Option<String>,
    house_cadnum: Option<String>,
    house_flat_count: Option<String>,
    house_type: Option<String>,
    house_type_full: Option<String>,
    house: Option<String>,
    block_type: Option<String>,
    block_type_full: Option<String>,
    block: Option<String>,
    entrance: Option<String>,
    floor: Option<String>,
    flat_fias_id: Option<String>,
    flat_cadnum: Option<String>,
    flat_type: Option<String>,
    flat_type_full: Option<String>,
    flat: Option<String>,
    flat_area: Option<String>,
    square_meter_price: Option<String>,
    flat_price: Option<String>,
    postal_box: Option<String>,
    fias_id: Option<String>,
    fias_code: Option<String>,
    fias_level: Option<String>,
    fias_actuality_state: Option<String>,
    kladr_id: Option<String>,
    geoname_id: Option<String>,
    capital_marker: Option<String>,
    okato: Option<String>,
    oktmo: Option<String>,
    tax_office: Option<String>,
    tax_office_legal: Option<String>,
    timezone: Option<String>,
    geo_lat: Option<String>,
    geo_lon: Option<String>,
    beltway_hit: Option<String>,
    beltway_distance: Option<String>,
    metro: Option<String>,
    divisions: Option<String>,
    qc_geo: Option<String>,
    qc_complete: Option<String>,
    qc_house: Option<String>,
    history_values: Option<Vec<String>>,
    unparsed_parts: Option<String>,
    source: Option<String>,
    qc: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up your DaData API token
    let token = "14e57df192d307af2ace246446ecd5469fa3c3db";

    // Construct the URL for the suggestion API endpoint
    let url = format!("http://suggestions.dadata.ru/suggestions/api/4_1/rs/suggest/address");

    // Prepare the request body
    let query = json!({
        "query": "москва хабар", // Your query string
        "count": 1 // Number of suggestions you want to receive
    });

    // Create a reqwest client
    let client = reqwest::blocking::Client::new();

    // Make a POST request to the DaData API
    let response = client
        .post(&url)
        .header("Authorization", format!("Token {}", token))
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&query)?)
        .send()?;

    // Parse the response JSON
    let json_response: Value = response.json()?;

    // Extract suggestions array from JSON
    if let Some(suggestions) = json_response.get("suggestions").and_then(|s| s.as_array()) {
        // Parse suggestions into Suggestion struct
        for suggestion in suggestions {
            let suggestion: Suggestion = serde_json::from_value(suggestion.clone())?;
            println!("{:?}", suggestion);
        }
    } else {
        eprintln!("No suggestions found in the response.");
    }

    Ok(())
}