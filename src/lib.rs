mod functions;

use phper::{modules::Module, php_get_module};
use phper::functions::Argument;
use functions::geo_hash::geo_hash_encode;
use functions::geo_hash::geo_hash_decode;
use functions::geo_hash::geo_hash_distance;

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
        //.argument(Argument::by_val("len")); <- This is not needed, as it has a default value

    module
        .add_function("geo_hash_decode", geo_hash_decode)
        .argument(Argument::by_val("hash"));

    module
        .add_function("geo_hash_distance", geo_hash_distance)
        .argument(Argument::by_val("hash1"))
        .argument(Argument::by_val("hash2"));

    module
}