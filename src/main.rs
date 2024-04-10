include!("in-src.rs");
include!("../in-crate.rs");
include!("../../in-workspace.rs");

fn main() {
    in_src();
    in_crate();
    in_workspace();
}
