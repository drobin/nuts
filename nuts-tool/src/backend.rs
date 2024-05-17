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

use log::error;
use nuts_backend::{Backend, Binary, Create, IdSize, Open, ReceiveHeader, HEADER_MAX_SIZE};
use nuts_tool_api::tool::{Plugin, PluginConnection, PluginError};
use std::cell::RefCell;
use std::collections::HashMap;
use std::str::FromStr;
use std::{cmp, fmt};

thread_local! {
    static ID_SIZE: RefCell<usize> = RefCell::new(0);
    static CONN: RefCell<Option<PluginConnection>> = RefCell::new(None);
}

fn setup_connection(mut connection: PluginConnection) -> Result<(), PluginError> {
    let id_size = connection.id_size()?;

    ID_SIZE.with(|size| *size.borrow_mut() = id_size);
    CONN.with(|cell| *cell.borrow_mut() = Some(connection));

    Ok(())
}

fn with_connection<T, F: FnOnce(&mut PluginConnection) -> Result<T, PluginError>>(
    f: F,
) -> Result<T, PluginError> {
    CONN.with_borrow_mut(|opt| match opt.as_mut() {
        Some(conn) => f(conn),
        None => Err(PluginError::NotConnected),
    })
}

#[derive(Clone, Debug)]
pub struct PluginSettings(Vec<u8>);

impl Binary for PluginSettings {
    fn from_bytes(bytes: &[u8]) -> Option<PluginSettings> {
        Some(PluginSettings(bytes.to_vec()))
    }

    fn as_bytes(&self) -> Vec<u8> {
        self.0.clone()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PluginId(Vec<u8>);

impl Binary for PluginId {
    fn from_bytes(bytes: &[u8]) -> Option<PluginId> {
        Some(PluginId(bytes.to_vec()))
    }

    fn as_bytes(&self) -> Vec<u8> {
        self.0.clone()
    }
}

impl IdSize for PluginId {
    fn size() -> usize {
        ID_SIZE.with(|n| *n.borrow())
    }
}

impl FromStr for PluginId {
    type Err = PluginError;

    fn from_str(s: &str) -> Result<PluginId, PluginError> {
        let bytes = with_connection(|conn| conn.id_string_to_bytes(s.to_string()))?;

        Ok(PluginId(bytes))
    }
}

impl fmt::Display for PluginId {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match with_connection(|conn| conn.id_bytes_to_string(self.0.clone())) {
            Ok(s) => fmt.write_str(&s),
            Err(_) => fmt.write_str("???"),
        }
    }
}

pub struct PluginBackendOpenBuilder;

impl PluginBackendOpenBuilder {
    pub fn new(plugin: Plugin, name: &str) -> Result<PluginBackendOpenBuilder, PluginError> {
        setup_connection(plugin.open(name)?)?;

        Ok(PluginBackendOpenBuilder)
    }
}

impl ReceiveHeader<PluginBackend> for PluginBackendOpenBuilder {
    fn get_header_bytes(&mut self, bytes: &mut [u8; HEADER_MAX_SIZE]) -> Result<(), PluginError> {
        let header = with_connection(|conn| conn.read_header())?;

        bytes.copy_from_slice(&header[..HEADER_MAX_SIZE]);

        Ok(())
    }
}

impl Open<PluginBackend> for PluginBackendOpenBuilder {
    fn build(self, settings: PluginSettings) -> Result<PluginBackend, PluginError> {
        with_connection(|conn| conn.open(settings.0.clone()))?;

        PluginBackend::new()
    }
}

pub struct PluginBackendCreateBuilder {
    settings: Vec<u8>,
}

impl PluginBackendCreateBuilder {
    pub fn new(plugin: Plugin, name: &str) -> Result<PluginBackendCreateBuilder, PluginError> {
        setup_connection(plugin.create(name)?)?;

        let settings = with_connection(|conn| conn.settings())?;

        Ok(PluginBackendCreateBuilder { settings })
    }
}

impl Create<PluginBackend> for PluginBackendCreateBuilder {
    fn settings(&self) -> PluginSettings {
        PluginSettings(self.settings.clone())
    }

    fn build(
        self,
        header: [u8; HEADER_MAX_SIZE],
        overwrite: bool,
    ) -> Result<PluginBackend, PluginError> {
        with_connection(|conn| conn.create(header.to_vec(), overwrite))?;

        PluginBackend::new()
    }
}

#[derive(Debug)]
pub struct PluginBackend {
    block_size: u32,
}

impl PluginBackend {
    fn new() -> Result<PluginBackend, PluginError> {
        let block_size = with_connection(|conn| conn.block_size())?;

        Ok(PluginBackend { block_size })
    }
}

impl ReceiveHeader<PluginBackend> for PluginBackend {
    fn get_header_bytes(&mut self, bytes: &mut [u8; HEADER_MAX_SIZE]) -> Result<(), PluginError> {
        let header = with_connection(|conn| conn.read_header())?;

        bytes.copy_from_slice(&header[..HEADER_MAX_SIZE]);

        Ok(())
    }
}

impl Backend for PluginBackend {
    type Settings = PluginSettings;
    type Err = PluginError;
    type Id = PluginId;
    type Info = HashMap<String, String>;

    fn info(&self) -> Result<Self::Info, PluginError> {
        with_connection(|conn| conn.info())
    }

    fn block_size(&self) -> u32 {
        self.block_size
    }

    fn aquire(&mut self, buf: &[u8]) -> Result<PluginId, PluginError> {
        let id = with_connection(|conn| conn.aquire(buf.to_vec()))?;

        Ok(PluginId(id))
    }

    fn release(&mut self, id: PluginId) -> Result<(), PluginError> {
        with_connection(|conn| conn.release(id.0))
    }

    fn read(&mut self, id: &PluginId, buf: &mut [u8]) -> Result<usize, PluginError> {
        let bytes = with_connection(|conn| conn.read(id.0.clone()))?;

        let n = cmp::min(bytes.len(), buf.len());
        buf.copy_from_slice(&bytes);

        Ok(n)
    }

    fn write(&mut self, id: &PluginId, buf: &[u8]) -> Result<usize, PluginError> {
        with_connection(|conn| conn.write(id.0.clone(), buf.to_vec()))
    }

    fn write_header(&mut self, buf: &[u8; HEADER_MAX_SIZE]) -> Result<(), PluginError> {
        with_connection(|conn| conn.write_header(buf.to_vec()))
    }

    fn delete(self) {
        if let Err(err) = with_connection(|conn| conn.delete()) {
            error!("failed to delete backend instance: {}", err);
        }
    }
}

impl Drop for PluginBackend {
    fn drop(&mut self) {
        if let Err(err) = with_connection(|conn| conn.quit()) {
            error!("failed to quit connection to plugin: {}", err);
        };
    }
}
