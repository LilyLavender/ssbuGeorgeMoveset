#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(unused_macros)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

mod normals;
//jab
//dtilt
//utilt
//ftilt
//dash

mod smashes;
//fsmash
//dsmash
//usmash

mod aerials;
//nair
//uair
//dair
//fair
//bair

mod throws;
//grab
//bthrow
//fthrow
//uthrow
//dthrow

mod monado;

#[skyline::main(name = "smashline_test")]
pub fn main() {
	normals::install();
	smashes::install();
	aerials::install();
	throws::install();
	monado::install();
}