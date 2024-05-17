use geohash::{encode, decode, Coord};
use phper::{values::ZVal, arrays::ZArray, Result, Error};
use haversine::{distance, Location, Units};

pub fn geo_hash_encode(arguments: &mut [ZVal]) -> Result<ZVal> {
    let lat = arguments.get(0).and_then(|v| v.as_double()).ok_or_else(|| {
        Error::boxed("Latitude must be a valid floating point number.")
    })?;

    let lon = arguments.get(1).and_then(|v| v.as_double()).ok_or_else(|| {
        Error::boxed("Longitude must be a valid floating point number.")
    })?;

    // Check if len is provided, otherwise use default value of 12
    let default_len: i64 = 12;
    let len = arguments.get(2).and_then(|v| v.as_long()).unwrap_or(default_len);

    let c = Coord { x: lat, y: lon };
    let hash = encode(c, len as usize).map_err(|e| {
        Error::boxed(format!("Encoding error: {}", e))
    })?;

    Ok(ZVal::from(hash))
}

pub fn geo_hash_decode(arguments: &mut [ZVal]) -> Result<ZVal> {
    let hash_val = arguments[0].expect_z_str().map_err(|_| {
        Error::boxed("Geohash must be a valid string.")
    })?;
    let hash_str = hash_val.to_str()?;
    match decode(hash_str) {
        Ok((coord, _error_lat, _error_lon)) => {
            let mut result = ZArray::new();
            result.insert("lat", coord.y);
            result.insert("lon", coord.x);
            Ok(ZVal::from(result)) // Return the ZArray in ZVal form
        },
        Err(e) => {
            let err_msg = format!("Error decoding geohash: {}", e);
            Err(Error::boxed(err_msg))
        }
    }
}

pub fn geo_hash_distance(arguments: &mut [ZVal]) -> Result<ZVal> {
    if arguments.len() != 2 {
        return Err(Error::boxed("geo_hash_distance expects exactly two arguments."));
    }
    let geohash1 = arguments[0].expect_z_str()?.to_str()?;
    let geohash2 = arguments[1].expect_z_str()?.to_str()?;
    let coord1 = decode(geohash1).map_err(|e| Error::boxed(e))?;
    let coord2 = decode(geohash2).map_err(|e| Error::boxed(e))?;
    let loc1 = Location { latitude: coord1.0.x, longitude: coord1.0.y };
    let loc2 = Location { latitude: coord2.0.x, longitude: coord2.0.y };
    let distance = distance(loc1, loc2, Units::Kilometers);
    Ok(ZVal::from(distance)) // Return the computed distance
}