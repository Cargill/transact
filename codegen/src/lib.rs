// Copyright 2019 Cargill Incorporated
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![recursion_limit = "128"]
extern crate proc_macro;

mod builder;
mod protos;

use proc_macro::TokenStream;
use builder::generate_builder_macro;
use syn::{parse_macro_input, DeriveInput, ItemStruct};
use protos::{generate_into_bytes_impl, generate_from_proto};
use quote::quote;

#[proc_macro_derive(Builder, attributes(builder_name, gen_build_impl, getter, optional))]
pub fn derive_builder(item: TokenStream) -> TokenStream {

    let derive_input = parse_macro_input!(item as DeriveInput);

    generate_builder_macro(derive_input) 
}

#[proc_macro_derive(FromProtoImpl, attributes(proto_type, from_proto_impl, from_proto_impl_enum))]
pub fn dervie_from_proto(item: TokenStream) -> TokenStream {
    
    let derive_input = parse_macro_input!(item as DeriveInput);

    generate_from_proto(derive_input)
        .map(|t| t.into())
        .unwrap_or_else(|err| {
            let compile_error = err.to_compile_error();
            quote!(#compile_error).into()
        })
}

#[proc_macro_attribute]
pub fn impl_proto_into_bytes(_: TokenStream, item: TokenStream) -> TokenStream {

    let parsed_item = item.clone();
    let stct = parse_macro_input!(parsed_item as ItemStruct);

    let into_bytes = generate_into_bytes_impl(stct);

    into_bytes
}
