use proc_macro::TokenStream;

use syn::__private::quote::__private::Span;
use syn::__private::quote::quote;
use syn::__private::ToTokens;
use syn::{parse_macro_input, Data, DeriveInput, Field, Ident, Meta};

#[proc_macro_derive(BStream, attributes(LittleEndian, BigEndian, Varint))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_identifier = &input.ident;

    match input.data {
        Data::Struct(syn::DataStruct { fields, .. }) => {
            let mut read = quote! {};
            let mut write = quote! {};

            for field in fields {
                let mut little_endian = true;
                let mut varint = false;
                scan_attribute(&field, &mut little_endian, &mut varint);
                read.extend(get_func(&field, little_endian, varint, true));
                write.extend(get_func(&field, little_endian, varint, false));
            }

            (quote! {
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
            })
            .into()
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
    let mut varint = varint;
    let mut func = String::from(match read {
        true => "read_",
        false => "write_",
    });
    if varint {
        func.push('v')
    }
    let mut endian = Ident::new("LittleEndian", Span::call_site());
    if !little_endian {
        endian = Ident::new("BigEndian", Span::call_site())
    }
    let field_id = &field.ident;
    let typ = field.ty.to_token_stream().to_string();

    func.push_str(&typ);
    let func_id = Ident::new(func.as_str(), Span::call_site());

    if typ == "bool" {
        varint = true
    }
    let ignore_endian = typ == "u8" || typ == "i8";
    if varint {
        return match read {
            true => {
                quote! {self.# field_id =::bstream::ReaderExt::# func_id(out) ?;}
            }
            false => {
                quote! {::bstream::WriterExt::# func_id(out,self.# field_id) ?;}
            }
        };
    }
    if !ignore_endian {
        return match read {
            true => {
                quote! {self.#field_id = ::byteorder::ReadBytesExt::#func_id::<::byteorder::#endian>(out)?;}
            }
            false => {
                quote! {::byteorder::WriteBytesExt::#func_id::<::byteorder::#endian>(out,self.#field_id)?;}
            }
        };
    }
    match read {
        true => {
            quote! {self.#field_id = ::byteorder::ReadBytesExt::#func_id(out)?;}
        }
        false => {
            quote! {::byteorder::WriteBytesExt::#func_id(out,self.#field_id)?;}
        }
    }
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
