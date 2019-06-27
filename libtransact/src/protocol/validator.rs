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

use crate::protocol::codegen::{impl_proto_into_bytes, Builder, FromProtoImpl};
use crate::protocol::errors::BuilderError;
use crate::protocol::builder::Build;
use crate::protos;
use crate::protos::{
    FromBytes, FromNative, FromProto, IntoBytes, IntoNative, IntoProto, ProtoConversionError,
};


//enum MessageType {
    //TpRegisterRequest,
    //TpRegisterResponse,
    //TpUnregisterRequest,
    //TpUnregisterResponse,
    //TpProcessRequest,
    //TpProcessResponse,
    //TpStateGetRequest,
    //TpStateGetResponse,
    //TpStateSetRequest,
    //TpStateSetResponse,
    //TpStateDeleteRequest,
    //TpStateDeleteResponse,
    //TpReceiptAddDataRequest,
    //TpReceiptAddDataResponse,
    //TpEventAddRequest,
    //TpEventAddResponse
//}

#[derive(FromProtoImpl, Debug, Clone)]
#[proto_type = "protos::processor::TpRegisterRequest_TpProcessRequestHeaderStyle"]
pub enum TpProcessRequestHeaderStyle {
    #[from_proto_impl_enum(HEADER_STYLE_UNSET)]
    HeaderStyleUnset,

    #[from_proto_impl_enum(EXPANDED)]
    Expanded,

    #[from_proto_impl_enum(RAW)]
    Raw,
}

#[derive(Builder, FromProtoImpl, Debug)]
#[gen_build_impl]
#[proto_type = "protos::processor::TpRegisterRequest"]
pub struct TpRegisterRequest {

    #[getter]
    #[from_proto_impl(to_string)]
    family: String,

    #[getter]
    #[from_proto_impl(to_string)]
    version: String,

    #[getter]
    #[from_proto_impl(Vec)]
    namespaces: Vec<String>,

    #[getter]
    max_occupancy: u32,

    #[getter]
    protocol_version: u32,

    #[getter]
    #[from_proto_impl(from_proto)]
    request_header_style: TpProcessRequestHeaderStyle
}
