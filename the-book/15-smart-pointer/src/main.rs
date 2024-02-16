mod box_t;
mod dref;
mod drop;

fn main() {
    box_t::point_to_recursive_type();
    dref::treat_smart_pointers_like_regular_ref_with_deref();
    dref::deref_coercion();
    drop::test_drop_trait();
}
