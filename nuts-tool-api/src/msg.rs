// MIT License
//
// Copyright (c) 2024 Robin Doer
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to
// deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

macro_rules! as_into_impls {
    ($as_name:ident + $into_name:ident => $variant:ident) => {
        pub fn $as_name(&self) -> Option<()> {
            match self {
                Self::$variant => Some(()),
                _ => None,
            }
        }

        pub fn $into_name(self) -> Option<()> {
            match self {
                Self::$variant => Some(()),
                _ => None,
            }
        }
    };

    ($as_name:ident + $into_name:ident => $variant:ident ( $arg:ident : $type:ty )) => {
        pub fn $as_name(&self) -> Option<&$type> {
            match self {
                Self::$variant($arg) => Some($arg),
                _ => None,
            }
        }

        pub fn $into_name(self) -> Option<$type> {
            match self {
                Self::$variant($arg) => Some($arg),
                _ => None,
            }
        }
    };

    ($as_name:ident + $into_name:ident => $variant:ident ( $( $arg:ident : $type:ty ),+ )) => {
        pub fn $as_name(&self) -> Option<( $( &$type ),+)> {
            match self {
                Self::$variant($( $arg ),+) => Some(($( $arg ),+)),
                _ => None,
            }
        }

        pub fn $into_name(self) -> Option<( $( $type ),+)> {
            match self {
                Self::$variant($( $arg ),+) => Some(($( $arg ),+)),
                _ => None,
            }
        }
    };
}

/// The request message.
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "op", content = "args", rename_all = "kebab-case")]
pub enum Request {
    /// Ask for plugin information.
    ///
    /// * The response must be a [`OkResponse::Map`] variant.
    PluginInfo,

    /// Ask for settings.
    ///
    /// * The response must be a [`OkResponse::Bytes`] variant.
    Settings,

    /// Ask for the size of the id of the backend.
    ///
    /// * The response must be a [`OkResponse::Usize`] variant.
    IdSize,

    /// Ask for the block-size of the backend.
    ///
    /// * The response must be a [`OkResponse::U32`] variant.
    BlockSize,

    /// Ask to convert a string-representation of an id into bytes.
    ///
    /// * The argument contains the string representation.
    /// * The response must be a [`OkResponse::Bytes`] variant.
    IdToBytes(String),

    /// Ask to convert the bytes representation of an id into a string.
    ///
    /// * The argument contains the byte data.
    /// * The response must be a [`OkResponse::String`] variant.
    IdToString(Vec<u8>),

    /// Request to open a backend-instance.
    ///
    /// * The argument contains binary data of the settings of the backend.
    /// * The response must be a [`OkResponse::Void`] variant.
    Open(Vec<u8>),

    /// Request to create a new backend-instance.
    ///
    /// * The first argument contains the binary data of the header.
    /// * The second argument contains the overwrite flag.
    /// * The response must be a [`OkResponse::Void`] variant.
    Create(Vec<u8>, bool),

    /// Ask for backend information.
    ///
    /// * The response must be a [`OkResponse::Map`] variant.
    Info,

    /// Request to aquire a new block in the backend.
    ///
    /// * The argument contains the initial data of the block.
    /// * The response must be a [`OkResponse::Bytes`] variant.
    Aquire(Vec<u8>),

    /// Request to release a block in the backend.
    ///
    /// * The argument contains the binary data of the id to release.
    /// * The response must be a [`OkResponse::Void`] variant.
    Release(Vec<u8>),

    /// Request to read the header data of the backend.
    ///
    /// * The response must be a [`OkResponse::Bytes`] variant.
    ReadHeader,

    /// Requerst to write the header of the backend.
    ///
    /// * The argument contains the header data to be written.
    /// * The response must be a [`OkResponse::Void`] variant.
    WriteHeader(Vec<u8>),

    /// Request to read a block in the backend.
    ///
    /// * The argument contains the binary data of the id to read.
    /// * The response must be a [`OkResponse::Bytes`] variant.
    Read(Vec<u8>),

    /// Request to write a block in the backend.
    ///
    /// * The first argument contains the binary data of the id to read.
    /// * The second argument contains the data to be written.
    /// * The response must be a [`OkResponse::Usize`] variant.
    Write(Vec<u8>, Vec<u8>),

    /// Asks to delete the backend.
    ///
    /// * The response must be a [`OkResponse::Void`] variant.
    Delete,

    /// Asks to quit the connection.
    ///
    /// * The response must be a [`OkResponse::Void`] variant.
    Quit,
}

