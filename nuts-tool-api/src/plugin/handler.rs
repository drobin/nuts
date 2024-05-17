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

use clap::Args;
use log::debug;
use nuts_backend::{Backend, Binary, Create, IdSize, Open, ReceiveHeader, HEADER_MAX_SIZE};
use std::collections::HashMap;
use std::convert::TryInto;
use std::marker::PhantomData;
use std::str::FromStr;
use std::{cmp, io};

use crate::bson::{BsonError, BsonReader, BsonWriter};
use crate::msg::{ErrorResponse, Request, Response};
use crate::plugin::cli::{CreateArgs, Format, InfoArgs, OpenArgs, PluginCommand};
use crate::PluginInfo;

fn into_header_bytes(bytes: &[u8]) -> Result<[u8; HEADER_MAX_SIZE], ErrorResponse> {
    match bytes.try_into() {
        Ok(header) => Ok(header),
        Err(_err) => Err(ErrorResponse::InvalidHeaderBytes),
    }
}

/// Trait enriches the generic implementation of the plugin with
/// plugin-specific behavior.
///
/// You need to implement a set of functions that provide plugin-specific
/// information.
///
/// There are several `handle_*` functions, which are already implemented. They
/// encapsulates the handling for all the [request types](Request).
pub trait PluginHandler<B: Backend> {
    /// Extra (plugin-specific) arguments passed to the `create` command.
    type CreateArgs: Args;

    /// The create builder for the attached backend.
    type Create: Create<B>;

    /// The open builder for the attached backend.
    type Open: Open<B>;

    /// Returns information of the plugin.
    fn plugin_info(&self) -> PluginInfo;

    /// Converts the given [`info`](Backend::Info) into a [`HashMap`].
    ///
    /// The hash is send back to the *nuts-tool* and displayed to the user. So
    /// you are advised to choose good, human readable keys.
    ///
    /// On success the info-hash wrapped into a [`Some`] value is returned. If
    /// you cannot create the hash [`None`] is returned.
    fn info_to_hash(&self, info: B::Info) -> Option<HashMap<String, String>>;

    /// Creates a builder instance used to open an already existing backend
    /// instance.
    fn open_builder(&self, args: &OpenArgs) -> Option<Self::Open>;

    /// Creates a builder instance used to create a new backend instance.
    fn create_builder(&self, args: &CreateArgs<Self::CreateArgs>) -> Option<Self::Create>;

    /// Handles the [`Request::PluginInfo`] command.
    fn handle_plugin_info(&self) -> Result<HashMap<String, String>, ErrorResponse> {
        let info = self.plugin_info();

        Ok([
            ("name".to_string(), info.name),
            ("version".to_string(), info.version),
        ]
        .into())
    }

    /// Handles the [`Request::Settings`] command.
    fn handle_settings(
        &self,
        args: &CreateArgs<Self::CreateArgs>,
    ) -> Result<Vec<u8>, ErrorResponse> {
        match self.create_builder(args) {
            Some(builder) => {
                let settings = builder.settings();

                Ok(<B::Settings as Binary>::as_bytes(&settings))
            }
            None => Err(ErrorResponse::from("unable to make a create-builder")),
        }
    }

    /// Handles the [`Request::Open`] command.
    fn handle_open(&self, args: &OpenArgs, settings: &[u8]) -> Result<B, ErrorResponse> {
        let settings = <B::Settings as Binary>::from_bytes(settings)
            .ok_or(ErrorResponse::InvalidSettingsData)?;

        match self.open_builder(args) {
            Some(builder) => builder.build(settings).map_err(|err| err.into()),
            None => Err(ErrorResponse::from("unable to make an open-builder")),
        }
    }

    /// Handles the [`Request::Create`] command.
    fn handle_create(
        &self,
        args: &CreateArgs<Self::CreateArgs>,
        header: &[u8],
        overwrite: bool,
    ) -> Result<B, ErrorResponse> {
        let header = into_header_bytes(header)?;

        match self.create_builder(args) {
            Some(builder) => builder.build(header, overwrite).map_err(|err| err.into()),
            None => Err(ErrorResponse::from("unable to make a create-builder")),
        }
    }

    /// Handles the [`Request::IdSize`] command.
    fn handle_id_size(&self) -> Result<usize, ErrorResponse> {
        Ok(<B::Id as IdSize>::size())
    }

