use rstest::*;
use singlemalt::server::CuckooFilterService;

#[fixture]
pub fn server() -> CuckooFilterService {
    CuckooFilterService::default()
}
