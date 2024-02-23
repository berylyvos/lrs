mod _unsafe;
mod adv_trait;
mod adv_type;
mod adv_fn_closure;
mod macros;

fn main() {
    _unsafe::test_split_at_mut();
    _unsafe::test_call_external_code();
    _unsafe::test_static_var();

    adv_trait::test_human_fly();
    adv_trait::test_outline_print_point();
    adv_trait::test_display_for_vec();

    adv_fn_closure::test_func_ptr();
}
