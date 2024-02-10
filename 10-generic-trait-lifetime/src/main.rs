mod extract_func;
mod generic;
mod traits;
mod lifetime;

fn main() {
    extract_func::test_max();

    generic::test_struct();

    traits::test_summary_trait();

    lifetime::generic_lifetimes_in_func();
    lifetime::lifetime_in_struct();
}
