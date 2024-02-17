mod box_t;
mod dref;
mod drop;
mod rc_t;
mod refcell_t;
mod rc_refcell_t;

fn main() {
    box_t::point_to_recursive_type();
    dref::treat_smart_pointers_like_regular_ref_with_deref();
    dref::deref_coercion();
    drop::test_drop_trait();
    rc_t::test_refrence_counted_smart_pointer();
    refcell_t::it_sends_an_over_75_percent_warning_message();
    rc_refcell_t::test_rc_refcell_list();
}
