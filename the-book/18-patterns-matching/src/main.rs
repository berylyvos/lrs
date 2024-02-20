mod patterns;
mod refutability;
mod pattern_syntax;

fn main() {
    patterns::if_let_else_if_let_else();
    patterns::while_let();
    patterns::for_loop();
    patterns::func_param_destructure_a_tuple();

    pattern_syntax::match_with_a_shadowed_var();
    pattern_syntax::match_ranges_of_values();
    pattern_syntax::destructuring_structs();
    pattern_syntax::destructuring_nested_enums();
}
