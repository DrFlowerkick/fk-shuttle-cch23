//!day_21.rs

use crate::app_error::{AppError, AppResult};
use axum::{extract::Path, routing::get, Router};

use country_boundaries::{CountryBoundaries, LatLon, BOUNDARIES_ODBL_360X180};
use country_emoji::name as country_name;
use dms_coordinates::DMS;
use s2::cell::Cell;
use s2::cellid::CellID;

pub fn get_routes() -> Router {
    eprintln!("loading routes day_21");
    Router::new()
        .route("/21/coords/:binary", get(s2_cell_id_to_dms))
        .route("/21/country/:binary", get(s2_cell_id_to_country))
}

async fn s2_cell_id_to_dms(Path(binary): Path<String>) -> AppResult<String> {
    let cellid = CellID(u64::from_str_radix(binary.as_str(), 2)?);
    let center = Cell::from(cellid).center();
    let longitude = DMS::from_decimal_degrees(center.longitude().deg(), false);
    let latitude = DMS::from_decimal_degrees(center.latitude().deg(), true);
    let result = format!(
        "{}°{}'{:.3}''{} {}°{}'{:.3}''{}",
        latitude.degrees,
        latitude.minutes,
        latitude.seconds,
        latitude.bearing,
        longitude.degrees,
        longitude.minutes,
        longitude.seconds,
        longitude.bearing
    );
    Ok(result)
}

async fn s2_cell_id_to_country(Path(binary): Path<String>) -> AppResult<String> {
    let cellid = CellID(u64::from_str_radix(binary.as_str(), 2)?);
    let center = Cell::from(cellid).center();
    let boundaries = CountryBoundaries::from_reader(BOUNDARIES_ODBL_360X180)?;
    let position = LatLon::new(center.latitude().deg(), center.longitude().deg())?;
    let codes = boundaries.ids(position);
    eprintln!(
        "binary: {}, position: {}, codes: {:?}",
        binary, position, codes
    );
    let code = *codes
        .last()
        .ok_or(AppError::bad_request("coordinates not in country"))?;
    let mut result = country_name(code).ok_or(AppError::bad_request("Unknown Country Code"))?;
    // country-emoji returns for "BN" "Brunei Darussalam", but Challenge test expects only "Brunei"
    if code == "BN" {
        result = "Brunei";
    }
    Ok(result.into())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_dms_formatting() {
        let binary = "0100111110010011000110011001010101011111000010100011110001011011";
        let cellid = CellID(u64::from_str_radix(binary, 2).unwrap());
        let cell = Cell::from(cellid);
        let center = cell.center();
        let longitude = DMS::from_decimal_degrees(center.longitude().deg(), false);
        let latitude = DMS::from_decimal_degrees(center.latitude().deg(), true);

        eprintln!("Default: {} {}", latitude, longitude);
        eprintln!(
            "task: {}°{}'{:.3}''{} {}°{}'{:.3}''{}",
            latitude.degrees,
            latitude.minutes,
            latitude.seconds,
            latitude.bearing,
            longitude.degrees,
            longitude.minutes,
            longitude.seconds,
            longitude.bearing
        );
    }

    #[test]
    fn test_country() {
        let binary = "0010000111110000011111100000111010111100000100111101111011000101";
        let cellid = CellID(u64::from_str_radix(binary, 2).unwrap());
        let cell = Cell::from(cellid);
        let center = cell.center();
        let boundaries = CountryBoundaries::from_reader(BOUNDARIES_ODBL_360X180).unwrap();
        let position = LatLon::new(center.latitude().deg(), center.longitude().deg()).unwrap();
        let code = boundaries.ids(position);
        eprintln!("{:?}", code.last().unwrap());
        eprintln!("{:?}", country_name(code.last().unwrap()));
        let code = "BN";
        eprintln!("code: {}, country: {:?}", code, country_name(code));
    }
}
