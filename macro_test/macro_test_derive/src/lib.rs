use proc_macro::TokenStream;
use quote::quote;
use cfg_if::cfg_if;




#[proc_macro_attribute]
pub fn macro_test_derive(attr: TokenStream, input: TokenStream) -> TokenStream {
    cfg_if!{
        if #[cfg(feature = "test")]{
            println!("the attr is11 {:?}",attr.to_string());
        }
    }

    (quote! {
        fn hello1(){
            println!("it's hello1")
        }
    }).into()
}


