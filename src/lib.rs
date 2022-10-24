use proc_macro::TokenStream;

#[proc_macro]
pub fn generator(input: TokenStream) -> TokenStream {
    println!("{:#?}", input);
    TokenStream::default()
}
