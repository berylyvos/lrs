mod closures;
mod iterators;

fn main() {
    closures::test_generate_workout();
    closures::sort_by_key_with_fnmut();
    closures::sort_by_key_with_fnmut_count();
}
