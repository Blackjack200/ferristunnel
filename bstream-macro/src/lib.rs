use proc_macro::TokenStream;

use syn::{Data, DeriveInput, Field, Ident, Meta, parse_macro_input};
use syn::__private::quote::__private::Span;
use syn::__private::quote::quote;
use syn::__private::ToTokens;
use syn::Type::Path;

#[inline]
fn is_supported_typ(str: &str) -> bool {
    matches!(
        str,
        "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" | "f32" | "f64" | "bool"
    )
}

#[proc_macro_derive(
BStream,
attributes(
LittleEndian,
BigEndian,
Varint,
)
)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_identifier = &input.ident;

    match input.data {
        Data::Struct(syn::DataStruct { fields, .. }) => {
            let mut read = quote! {};
            let mut write = quote! {};

            for field in fields {
                match &field.ty {
                    Path(_) => {
                        let typ_str = field.ty.to_token_stream().to_string();
                        if !is_supported_typ(&typ_str) {
                            let field_id = &field.ident;
                            let typ = &field.ty;
                            read.extend(quote! {
                                self.#field_id = #typ::read(out)?;
                            });
                            write.extend(quote! {
                                self.#field_id.write(out)?;
                            });
                            continue;
                        }
                        let mut little_endian = true;
                        let mut varint = false;
                        scan_attribute(&field, &mut little_endian, &mut varint);
                        read.extend(get_func(&field, little_endian, varint, true));
                        write.extend(get_func(&field, little_endian, varint, false));
                    }
                    _ => unimplemented!(
                        "unsupported type {}",
                        field.ty.to_token_stream().to_string()
                    ),
                }
            }

            quote! {
                impl bstream::BinaryStream for #struct_identifier {
                    fn read(&mut self, out: &mut impl ::std::io::Read) -> ::std::io::Result<()> {
                        #read
                        ::std::io::Result::Ok(())
                    }

                    fn write(&self, out: &mut impl ::std::io::Write) -> ::std::io::Result<()> {
                        #write
                        ::std::io::Result::Ok(())
                    }
                }
            }.into()
        }
        _ => unimplemented!(),
    }
}

fn get_func(
    field: &Field,
    little_endian: bool,
    varint: bool,
    read: bool,
) -> proc_macro2::TokenStream {
    let field_id = &field.ident;
    let mut typ = field.ty.to_token_stream().to_string();
    if varint {
        typ.insert(0, 'v');
    }
    let b = gen_fn(&typ, little_endian, read);
    if read {
        return quote! {self.#field_id = #b(out)?;};
    }
    quote! {#b(out, self.#field_id)?;}
}

fn gen_fn(typ: &String, little_endian: bool, read: bool) -> proc_macro2::TokenStream {
    let mut varint = typ == "vi32" || typ == "vu32" || typ == "vi64" || typ == "vu64";
    let mut func = String::from(match read {
        true => "read_",
        false => "write_",
    });
    let mut endian = Ident::new("LittleEndian", Span::call_site());
    if !little_endian {
        endian = Ident::new("BigEndian", Span::call_site())
    }

    func.push_str(typ);
    let func_id = Ident::new(func.as_str(), Span::call_site());

    if typ == "bool" {
        varint = true
    }
    let ignore_endian = typ == "u8" || typ == "i8";
    if varint {
        return match read {
            true => {
                quote! {::bstream::ReaderExt::#func_id}
            }
            false => {
                quote! {::bstream::WriterExt::#func_id}
            }
        };
    }
    let ext = Ident::new(
        match read {
            true => "ReadBytesExt",
            false => "WriteBytesExt",
        },
        Span::call_site(),
    );
    if !ignore_endian {
        return quote! {::byteorder::#ext::#func_id::<::byteorder::#endian>};
    }
    quote! {::byteorder::#ext::#func_id}
}

fn scan_attribute(field: &Field, little_endian: &mut bool, varint: &mut bool) {
    for attr in &(*field.attrs) {
        if let Meta::Path(path) = &attr.meta {
            let stream = path.to_token_stream().to_string();
            let full = stream.as_str();
            if full == "BigEndian" {
                *little_endian = false
            }
            if full == "Varint" {
                *varint = true
            }
        };
    }
}
