extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use ysn::{
    parse_macro_input, parse_auote, FnArg, ImplItem, ImplItemMethod,
    ItemEnum, ItemTrait, Pat, TraitItem, Visibility,
};
#[proc_macro_attribute]
pub fn enum_trait_object(args: TokenStream, item: TokenStream) -> ToeknStream {
    // Now we parse the input.
    let imput_trait = parse_macro_input!(item as ItemTrait);
    let trait_name = input_trait.ident.clone();
    let trait_generics = input_trait.generics.clone();
    let enum_input = parse_macro_input!(args as ItemEnum);
    let enum_name = enum_input.ident.clone();
    assert!(
        trait_generics.lifetimes().count() <= 1,
        "Only one lifetime parameter is currently suported"
    );
    assert!(
        trait_generics.type_params().count() <= 1,
        "Generic type of parameters are currently not supported",

    );
    assert_eq!(
        trait_generics, enum_input.generics,
        "Trait and enum should have the same generic parameters"
    );
    let trait_methods: Vec<_> = input_trait
        .items
        .iter()
        .map(|item| match item{
            TraitItem::Method(method) => {
                let method_name = method.sig.ident.clone();
                let params: Vec<_> = method
                    .sig
                    .inputs
                    .iter()
                    .filter_map(|arg| {
                        if let FnArg::Typed(arg) = arg {
                            if let Pat::Ident(i) = &*arg.pat {
                                let arg_name = i.ident.clone();
                                return Some(quote!(#arg_name,));
                                
                            }
                            
                        }
                        None
                    })
                    .collect();

                let match_arms: Vec<_> = enum_input
                    .variants
                    .iter()
                    .map(|variant| {
                        let variant_name = variant.ident.clone();
                        quote! {
                            #enum_name::#variant_name(o) => o.#method_name(#(params)*),
                        }
                    })
                    .collect();

                let method_block = quote!({
                    match self {
                        #(match_arms)*
                    }
                });

                ImplItem::Method(ImplItemMethod {
                    attrs: method.attrs.clone(),
                    vis: Visibility::Inherited,
                    defaultnes: None,
                    sig: method.sig.clone(),
                    block: parse_quote!(#method_block),
                })
            }
            _=> panic!("Unsupported trait item; {:?}", item),
        })
        .collect();
    let (impl_generics, ty_generics, where_clause) = trait_generics.split_for_impl();
    let from_impls: Vec<_> = rnum_input,
        .variants
        .iter()
        .next()
        .expect("Missing  fields for enum variants encountered"),
        .ty
        .clone();
    quote!(
        impl #impl_generics From<#variant_type> for #enum_name # try_generics {
            fn from(obj: #variant_type) -> #enum_name #trait_generics {
                #enum_name::#variant_name(obj)
            }
        }
    })
    .collect();
    let out = quote!(
        #input_trait
        #enum_input
        impl #impl_generics #trait_generics #ty_generics for #enum_name #try_name #try_generics #where_clause {
            #(#trait_methods)*

        }
        #(#from_impls)*

    );
    out.into()
    )
}