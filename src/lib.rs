#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, format_ident};
use syn::{Generics, ItemFn, ItemStatic, Signature, Token, parse_macro_input, spanned::Spanned};

#[proc_macro_attribute]
#[doc = include_str!("../README.md")]
pub fn null_fn(_: TokenStream, input: TokenStream) -> TokenStream {
	let mut static_item = parse_macro_input!(input as ItemStatic);

	let null_fn = ItemFn {
		attrs: vec![],
		vis: syn::parse_str("pub(super)").unwrap(),
		block: syn::parse_str("{ panic!(\"This function has not been initialized yet\") }").unwrap(),
		sig: Signature {
			constness: None,
			asyncness: None,
			unsafety: Some(Token![unsafe](static_item.span())),
			abi: None,
			fn_token: Default::default(),
			ident: Ident::new("null_fn", static_item.span()),
			generics: Generics::default(),
			paren_token: Default::default(),
			inputs: Default::default(),
			variadic: Default::default(),
			output: syn::ReturnType::Default
		},
	};

	static_item.expr = syn::parse_str(&format!("unsafe{{std::mem::transmute(__null_fn__{}::null_fn as *const ())}}", static_item.ident)).unwrap();

	let mod_name = format_ident!("__null_fn__{}", static_item.ident);
	quote!(
		#[allow(non_snake_case)]
		mod #mod_name {
			#[allow(unused_variables)]
			#null_fn
		}
		#static_item
	).into()
}