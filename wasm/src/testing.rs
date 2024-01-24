use crate::utils::set_panic_hook;

mod formatting;
mod parsing;

#[warn(dead_code)]
fn setup() {
    set_panic_hook();
}
