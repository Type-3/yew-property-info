#![feature(proc_macro_diagnostic)]

use quote::quote;
use syn::spanned::Spanned;
use proc_macro::TokenStream;

#[proc_macro_derive(PropertyInfo, attributes(prop_description))]
pub fn derive(input: TokenStream) -> TokenStream {
    let macro_input = syn::parse_macro_input!(input as syn::DeriveInput);
    let ident = &macro_input.ident;
    let crate_path = quote! {::yew_property_info};
    let iterator = if let syn::Data::Struct(strct) = &macro_input.data {
        if let syn::Fields::Named(fields) = &strct.fields {
            fields.named.iter().map(|item| {
                let name = item.ident.as_ref().unwrap();
                let ty = get_field_type(&item.ty);
                let (required, default, description) = get_field_info(&item);
                let desc = match description {
                    None => quote! {None},
                    Some(item) => quote!{Some(#item)}
                };
                let def = match default {
                    None => quote! {None},
                    Some(item) => quote!{Some(#item)}
                };
                quote! {
                    #crate_path::PropertyRef {
                        ty: #ty,
                        name: stringify!(#name),
                        required: #required,
                        default: #def,
                        description: #desc
                    }
                }
            })
        } else {
            unimplemented!("Currently Only Works on structs with named fields");
        }
    } else {
        unimplemented!("Currently Only works on structs");
    };
    let output = quote! {
        impl #crate_path::HasPropertyInfo for #ident {

            fn property_info() -> #crate_path::PropertiesInfo {
                #crate_path::PropertiesInfo {
                    ty: stringify!(#ident),
                    module: std::module_path!(),
                    fields: &[#(#iterator,)*]
                }
            }
        }

        impl #ident {
            pub fn property_info() -> #crate_path::PropertiesInfo {
                <#ident as #crate_path::HasPropertyInfo>::property_info()
            }
        }
    };

    output.into()
}

fn get_field_type(ty: &syn::Type) -> String {
    match ty {
        syn::Type::Path(path) => quote! {#path}.to_string(),
        syn::Type::Array(arr) => quote! {#arr}.to_string(),
        syn::Type::BareFn(bare_fn) => quote! {#bare_fn}.to_string(),
        syn::Type::Group(group) => quote! {#group}.to_string(),
        syn::Type::ImplTrait(item) => quote! {#item}.to_string(),
        syn::Type::Infer(infer) => quote! {#infer}.to_string(),
        syn::Type::Macro(mac) => quote! {#mac}.to_string(),
        syn::Type::Never(never) => quote! { #never }.to_string(),
        syn::Type::Paren(paran) => quote! { #paran }.to_string(),
        syn::Type::Ptr(ptr) => quote! {#ptr}.to_string(),
        syn::Type::Reference(reference) => quote! {#reference}.to_string(),
        syn::Type::Slice(slice) => quote! {#slice}.to_string(),
        syn::Type::TraitObject(obj) => quote! {#obj}.to_string(),
        syn::Type::Tuple(tup) => quote! {#tup}.to_string(),
        syn::Type::Verbatim(verb) => quote! {#verb}.to_string(),
        _ => format!("Unknown")
    }
}

fn get_field_info(item: &syn::Field) -> (bool, Option<String>, Option<String>) {
    let mut required = true;
    let mut prop_default = None;
    let mut description = None;
    for attr in item.attrs.iter() {
        if let Some(ident) = attr.path.get_ident() {
            match ident.to_string().as_ref() {
                "prop_or_default" => {
                    required = false;
                    prop_default = Some("Default::default".to_string());
                },
                "prop_description" => {
                    if let Ok(meta) = attr.parse_meta() {
                        description = get_prop_description(meta);
                    }
                },
                "prop_or" => {
                    if let Ok(meta) = attr.parse_meta() {
                        required = false;
                        prop_default = get_prop(meta);
                    } else {
                        let value = attr.tokens.to_string();
                        required = false;
                        prop_default = Some(value[1..value.len() - 1].into());
                    }
                },
                _ => {}
            }
        } else {
            attr.path.span().unwrap().error("Expected An Ident");
        }
    }
    (required, prop_default, description)
}

fn get_prop_description(meta: syn::Meta) -> Option<String> {
    match meta {
        syn::Meta::List(list) => {
            if list.nested.len() != 1 {
                list.span().unwrap().error("Expected exactly one argument").emit();
                None
            } else {
                let next = list.nested.iter().next();
                if let Some(&syn::NestedMeta::Lit(syn::Lit::Str(string))) = next.as_ref() {
                    Some(string.value())
                } else {
                    list.nested.span().unwrap().error("Expected A String literal").emit();
                    None
                }
            }
        },
        item => {item.span().unwrap().error("Expected a Meta List attribute").emit();None}
    }
}

fn get_prop(meta: syn::Meta) -> Option<String> {
    match meta {
        syn::Meta::List(list) => {
            if list.nested.len() != 1 {
                list.span().unwrap().error("Expected exactly one argument").emit();
                None
            } else {
                let next = list.nested.iter().next();
                if let Some(item) = next.as_ref() {
                    Some(quote!{#item}.to_string())
                } else {
                    list.nested.span().unwrap().error("Expected A String literal").emit();
                    None
                }
            }
        },
        item => {item.span().unwrap().error("Expected a Meta List attribute").emit();None}
    }
}
