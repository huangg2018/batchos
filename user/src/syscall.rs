use core::arch::asm;
//系统写入调用系统编号
const SYSCALL_WRITE: usize = 64;
//系统退出的系统调用编号
const SYSCALL_EXIT: usize = 93;

///功能：将内存缓冲区中的数据写入文件
///参数：`fd`表示待写入文件的文件描述符；
///       `buf` 表示内存缓冲区的起始地址
///       `len` 表示内存缓冲区的长度
///返回值：返回成功写入的长度
/// syscall ID:64     
pub fn sys_write(fd:usize,buffer:&[u8], len:usize) -> isize {
    syscall(SYSCALL_WRITE,[fd,buffer.as_ptr() as usize,len])
}

/// 功能：退出应用程序并将返回值告知批处理系统。
/// 参数：`exit_code` 表示应用程序的返回值。
/// 返回值：该系统调用不应该返回。
/// syscall ID：93
pub fn sys_exit(xstate: i32) ->!{
    syscall(SYSCALL_EXIT, [xstate as usize,0,0]);
    panic!("sys_exit never returns!");
}
/// 功能：调用ecall指令进行系统调用
/// 参数：`id`表示系统调用ID
///       `args`表示系统调用参数
/// 返回值：系统调用返回值
fn syscall(id: usize,args:[usize;3]) -> isize {
    let mut ret:isize;  //保存系统调用的返回值
    unsafe {
        asm!(  //汇编宏，可获取上下文的变量，并对其进行操作
            "ecall", //echo指令出发系统调用
            inlateout("x10") args[0]=>ret, //a0寄存器同时作为输入和输出寄存器
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id,
        );
    }
    return ret;
}
