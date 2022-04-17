use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

#[proc_macro_derive(MySerialize)]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(input as syn::DeriveInput);
    match do_expand(&st) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn do_expand(st: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_name_literal = st.ident.to_string();
    let struct_name_ident = syn::Ident::new(&struct_name_literal, st.span());

    let fields = get_fields_from_derive_input(st)?;
    let builder_println = generate_println_from_struct_fields(fields)?;

    let ret = quote! {
        impl #struct_name_ident {
            fn serialize(&self) {
                #builder_println
            }
        }
    };
    Ok(ret)
}

type StructFields = syn::punctuated::Punctuated<syn::Field, syn::Token!(,)>;

/// 从derive input数据结构中获取field
fn get_fields_from_derive_input(d: &syn::DeriveInput) -> syn::Result<&StructFields> {
    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = d.data
    {
        return Ok(named);
    }
    Err(syn::Error::new_spanned(
        d,
        "Must define on a Struct, not Enum".to_string(),
    ))
}

/// 根据struct的field生成println
fn generate_println_from_struct_fields(
    fields: &StructFields,
) -> syn::Result<proc_macro2::TokenStream> {
    let idents: Vec<_> = fields.iter().map(|f| &f.ident).collect();

    let mut println_code_pieces = Vec::new();
    for idx in 0..idents.len() {
        let ident = idents[idx].as_ref().unwrap();
        let ident_name = ident.to_string();
        println_code_pieces.push(quote! {
            println!("{:?}, {:?}", #ident_name, self.#ident);
        });
    }

    let token_stream = quote! {
        #(#println_code_pieces)*
    };
    Ok(token_stream)
}
