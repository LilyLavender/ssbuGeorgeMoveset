#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_imports,
	unused_macros,
	unused_variables,
	unused_assignments,
	unused_unsafe,
	non_upper_case_globals,
	non_snake_case,
    clippy::borrow_interior_mutable_const
)]

mod normals;
mod smashes;
mod aerials;
mod throws;
mod specials;

#[skyline::main(name = "george_moveset")]
pub fn main() {
	normals::install();
	smashes::install();
	aerials::install();
	throws::install();
	specials::install();
}