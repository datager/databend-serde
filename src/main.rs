extern crate myserde;

fn main() {
    myserde::serde_derive::serde_derive_fn();
    myserde::serde_std::serde_std_fn();
}
