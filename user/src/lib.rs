#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]


#[macro_use]
pub mod console;
mod syscall;
mod lang_items;

use syscall::*;
#[no_mangle]
#[link_section=".text.entry"]
pub extern "C" fn _start()->() {
    clear_bss();
    exit(main());
}

pub fn write(fd:usize,buf:&[u8]) -> isize {
    sys_write(fd,buf,buf.len())
}

pub fn exit(exit_code: i32) -> !{
    sys_exit(exit_code)
}

fn clear_bss(){

}

#[linkage = "weak"]
#[no_mangle]
fn main() ->i32 {
    panic!("Cannot find main!");
}