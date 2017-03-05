extern crate xargo_lld_wrapper;

fn main() {
    xargo_lld_wrapper::lib_main(std::env::args().collect::<Vec<_>>()
                                                .iter()
                                                .map(|el| el.as_str()).collect::<Vec<_>>());
}
