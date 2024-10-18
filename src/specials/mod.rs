pub static mut state: [&str; 8] = ["none"; 8];
pub static mut change: [&str; 8] = ["none"; 8];
pub static mut changing: [bool; 8] = [false; 8];

pub static mut smashCD: [f32; 8] = [0.0; 8];
pub static mut busterCD: [f32; 8] = [0.0; 8];
pub static mut jumpCD: [f32; 8] = [0.0; 8];
pub static mut shieldCD: [f32; 8] = [0.0; 8];
pub static mut speedCD: [f32; 8] = [0.0; 8];

pub static mut totalTimer: [f32; 8] = [0.0; 8];
pub static mut timer: [f32; 8] = [0.0; 8];

pub static mut r: [f32; 8] = [0.0; 8];
pub static mut g: [f32; 8] = [0.0; 8];
pub static mut b: [f32; 8] = [0.0; 8];
pub static mut effect: [i32; 8] = [0; 8];

pub mod specialhi;
pub mod specialhiclose;
pub mod specialhifall;
pub mod specialn;
pub mod specialnaction;
pub mod acmd;
pub mod frame;

pub fn install() {
    specialhi::install();
    specialhiclose::install();
    specialhifall::install();
    specialn::install();
    specialnaction::install();
    acmd::install();
    frame::install();
}