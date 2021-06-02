
// 好多库中封装了c语言类型^-^
use libc::c_char;
use std::ffi::c_void;
use std::os::raw::c_int;

#[repr(C)]
struct Ext {
    id: c_int,
    name: *const c_char,
}

/// 声明c库中的函数和变量
//#[link(name="ext")] // 这里使用build.rs进行静态链接，故可以不用再显式指明link
extern {
    fn ext_neg_square(input: c_int) -> c_int;
    fn ext_call(cb: extern fn(c_int)->c_int, input: c_int) -> c_int;
    fn ext_print(e: *mut Ext) -> c_void;
}

/// 传给c调用的函数
#[no_mangle]
extern "C" fn callback(input: c_int) -> c_int {
    println!("callback from rust");
    return - input * input;
}

pub fn run() {
    unsafe {
        println!("ext_neg_square(3): {}", ext_neg_square(3));
        //println!("ext_call(ext_neg_square, 3): {}", ext_call(ext_neg_square, 3)); // error, Why?
        println!("ext_call(callback, 3): {}", ext_call(callback, 3));
        let mut e = Ext {
            id: 5,
            name: "EXT".as_ptr() as *const c_char, // name字段为常量指针
        };
        ext_print(&mut e as *mut Ext); // 参数为可变指针
    }
}
