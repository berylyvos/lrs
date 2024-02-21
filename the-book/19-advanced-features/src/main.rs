mod _unsafe;

fn main() {
    _unsafe::test_split_at_mut();
    _unsafe::test_call_external_code();
    _unsafe::test_static_var();
}