    /// Handles the [`Request::BlockSize`] command.
    fn handle_block_size(&self, backend: &B) -> Result<u32, ErrorResponse> {
        Ok(B::block_size(backend))
    }

    /// Handles the [`Request::IdToBytes`] command.
    fn handle_id_to_bytes(&self, str: &str) -> Result<Vec<u8>, ErrorResponse> {
        let id = <B::Id as FromStr>::from_str(str).map_err(|_| ErrorResponse::InvalidIdData)?;

        Ok(<B::Id as Binary>::as_bytes(&id))
    }

    /// Handles the [`Request::IdToString`] command.
    fn handle_id_to_string(&self, bytes: &[u8]) -> Result<String, ErrorResponse> {
        let id = <B::Id as Binary>::from_bytes(bytes).ok_or(ErrorResponse::InvalidIdData)?;

        Ok(id.to_string())
    }

    /// Handles the [`Request::Info`] command.
    fn handle_info(&self, backend: &B) -> Result<HashMap<String, String>, ErrorResponse> {
        match backend.info() {
            Ok(info) => Ok(self.info_to_hash(info).ok_or(ErrorResponse::InvalidInfo)?),
            Err(err) => Err(err.into()),
        }
    }

    /// Handles the [`Request::Aquire`] command.
    fn handle_aquire(&self, backend: &mut B, bytes: &[u8]) -> Result<Vec<u8>, ErrorResponse> {
        match B::aquire(backend, bytes) {
            Ok(id) => Ok(<B::Id as Binary>::as_bytes(&id)),
            Err(err) => Err(err.into()),
        }
    }

    /// Handles the [`Request::Release`] command.
    fn handle_release(&self, backend: &mut B, id: &[u8]) -> Result<(), ErrorResponse> {
        let id = <B::Id as Binary>::from_bytes(id).ok_or(ErrorResponse::InvalidIdData)?;

        B::release(backend, id).map_err(|err| err.into())
    }

    /// Handles the [`Request::ReadHeader`] command.
    fn handle_read_header<T: ReceiveHeader<B>>(
        &self,
        header: &mut T,
    ) -> Result<Vec<u8>, ErrorResponse> {
        let mut bytes = [0; HEADER_MAX_SIZE];

        match header.get_header_bytes(&mut bytes) {
            Ok(()) => Ok(bytes.to_vec()),
            Err(err) => Err(err.into()),
        }
    }

    /// Handles the [`Request::WriteHeader`] command.
    fn handle_write_header(&self, backend: &mut B, header: &[u8]) -> Result<(), ErrorResponse> {
        let header = into_header_bytes(header)?;

        B::write_header(backend, &header).map_err(|err| err.into())
    }

    /// Handles the [`Request::Read`] command.
    fn handle_read(&self, backend: &mut B, id: &[u8]) -> Result<Vec<u8>, ErrorResponse> {
        let id = <B::Id as Binary>::from_bytes(id).ok_or(ErrorResponse::InvalidIdData)?;
        let bsize = B::block_size(backend) as usize;
        let mut buf = vec![0; bsize];

        let nread = B::read(backend, &id, &mut buf)?;

        Ok(buf[..nread].to_vec())
    }

    /// Handles the [`Request::Write`] command.
    fn handle_write(
        &self,
        backend: &mut B,
        id: &[u8],
        bytes: &[u8],
    ) -> Result<usize, ErrorResponse> {
        let id = <B::Id as Binary>::from_bytes(id).ok_or(ErrorResponse::InvalidIdData)?;

        let bsize = B::block_size(backend) as usize;
        let nbytes = cmp::min(bytes.len(), bsize);
        let bytes = &bytes[..nbytes];

        B::write(backend, &id, bytes).map_err(|err| err.into())
    }

    fn handle_delete(&self, backend: B) -> Result<(), ErrorResponse> {
        B::delete(backend);
        Ok(())
    }

    fn handle_quit(&self) -> Result<(), ErrorResponse> {
        Ok(())
    }
}

/// Handler for the `info` command.
pub struct InfoHandler<B, T> {
    handler: T,
    _data: PhantomData<B>,
}

