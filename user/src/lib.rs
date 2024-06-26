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
pub extern "C" fn _start()->! {
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
    extern "C" {
        fn start_bss();
        fn end_bss();
    }
    unsafe {
        core::slice::from_raw_parts_mut(
            start_bss as usize as *mut u8,
            end_bss as usize - start_bss as usize,
        )
        .fill(0);
    }
}

#[linkage = "weak"]
#[no_mangle]
fn main() ->i32 {
    panic!("Cannot find main!");
}