
/// 作为过程宏（proc-macro）的crate，能导出（pub属性）的只能为带有#[proc_macro],#[proc_macro_derive]或#[proc_macro_attribute]的函数。
//extern crate proc_macro; // 在rust 1.31.0时，需要使用extern crate语法
use proc_macro; // proc_macro是编译器用来读取和操作我们rust代码的API
use quote::quote; // quote将syn解析的数据结构转换回rust代码
use syn; // syn将字符串中的rust代码解析成为一个可以操作的数据结构

/// 通过派生宏（derive）实现Hello trait；
/// 当用户在一个enum或struct指定#[derive(HelloMacro)]时，会调用hello_macro_derive生成代码；
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // 构建rust代码所代表的语法树以便可以进行操作
    let ast = syn::parse(input).unwrap();
    // 构建Hello trait实现
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    // ident为enum或struct的名称
    let name = &ast.ident;
    let gen = quote! {
        // 为enum或struct实现Hello trait
        // quote!宏可以将name的值来替换#name（类似C语言宏中用#转换字符串的功能）
        impl Hello for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}.", stringify!(#name));
            }
        }
    };
    gen.into()
}
