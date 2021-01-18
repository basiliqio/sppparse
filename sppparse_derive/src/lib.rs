extern crate proc_macro;

use quote::quote;
use synstructure::BindStyle;

fn sparsable_derive(mut s: synstructure::Structure) -> proc_macro2::TokenStream {
    let body = s.bind_with(|_bi| BindStyle::RefMut).each(|bi| {
        quote! {
            #bi.sparse_init(state, metadata, ndepth)?;
        }
    });

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

fn sparsable_derive_inner(mut s: synstructure::Structure) -> proc_macro2::TokenStream {
    let body = s.bind_with(|_bi| BindStyle::RefMut).each(|bi| {
        quote! {
            #bi.sparse_init(state, metadata, ndepth)?;
        }
    });

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
