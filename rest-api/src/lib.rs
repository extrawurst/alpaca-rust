use proc_macro2::{Ident, Span};
use quote::quote;
use syn::export::TokenStream;
use syn::Meta;

#[proc_macro_attribute]
pub fn api(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemTrait);
    let name = &input.ident;

    let struct_name = Ident::new(&format!("{}RestClient", name), Span::call_site());

    let struct_def = quote! {
        struct #struct_name {
            headers: http::HeaderMap,
            base_url: String,
        }
    };

    let impl_def = quote! {
        impl #struct_name {
            fn add_headers(&self, builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
                let mut new_builder = builder;
                for (k, v) in self.headers.iter() {
                    new_builder = new_builder.header(k, v);
                }
                new_builder
            }
        }
    };

    // lets create the gutted version of the impl trait first and add the methods later
    let mut impl_trait_def: syn::ItemImpl = syn::parse2(quote! {
        impl #name for #struct_name {
        }
    })
    .unwrap();

    // add the created methods now
    impl_trait_def.items = input
        .clone()
        .items
        .into_iter()
        .map(|a| syn::ImplItem::from(get_impl_item(a)))
        .collect();

    // outputting it all
    let result = quote! {
        #input
        #struct_def
        #impl_def
        #impl_trait_def
    };
    result.into()
}

fn get_impl_item(trait_item: syn::TraitItem) -> syn::ImplItemMethod {
    if let syn::TraitItem::Method(trait_method) = trait_item {
        get_impl_method(trait_method)
    } else {
        panic!("foo");
    }
}

fn get_impl_method(trait_item: syn::TraitItemMethod) -> syn::ImplItemMethod {
    let name = trait_item.clone().sig.ident;
    // find the endpoint attribute defining the url postfix to use
    let endpoint_name = match get_endpoint_attr(trait_item) {
        Some(s) => s,
        None => name.to_string().clone(),
    };

    // build the method
    syn::parse2(quote! {
        fn #name(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
            let url = self.base_url.clone() + #endpoint_name;
            let mut builder = reqwest::Client::new().get(url.as_str());
            builder = self.add_headers(builder);
            let json: serde_json::Value = builder.send()?.json()?;

            Ok(json)
        }
    })
    .unwrap()
}

fn get_endpoint_attr(trait_item: syn::TraitItemMethod) -> Option<String> {
    trait_item
        .attrs
        .into_iter()
        .filter_map(|attr| attr.parse_meta().ok())
        .filter_map(|meta| match meta {
            Meta::List(meta_list) => {
                if meta_list.path.is_ident("endpoint") {
                    match meta_list.nested.first().unwrap() {
                        syn::NestedMeta::Lit(syn::Lit::Str(str_lit)) => Some(str_lit.value()),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            _ => None,
        })
        .next()
}

#[proc_macro_attribute]
pub fn endpoint(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn keep(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_endpoint_none() {
        let a: syn::TraitItemMethod = syn::parse2(quote! {
            fn foo() {}
        })
        .unwrap();

        assert_eq!(get_endpoint_attr(a), None);
    }
    #[test]
    fn test_endpoint_some() {
        let a: syn::TraitItemMethod = syn::parse2(quote! {
            #[endpoint("foo")]
            fn foo() {}
        })
        .unwrap();

        assert_eq!(get_endpoint_attr(a), Some("foo".to_owned()));
    }
    #[test]
    fn test_endpoint_none2() {
        let a: syn::TraitItemMethod = syn::parse2(quote! {
            #[keep]
            fn foo() {}
        })
        .unwrap();

        assert_eq!(get_endpoint_attr(a), None);
    }
}
