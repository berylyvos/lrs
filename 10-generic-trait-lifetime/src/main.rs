mod extract_func;
mod generic;
mod traits;

fn main() {
    extract_func::test_max();

    generic::test_struct();

    traits::test_summary_trait();
}