impl Request {
    as_into_impls!(as_plugin_info + into_plugin_info => PluginInfo);
    as_into_impls!(as_settings + into_settings => Settings);
    as_into_impls!(as_id_size + into_id_size => IdSize);
    as_into_impls!(as_block_size + into_block_size => BlockSize);
    as_into_impls!(as_id_to_bytes + into_id_to_bytes => IdToBytes(arg1: String));
    as_into_impls!(as_id_to_string + into_id_to_string => IdToString(arg1: Vec<u8>));
    as_into_impls!(as_open + into_open => Open (arg1: Vec<u8>));
    as_into_impls!(as_create + into_create => Create (arg1: Vec<u8>, args: bool));
    as_into_impls!(as_info + into_info => Info);
    as_into_impls!(as_aquire + into_aquire => Aquire (arg1: Vec<u8>));
    as_into_impls!(as_release + into_release => Release (arg1: Vec<u8>));
    as_into_impls!(as_read_header + into_read_header => ReadHeader);
    as_into_impls!(as_write_header + into_write_header => WriteHeader (arg1: Vec<u8>));
    as_into_impls!(as_read + into_read => Read (arg1: Vec<u8>));
    as_into_impls!(as_write + into_write => Write (arg1: Vec<u8>, arg2: Vec<u8>));
    as_into_impls!(as_delete + into_delete => Delete);
    as_into_impls!(as_quit + into_quit => Quit);
}

/// The response message.
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "code", content = "args", rename_all = "kebab-case")]
pub enum Response {
    /// A successful response.
    Ok(OkResponse),

    /// An error response.
    Err(ErrorResponse),
}

impl Response {
    /// Creates a successful response with an attached [`OkResponse::Void`].
    pub fn ok_void() -> Response {
        Self::Ok(OkResponse::Void)
    }

    /// Creates a successful response with an attached [`OkResponse::U32`].
    pub fn ok_u32(value: u32) -> Response {
        Self::Ok(OkResponse::U32(value))
    }

    /// Creates a successful response with an attached [`OkResponse::Usize`].
    pub fn ok_usize(value: usize) -> Response {
        Self::Ok(OkResponse::Usize(value))
    }

    /// Creates a successful response with an attached [`OkResponse::Bytes`].
    pub fn ok_bytes(value: Vec<u8>) -> Response {
        Self::Ok(OkResponse::Bytes(value))
    }

    /// Creates a successful response with an attached [`OkResponse::String`].
    pub fn ok_string(value: String) -> Response {
        Self::Ok(OkResponse::String(value))
    }

    /// Creates a successful response with an attached [`OkResponse::Map`].
    pub fn ok_map(value: HashMap<String, String>) -> Response {
        Self::Ok(OkResponse::Map(value))
    }

    /// Creates an error response with an attached
    /// [`ErrorResponse::NotApplicable`].
    pub fn err_not_applicable() -> Response {
        Self::Err(ErrorResponse::NotApplicable)
    }

    /// Creates an error response with an attached [`ErrorResponse::Message`].
    pub fn err_message<M: AsRef<str>>(msg: M) -> Response {
        Self::Err(ErrorResponse::Message(msg.as_ref().to_string()))
    }

    as_into_impls!(as_ok + into_ok => Ok (ok: OkResponse));
    as_into_impls!(as_error + into_error => Err (err: ErrorResponse));
}

/// A successful response.
///
/// This is a container for various return types.
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", content = "args", rename_all = "kebab-case")]
pub enum OkResponse {
    /// A successful response without an attached value.
    Void,

    /// A successful response with an attached [`u32`].
    U32(u32),

    /// A successful response with an attached [`usize`].
    Usize(usize),

    /// A successful response with an attached [`Vec<u8>`].
    Bytes(Vec<u8>),

    /// A successful response with an attached [`String`].
    String(String),

    /// A successful response with an attached [`HashMap`].
    Map(HashMap<String, String>),
}

/// An error response.
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "code", content = "args", rename_all = "kebab-case")]
pub enum ErrorResponse {
    /// The call is not applicable in the current backend state.
    ///
    /// I.e. you need an open backend instance for a request call but the
    /// backend was not opened yet.
    NotApplicable,

    /// Could not convert an id into its binary representation.
    InvalidId,

    /// Could not create an id from its binary representation.
    InvalidIdData,

    /// Could not convert the settings into its binary representation.
    InvalidSettings,

    /// Could not create the settings from its binary representation.
    InvalidSettingsData,

    /// Could not convert backend information into a hash representation.
    InvalidInfo,

    /// The header data has an invalid length.
    InvalidHeaderBytes,

    /// An arbitrary error with the given message occured.
    Message(String),
}

impl<T: fmt::Display> From<T> for ErrorResponse {
    fn from(msg: T) -> Self {
        Self::Message(msg.to_string())
    }
}
