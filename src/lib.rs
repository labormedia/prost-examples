// Include the `items` module, which is generated from items.proto.
pub mod test {
    include!(concat!(env!("OUT_DIR"), "/prost.test.rs"));
}

pub fn create_seed(trend: String) -> test::Product {
    let mut myseed = test::Product::default();
    myseed.trend = trend;
    myseed.set_myseed( test::product::Seed::Two );
    myseed
}