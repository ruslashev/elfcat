use proc_macro::TokenStream;

#[proc_macro_derive(ElfHeaderMethods)]
pub fn derive_methods(item: TokenStream) -> TokenStream {
    println!("item: {:?}", item);

    "fn test() {}".parse().unwrap()
}
