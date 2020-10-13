
use proc_macro;
use quote::quote;
use syn;

#[proc_macro]
pub fn call(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let name: syn::Ident = syn::parse(input).unwrap();
    let gen = quote! {
        #name(); // 调用函数
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn show_attr(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    println!("attr:\n {}", attr.to_string()); // 在编译时打印
    println!("item:\n {}", item.to_string());
    let ast: syn::ItemFn = syn::parse(item).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        fn #name() {
            println!("This is {} function", stringify!(#name));
        }
    };
    gen.into()
}
