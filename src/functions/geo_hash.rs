use geohash::{encode, decode, Coord};
use phper::{echo, values::ZVal, arrays::ZArray, Result, Error};
use haversine::{distance, Location, Units};

pub fn geo_hash_encode(arguments: &mut [ZVal]) -> Result<()> {
    let lat = arguments[0].expect_double().map_err(|_| {
        Error::boxed("Latitude must be a valid floating point number.")
    })?;

    let lon = arguments[1].expect_double().map_err(|_| {
        Error::boxed("Longitude must be a valid floating point number.")
    })?;

    let c = Coord { x: lat, y: lon };
    let hash = encode(c, 12);
    echo!("{:?}", hash);
    Ok(())
}

pub fn geo_hash_decode(arguments: &mut [ZVal]) -> Result<()> {
    let hash_val = arguments[0].expect_z_str().map_err(|_| {
        Error::boxed("Geohash must be a valid string.")
    })?;
    let hash_str = hash_val.to_str()?;

    match decode(hash_str) {
        Ok((coord, _error_lat, _error_lon)) => {
            let mut result = ZArray::new();
            result.insert("lat", coord.y);
            result.insert("lon", coord.x);

            echo!("{:?}", result);
        },
        Err(e) => {
            let error_message = format!("Error decoding geohash: {}", e);
            return Err(Error::boxed(error_message));
        }
    }

    Ok(())
}

pub fn geo_hash_distance(arguments: &mut [ZVal]) -> Result<()> {
    if arguments.len() != 2 {
        return Err(Error::boxed("geo_hash_distance expects exactly two arguments."));
    }

    let geohash1 = arguments[0].expect_z_str()?.to_str()?;
    let geohash2 = arguments[1].expect_z_str()?.to_str()?;

    let coord1 = decode(geohash1).map_err(|e| Error::boxed(Box::new(e) as Box<dyn std::error::Error>))?;
    let coord2 = decode(geohash2).map_err(|e| Error::boxed(Box::new(e) as Box<dyn std::error::Error>))?;

    let loc1 = Location { latitude: coord1.0.x, longitude: coord1.0.y };
    let loc2 = Location { latitude: coord2.0.x, longitude: coord2.0.y };

    let distance = distance(loc1, loc2, Units::Kilometers);

    echo!("{:?}", distance);
    Ok(())
}