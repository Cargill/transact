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

macro_rules! __gen_field {
    ( $field:ident, req) => {
        let $field = $field.ok_or_else(|| BuilderError::MissingField(stringify!($field).into()))?;
    };
    ($field:ident, opt) => {
        let $field = $field.unwrap_or_default();
    };
}

/// Creates struct and implemnts corresponding builder class
///
/// # Example Usage
///
/// ```rust
///
/// impl_struct_with_builder!(FinalizeRecordActionBuilder => FinalizeRecordAction {
///     req record_id: String => &str,
///     opt name: String => &str
/// });
/// ````
///
/// The above invocation will generate the following struct and builder
///
/// ```rust
///
/// #[derive(Default, Debug, Clone, PartialEq)]
/// pub struct FinalizeRecordAction {
///     record_id: String,
///     name: String
/// }
///
/// impl FinalizeRecordAction {
///     # return values for getters are defined by `=> {type}`
///     pub fn record_id(&self) -> &str {
///         &self.record_id
///     }
///
///     pub fn name(&self) -> &str {
///         &self.name
///     }
/// }
///
/// #[derive(Default, Clone)]
/// pub struct FinalizeRecordActionBuilder {
///     record_id: Option<String>,
///     name: Option<String>
/// }
///
/// impl FinalizeRecordActionBuilder {
///     pub fn new() -> Self {
///        FinalizeRecordActionBuilder::default()
///     }
///
///     pub fn record_id(mut self, value: &str) -> Self {
///         self.record_id = Some(value);
///         self
///     }
///
///     pub fn name(mut self, value: &str) -> Self {
///         self.name = Some(value);
///         self
///     }
///
///     pub fn build(self) -> Result<FianlizeRecordAction, BuilderError> {
///
///         # Requires field because `req` was set
///         let record_id = self.record_id.ok_or_else(|| {
///            BuilderError::MissingField("record_id".into())
///        })?;
///
///        # Looks for default because `opt` was set
///        let name = self.name.unwrap_or_default();
///
///        FinalizeRecordAction {
///            record_id,
///            name
///        }
///     }
/// }
/// ```
///
macro_rules! impl_struct_with_builder {
    ($builder_name:ident => $struct_name:ident {
        $($req:ident $field:ident: $type:ty => $getter_type:ty), *
    }) => {
        #[derive(Default, Debug, Clone, PartialEq)]
        pub struct $struct_name {
            $($field: $type), *
        }

        impl $struct_name {
            $(pub fn $field(&self) -> $getter_type {
                &self.$field
            })*
        }

        #[derive(Default, Clone)]
        pub struct $builder_name {
            $($field: Option<$type>), *
        }

        impl $builder_name {
            pub fn new() -> Self {
                $builder_name::default()
            }

            $(pub fn $field(mut self, value: $type) -> Self {
                self.$field = Some(value);
                self
            })*

            pub fn build(self) -> Result<$struct_name, BuilderError> {
                $(
                    let $field = self.$field;
                    __gen_field!($field, $req);
                )*

                Ok($struct_name {
                    $($field), *
                })
            }
        }
    }
}

macro_rules! impl_from_bytes {
    ($native:ident, $proto:path) => {
        impl FromBytes<$native> for $native {
            fn from_bytes(bytes: &[u8]) -> Result<$native, ProtoConversionError> {
                let proto: $proto = protobuf::parse_from_bytes(bytes).map_err(|_| {
                    ProtoConversionError::SerializationError(format!(
                        "Unable to get {} from bytes",
                        stringify!($native)
                    ))
                })?;

                proto.into_native()
            }
        }
    };
}

macro_rules! impl_into_bytes {
    ($native:ident) => {
        impl IntoBytes for $native {
            fn into_bytes(self) -> Result<Vec<u8>, ProtoConversionError> {
                let proto = self.into_proto()?;
                let bytes = proto.write_to_bytes().map_err(|_| {
                    ProtoConversionError::SerializationError(format!(
                        "Unable to get {} from Bytes",
                        stringify!($native)
                    ))
                })?;
                Ok(bytes)
            }
        }
    };
}

macro_rules! impl_into_proto {
    ($native:ident, $proto:path) => {
        impl IntoProto<$proto> for $native {}
    };
}

macro_rules! impl_into_native {
    ($native:ident, $proto:path) => {
        impl IntoNative<$native> for $proto {}
    };
}

