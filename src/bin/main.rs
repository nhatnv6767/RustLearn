// we putted in [lib] name = "demo"
use demo::print_from_lib;
use demo::group;
fn main() {
    print_from_lib();
    group::g1::g1_hello();
}
