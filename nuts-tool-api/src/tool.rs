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

mod connection;
mod plugin;

use log::error;
use std::process::ExitStatus;
use std::sync::mpsc::{RecvError, SendError};
use thiserror::Error;

use crate::bson::BsonError;
use crate::msg::{ErrorResponse, Request, Response};

#[derive(Debug, Error)]
pub enum PluginError {
    #[error(transparent)]
    Bson(#[from] BsonError),

    #[error("{0:?}")]
    Response(ErrorResponse),

    #[error("invalid response")]
    InvalidResponse,

    #[error(transparent)]
    ChannelSendRequest(#[from] SendError<Request>),

    #[error(transparent)]
    ChannelSendResponse(#[from] SendError<Response>),

    #[error(transparent)]
    ChannelRecv(#[from] RecvError),

    #[error("channel is closed")]
    ChannelClosed,

    #[error("short plugin infos")]
    ShortPluginInfo,

    #[error("failed to fetch plugin information, plugin exited with {0}")]
    BadPluginInfo(ExitStatus),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("not connected")]
    NotConnected,
}

type PluginResult<T> = std::result::Result<T, PluginError>;

pub use connection::PluginConnection;
pub use plugin::Plugin;
