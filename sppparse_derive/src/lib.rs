extern crate proc_macro;

use quote::quote;
use synstructure::BindStyle;

fn sparsable_derive(mut s: synstructure::Structure) -> proc_macro2::TokenStream {
    let body = s.bind_with(|_bi| BindStyle::RefMut).each(|bi| {
        quote! {
            #bi.sparse_init(state, metadata, ndepth)?;
        }
    });
    let crate_name = proc_macro_crate::crate_name("sppparse")
        .map(|v| syn::Ident::new(v.as_str(), proc_macro2::Span::call_site()));
    s.add_bounds(synstructure::AddBounds::Both);
    match crate_name {
		Ok(name) => {
			s.gen_impl(quote! {
				extern crate #name as sppparse;
				gen impl sppparse::SparsableTrait for @Self {
					fn sparse_init(&mut self, state: &mut sppparse::SparseState, metadata: &sppparse::SparseMetadata, depth: u32) -> Result<(), sppparse::SparseError>
					{
						let ndepth = depth+1;
						match *self { #body };
						Ok(())
					}
				}
			})
		}
		_ => {
			s.gen_impl(quote! {
				extern crate sppparse;
				gen impl sppparse::SparsableTrait for @Self {
					fn sparse_init(&mut self, state: &mut sppparse::SparseState, metadata: &sppparse::SparseMetadata, depth: u32) -> Result<(), sppparse::SparseError>
					{
						let ndepth = depth+1;
						match *self { #body };
						Ok(())
					}
				}
			})
		}
	}
}

fn sparsable_derive_inner(mut s: synstructure::Structure) -> proc_macro2::TokenStream {
    let body = s.bind_with(|_bi| BindStyle::RefMut).each(|bi| {
        quote! {
            #bi.sparse_init(state, metadata, ndepth)?;
        }
    });

    s.add_bounds(synstructure::AddBounds::Fields);
    s.gen_impl(quote! {
        use crate::*;
        gen impl SparsableTrait for @Self {
            fn sparse_init(&mut self, state: &mut SparseState, metadata: &SparseMetadata, depth: u32) -> Result<(), SparseError>
            {
				let ndepth = depth+1;

                match *self { #body };
                Ok(())
            }
        }
    })
}

synstructure::decl_derive!([Sparsable] => sparsable_derive);
synstructure::decl_derive!([SparsableInner] => sparsable_derive_inner);
