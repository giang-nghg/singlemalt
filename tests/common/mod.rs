use rstest::*;
use singlemalt::server::DefaultCuckooFilter;

#[fixture]
pub fn server() -> DefaultCuckooFilter { DefaultCuckooFilter::default() }