impl<B: Backend, T: PluginHandler<B>> InfoHandler<B, T> {
    /// Creates a new `InfoHandler` and attaches the given [`PluginHandler`] to
    /// the instance.
    pub fn new(handler: T) -> InfoHandler<B, T> {
        InfoHandler {
            handler,
            _data: PhantomData,
        }
    }

    /// Runs the handler.
    ///
    /// The `args` arguments contains arguments passed to the command line.
    pub fn run(&self, args: &InfoArgs) -> Result<(), BsonError> {
        let info = self.handler.plugin_info();

        match args.format {
            Format::Text => {
                println!("name:    {}", info.name);
                println!("version: {}", info.version);
            }
            Format::Bson => {
                BsonWriter::new(io::stdout()).write(info)?;
            }
        };

        Ok(())
    }
}

/// Handler for the `open` and `create` commands.
pub struct OpenCreateHandler<'a, B: Backend, T: PluginHandler<B>> {
    command: &'a PluginCommand<T::CreateArgs>,
    handler: T,
    backend: Option<B>,
}

impl<'a, B: Backend, T: PluginHandler<B>> OpenCreateHandler<'a, B, T> {
    /// Creates a new `OpenCreateHandler` and attaches the given
    /// [`PluginHandler`] to the instance. `command` is the command, which is
    /// executed on the command line. It can be eiter `open` or `create`.
    pub fn new(
        command: &'a PluginCommand<T::CreateArgs>,
        handler: T,
    ) -> OpenCreateHandler<'a, B, T> {
        OpenCreateHandler {
            command,
            handler,
            backend: None,
        }
    }

    /// Runs the handler.
    pub fn run(&mut self) -> Result<(), BsonError> {
        let mut reader = BsonReader::new(io::stdin());
        let mut writer = BsonWriter::new(io::stdout());

        let mut leave_loop = false;

        while !leave_loop {
            match reader.read::<Request>() {
                Ok(Some(request)) => {
                    debug!("request: {:?}", request);

                    let response = match request {
                        Request::PluginInfo => self.on_plugin_info(),
                        Request::Settings => self.on_settings(),
                        Request::Open(ref settings) => self.on_open(settings),
                        Request::Create(ref header, overwrite) => self.on_create(header, overwrite),
                        Request::IdSize => self.on_id_size(),
                        Request::BlockSize => self.on_block_size(),
                        Request::IdToBytes(ref str) => self.on_id_to_bytes(str),
                        Request::IdToString(ref bytes) => self.on_id_to_string(bytes),
                        Request::Info => self.on_info(),
                        Request::Aquire(ref bytes) => self.on_aquire(bytes),
                        Request::Release(ref id) => self.on_release(id),
                        Request::ReadHeader => self.on_read_header(),
                        Request::WriteHeader(ref header) => self.on_write_header(header),
                        Request::Read(ref id) => self.on_read(id),
                        Request::Write(ref id, ref bytes) => self.on_write(id, bytes),
                        Request::Delete => self.on_delete(),
                        Request::Quit => self.on_quit(),
                    };

                    leave_loop = request.as_quit().is_some() || response.as_error().is_some();
                    debug!("response: {:?}, leave: {}", response, leave_loop);

                    writer.write(response)?;
                }
                Ok(None) => {
                    leave_loop = true;
                }
                Err(err) => return Err(err),
            }
        }

        Ok(())
    }

    fn on_plugin_info(&mut self) -> Response {
        match self.handler.handle_plugin_info() {
            Ok(info) => Response::ok_map(info),
            Err(err) => Response::Err(err),
        }
    }

    fn on_settings(&mut self) -> Response {
        if let Some(args) = self.command.as_create() {
            match self.handler.handle_settings(args) {
                Ok(settings) => Response::ok_bytes(settings),
                Err(err) => Response::Err(err),
            }
        } else {
            Response::err_not_applicable()
        }
    }

    fn on_open(&mut self, settings: &[u8]) -> Response {
        if let Some(args) = self.command.as_open() {
            match self.handler.handle_open(args, settings) {
                Ok(backend) => {
                    self.backend = Some(backend);
                    Response::ok_void()
                }
                Err(err) => Response::Err(err),
            }
        } else {
            Response::err_not_applicable()
        }
    }

    fn on_create(&mut self, header: &[u8], overwrite: bool) -> Response {
        if let Some(args) = self.command.as_create() {
            match self.handler.handle_create(args, header, overwrite) {
                Ok(backend) => {
                    self.backend = Some(backend);
                    Response::ok_void()
                }
                Err(err) => Response::Err(err),
            }
        } else {
            Response::err_not_applicable()
        }
    }

    fn on_id_size(&mut self) -> Response {
        match self.handler.handle_id_size() {
            Ok(size) => Response::ok_usize(size),
            Err(err) => Response::Err(err),
        }
    }

    fn on_block_size(&mut self) -> Response {
        if let Some(backend) = self.backend.as_ref() {
            match self.handler.handle_block_size(backend) {
                Ok(size) => Response::ok_u32(size),
                Err(err) => Response::Err(err),
            }
        } else {
            Response::err_not_applicable()
        }
    }

    fn on_id_to_bytes(&mut self, str: &str) -> Response {
        match self.handler.handle_id_to_bytes(str) {
            Ok(bytes) => Response::ok_bytes(bytes),
            Err(err) => Response::Err(err),
        }
    }

    fn on_id_to_string(&mut self, bytes: &[u8]) -> Response {
        match self.handler.handle_id_to_string(bytes) {
            Ok(str) => Response::ok_string(str),
            Err(err) => Response::Err(err),
        }
    }

    fn on_info(&mut self) -> Response {
        if let Some(backend) = self.backend.as_ref() {
            match self.handler.handle_info(backend) {
                Ok(info) => Response::ok_map(info),
                Err(err) => Response::Err(err),
            }
        } else {
            Response::err_not_applicable()
        }
    }

    fn on_aquire(&mut self, bytes: &[u8]) -> Response {
        if let Some(backend) = self.backend.as_mut() {
            match self.handler.handle_aquire(backend, bytes) {
                Ok(id) => Response::ok_bytes(id),
                Err(err) => Response::Err(err),
            }
        } else {
            Response::err_not_applicable()
        }
    }

    fn on_release(&mut self, id: &[u8]) -> Response {
        if let Some(backend) = self.backend.as_mut() {
            match self.handler.handle_release(backend, id) {
                Ok(()) => Response::ok_void(),
                Err(err) => Response::Err(err),
            }
        } else {
            Response::err_not_applicable()
        }
    }

    fn on_read_header(&mut self) -> Response {
        if let Some(args) = self.command.as_open() {
            if let Some(mut builder) = self.handler.open_builder(args) {
                match self.handler.handle_read_header(&mut builder) {
                    Ok(header) => Response::ok_bytes(header),
                    Err(err) => Response::Err(err),
                }
            } else {
                Response::err_message("unable to build an open-builder")
            }
        } else if let Some(backend) = self.backend.as_mut() {
            match self.handler.handle_read_header(backend) {
                Ok(header) => Response::ok_bytes(header),
                Err(err) => Response::Err(err),
            }
        } else {
            Response::err_not_applicable()
        }
    }

    fn on_write_header(&mut self, header: &[u8]) -> Response {
        if let Some(backend) = self.backend.as_mut() {
            match self.handler.handle_write_header(backend, header) {
                Ok(()) => Response::ok_void(),
                Err(err) => Response::Err(err),
            }
        } else {
            Response::err_not_applicable()
        }
    }

    fn on_read(&mut self, id: &[u8]) -> Response {
        if let Some(backend) = self.backend.as_mut() {
            match self.handler.handle_read(backend, id) {
                Ok(data) => Response::ok_bytes(data),
                Err(err) => Response::Err(err),
            }
        } else {
            Response::err_not_applicable()
        }
    }

    fn on_write(&mut self, id: &[u8], bytes: &[u8]) -> Response {
        if let Some(backend) = self.backend.as_mut() {
            match self.handler.handle_write(backend, id, bytes) {
                Ok(n) => Response::ok_usize(n),
                Err(err) => Response::Err(err),
            }
        } else {
            Response::err_not_applicable()
        }
    }

    fn on_delete(&mut self) -> Response {
        if let Some(backend) = self.backend.take() {
            match self.handler.handle_delete(backend) {
                Ok(()) => Response::ok_void(),
                Err(err) => Response::Err(err),
            }
        } else {
            Response::err_not_applicable()
        }
    }

    fn on_quit(&self) -> Response {
        match self.handler.handle_quit() {
            Ok(()) => Response::ok_void(),
            Err(err) => Response::Err(err),
        }
    }
}
