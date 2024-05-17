mod functions;

use phper::{modules::Module, php_get_module};
use phper::functions::Argument;
use functions::geo_hash::geo_hash_encode;
use functions::geo_hash::geo_hash_decode;

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    module
        .add_function("geo_hash_encode", geo_hash_encode)
        .argument(Argument::by_val("lat"))
        .argument(Argument::by_val("lon"));

    module
        .add_function("geo_hash_decode", geo_hash_decode)
        .argument(Argument::by_val("hash"));

    module
        .add_function("geo_hash_distance", functions::geo_hash::geo_hash_distance)
        .argument(Argument::by_val("hash1"))
        .argument(Argument::by_val("hash2"));

    module
}