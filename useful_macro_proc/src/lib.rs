extern crate proc_macro;
extern crate syn;
extern crate quote;
use proc_macro::TokenStream;
use syn::*;
use syn::__private::Span;
use parse::*;
use quote::*;
use punctuated::*;
use rand::seq::SliceRandom;

struct AnonymousType(Punctuated<Variant, Token![,]>);

impl Parse for AnonymousType {
  fn parse(input: ParseStream) -> Result<Self> {
    let items = Punctuated::<Variant, Token![,]>::parse_terminated(&input)?;
    Ok(Self(items))
  }
}

fn generate_name() -> String {
  let token_length = 4;
  let alpha_upper = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
  let alpha_lower = "abcdefghijklmnopqrstuvwxyz";
  let alpha = format!("{}{}", alpha_upper, alpha_lower);
  let digit = "0123456789";
  let alpha_digit = format!("{}{}", alpha, digit);
  let mut rng = rand::thread_rng();
  let bytes: Vec<u8> = alpha_digit
    .as_bytes()
    .choose_multiple(&mut rng, token_length)
    .cloned()
    .collect();
  String::from(format!("Enum_{}", std::str::from_utf8(&bytes).unwrap()))
}

#[proc_macro]
pub fn anon_enum(input: TokenStream) -> TokenStream {
  let AnonymousType(variants) = parse_macro_input!(input as AnonymousType);
  let name = Ident::new(&generate_name(), Span::call_site());

  let expanded = quote! {
    #name;
    
    enum #name {
      #variants
    }
  };

  TokenStream::from(expanded)
}


