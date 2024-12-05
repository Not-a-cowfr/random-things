extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn use_in_menu(
	_attr: TokenStream,
	item: TokenStream,
) -> TokenStream {
	let input = parse_macro_input!(item as ItemFn);
	let name = &input.sig.ident;
	let new_name = syn::Ident::new(&format!("{}_menu", name), name.span());
	let gen = quote! {
		#input

		pub fn #new_name() {
			println!("Running {}...", stringify!(#name));
			#name();
		}
	};
	gen.into()
}
