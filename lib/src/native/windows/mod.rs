#[macro_use]
mod macros;
mod slateapp;
mod newgame;
mod tick;
mod controller;

pub use self::slateapp::{hook_slateapp, FSlateApplication};
pub use self::newgame::hook_newgame;
pub use self::tick::hook_tick;
pub use self::controller::{hook_controller, AController};

use std::ptr::null;

use winapi::c_void;
use winapi::winnt::{PAGE_READWRITE, PAGE_EXECUTE_READ};
use kernel32::{VirtualProtect, GetModuleHandleA};

use consts;

// https://www.unknowncheats.me/forum/general-programming-and-reversing/123333-demo-pure-rust-internal-coding.html
// Entry Point
#[no_mangle]
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub extern "stdcall" fn DllMain(module: u32, reason: u32, reserved: *mut c_void) {
    match reason {
        1 => ::initialize(),
        _ => ()
    }
}

lazy_static! {
    static ref BASE: usize = unsafe { GetModuleHandleA(null()) as usize };
}

pub static mut FSLATEAPPLICATION: usize = 0;
pub static mut FSLATEAPPLICATION_TICK: usize = 0;
pub static mut AMYCHARACTER_EXECFORCEDUNCROUCH: usize = 0;
pub static mut FENGINELOOP_TICK_AFTER_UPDATETIME: usize = 0;
pub static mut APP_DELTATIME: usize = 0;
pub static mut FSLATEAPPLICATION_ONKEYDOWN: usize = 0;
pub static mut FSLATEAPPLICATION_ONKEYUP: usize = 0;
pub static mut FSLATEAPPLICATION_ONRAWMOUSEMOVE: usize = 0;
pub static mut ACONTROLLER_GETCONTROLROTATION: usize = 0;

pub fn init() {
    let base = &*BASE;
    log!("Got Base address: {:#x}", base);
    unsafe {
        FSLATEAPPLICATION_TICK = base + consts::FSLATEAPPLICATION_TICK;
        AMYCHARACTER_EXECFORCEDUNCROUCH = base + consts::AMYCHARACTER_EXECFORCEDUNCROUCH;
        FENGINELOOP_TICK_AFTER_UPDATETIME = base + consts::FENGINELOOP_TICK_AFTER_UPDATETIME;
        APP_DELTATIME = base + consts::APP_DELTATIME;
        FSLATEAPPLICATION_ONKEYDOWN = base + consts::FSLATEAPPLICATION_ONKEYDOWN;
        FSLATEAPPLICATION_ONKEYUP = base + consts::FSLATEAPPLICATION_ONKEYUP;
        FSLATEAPPLICATION_ONRAWMOUSEMOVE = base + consts::FSLATEAPPLICATION_ONRAWMOUSEMOVE;
        ACONTROLLER_GETCONTROLROTATION = base + consts::ACONTROLLER_GETCONTROLROTATION;
    }
}

pub fn make_rw(addr: usize) {
    log!("make_rw: {:#x}", addr);
    let page = addr & !0xfff;
    let page = page as *mut c_void;
    let mut out = 0;
    unsafe { VirtualProtect(page, 0x1000, PAGE_READWRITE, &mut out); }
}

pub fn make_rx(addr: usize) {
    log!("make_rx: {:#x}", addr);
    let page = addr & !0xfff;
    let page = page as *mut c_void;
    let mut out = 0;
    unsafe { VirtualProtect(page, 0x1000, PAGE_EXECUTE_READ, &mut out); }
}
