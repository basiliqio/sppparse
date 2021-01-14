extern crate proc_macro;

use quote::quote;
use synstructure::BindStyle;

fn sparsable_derive(mut s: synstructure::Structure) -> proc_macro2::TokenStream {
    let body = s.bind_with(|_bi| BindStyle::RefMut).each(|bi| {
        quote! {
            #bi.sparse_init(state)?;
        }
    });

    s.gen_impl(quote! {
		extern crate sparse;
		use sparse::SparsableTrait;
        gen impl SparsableTrait for @Self {
			fn sparse_init(&mut self, state: &mut sparse::SparseState) -> Result<(), sparse::SparseError>
			{
				match *self { #body };
				Ok(())
			}
        }
    })
}
synstructure::decl_derive!([Sparsable] => sparsable_derive);
