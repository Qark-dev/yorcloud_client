use std::path::PathBuf;

use proc_macro::{self, Ident, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

fn lowercase(ident: &syn::Ident) -> syn::Ident {
    syn::Ident::new(&ident.to_string().to_lowercase(), ident.span())
}

#[proc_macro_derive(StaticAsset)]
pub fn static_asset(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let enum_ = match data {
        syn::Data::Enum(e) => e,

        _ => {
            panic!("i want an enum tho")
        }
    };

    let variants = || enum_.variants.iter().map(|variant| &variant.ident);
    let each_file = variants().map(|var_ident| {
        format!(
            "{}/assets/{}--{}.png",
            std::env::current_dir()
                .unwrap_or(PathBuf::from("./"))
                .as_path()
                .to_str()
                .unwrap()
                .to_string(),
            ident.to_string().to_lowercase(),
            var_ident.to_string().to_lowercase()
        )
    });
    let each_variant = variants();
    let each_variant2 = variants();

    quote!(
        impl #ident {
            pub fn to_bytes(&self) -> &'static [::core::primitive::u8] {
                match *self {
                    #(
                        Self::#each_variant { .. } => ::core::include_bytes!(#each_file),
                    )*
                 }
            }

            pub fn to_map() -> impl Iterator<Item = (Self, &'static [::core::primitive::u8])> {
                let v = vec![
                    #(

                            ( Self::#each_variant2, Self::#each_variant2.to_bytes() ),

                    )*
                ];

                v.into_iter()
            }

            pub fn to_retained_img_map() -> std::collections::HashMap<Self, egui_extras::RetainedImage> {
                Self::to_map().map(|(variant, bytes)| {
                    let img = egui_extras::RetainedImage::from_image_bytes(format!("{:?}", variant), bytes).unwrap();

                    (variant, img)
                }).collect()
            }
        }
    )
    .into()
}
