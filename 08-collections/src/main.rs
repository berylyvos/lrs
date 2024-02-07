mod vec;
mod string;
mod hashmap;

fn main() {
    vec::new_vec();
    vec::update_vec();
    vec::read_vec();
    vec::iterate_vec();
    vec::enum_vec();

    string::new_string();
    string::string_from_utf8();
    string::update_string();
    string::concat_string();
    string::concat_strings();
    string::no_indexing();
    string::slicing_string();

    hashmap::new_hashmap();
    hashmap::new_hashmap_from_vec();
    hashmap::ownerships();
    hashmap::accessing();
    hashmap::inserting();
    hashmap::update_on_old_val();
}
