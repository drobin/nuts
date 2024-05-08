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

use log::{debug, error, info, log, trace, warn, Level};
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::panic;
use std::process::{Child, ChildStderr, ChildStdin, ChildStdout};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};

use crate::bson::{BsonReader, BsonWriter};
use crate::msg::{OkResponse, Request, Response};
use crate::tool::{PluginError, PluginResult};
use crate::PluginInfo;

fn stdin_thread(stdin: ChildStdin, rx_in: Receiver<Request>) -> PluginResult<()> {
    let mut writer = BsonWriter::new(stdin);

    for request in rx_in {
        debug!("stdin: sending {:?}", request);

        writer.write(request)?;
    }

    trace!("stdin: leaving thread");

    Ok(())
}

fn stdout_thread(stdout: ChildStdout, tx_out: Sender<Response>) -> PluginResult<()> {
    let mut reader = BsonReader::new(stdout);

    loop {
        match reader.read() {
            Ok(Some(response)) => {
                trace!("stdout: received {:?}", response);
                tx_out.send(response)?;
            }
            Ok(None) => {
                trace!("stdout: end of stream");
                break;
            }
            Err(err) => {
                error!("stdout: {:?}", err);
                return Err(err.into());
            }
        }
    }

    trace!("stdout: leaving thread");

    Ok(())
}

fn stderr_thread(stderr: ChildStderr) -> PluginResult<()> {
    let mut reader = BufReader::new(stderr);
    let mut line_buf = Vec::<u8>::new();

    loop {
        line_buf.clear();

        let n = reader.read_until(0x0A, &mut line_buf)?;

        if n > 0 {
            let line = String::from_utf8_lossy(&line_buf);

            if line.starts_with("nuts-log-error:") {
                error!("[plugin] {}", line[15..].trim());
            } else if line.starts_with("nuts-log-warn:") {
                warn!("[plugin] {}", line[14..].trim());
            } else if line.starts_with("nuts-log-info:") {
                info!("[plugin] {}", line[14..].trim());
            } else if line.starts_with("nuts-log-debug:") {
                debug!("[plugin] {}", line[15..].trim());
            } else if line.starts_with("nuts-log-trace:") {
                trace!("[plugin] {}", line[15..].trim());
            } else {
                error!("stderr: {}", line.trim_end());
            }
        } else {
            trace!("stderr: end of stream");
            break;
        }
    }

    trace!("stderr: leaving thread");

    Ok(())
}

macro_rules! handshake_func {
    ($name:ident ( $( $argn:ident : $argt:ty ),* ) -> $ty:ty, $req:expr, $variant:pat => $ret:tt) => {
        pub fn $name(&mut self, $($argn: $argt),*) -> PluginResult<$ty> {
            let response = self.handshake($req).map_err(|err| {
                error!("failed message handshake: {}",err);
                err
            })?;

            let result = match response {
                Response::Ok($variant) => Ok($ret),
                Response::Ok(_) => Err(PluginError::InvalidResponse),
                Response::Err(err) => Err(PluginError::Response(err)),
            };

            if result.is_err() {
                self.shutdown();
            }

            result
        }
    };
}

macro_rules! join_thread {
    ($id:literal, $opt:expr) => {
        if let Some(handle) = $opt.take() {
            debug!("join thread: {}", $id);

            if let Err(err) = handle.join() {
                panic::resume_unwind(err);
            }
        } else {
            debug!("nothing to join: {}", $id);
        }
    };
}

#[derive(Debug)]
pub struct PluginConnection {
    child: Child,
    tx_in: Option<Sender<Request>>,
    rx_out: Receiver<Response>,
    t_stdin: Option<JoinHandle<Result<(), PluginError>>>,
    t_stdout: Option<JoinHandle<Result<(), PluginError>>>,
    t_stderr: Option<JoinHandle<Result<(), PluginError>>>,
}

impl PluginConnection {
    pub(crate) fn new(mut child: Child) -> PluginConnection {
        let (tx_in, rx_in) = mpsc::channel();
        let (tx_out, rx_out) = mpsc::channel();

        let stdin = child.stdin.take().unwrap();
        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        let t_stdin = thread::spawn(move || stdin_thread(stdin, rx_in));
        let t_stdout = thread::spawn(move || stdout_thread(stdout, tx_out));
        let t_stderr = thread::spawn(move || stderr_thread(stderr));

        PluginConnection {
            child,
            tx_in: Some(tx_in),
            rx_out: rx_out,
            t_stdin: Some(t_stdin),
            t_stdout: Some(t_stdout),
            t_stderr: Some(t_stderr),
        }
    }

    pub fn plugin_info(&mut self) -> PluginResult<PluginInfo> {
        let response = self.handshake(Request::PluginInfo)?;

        match response {
            Response::Ok(OkResponse::Map(map)) => {
                let name = map
                    .get("name")
                    .map(ToString::to_string)
                    .ok_or(PluginError::InvalidResponse)?;
                let version = map
                    .get("version")
                    .map(ToString::to_string)
                    .ok_or(PluginError::InvalidResponse)?;

                Ok(PluginInfo { name, version })
            }
            Response::Ok(_) => Err(PluginError::InvalidResponse),
            Response::Err(err) => Err(PluginError::Response(err)),
        }
    }

    handshake_func!(id_string_to_bytes(str: String) -> Vec<u8>, Request::IdToBytes(str), OkResponse::Bytes(bytes) => bytes);
    handshake_func!(id_bytes_to_string(bytes: Vec<u8>) -> String, Request::IdToString(bytes), OkResponse::String(str) => str);
    handshake_func!(settings() -> Vec<u8>, Request::Settings, OkResponse::Bytes(bytes) => bytes);
    handshake_func!(id_size() -> usize, Request::IdSize, OkResponse::Usize(num) => num);
    handshake_func!(block_size() -> u32, Request::BlockSize, OkResponse::U32(num) => num);
    handshake_func!(open(settings: Vec<u8>) -> (), Request::Open(settings), OkResponse::Void => ());
    handshake_func!(create(header: Vec<u8>, overwrite: bool) -> (), Request::Create(header, overwrite), OkResponse::Void => ());
    handshake_func!(info() -> HashMap<String, String>, Request::Info, OkResponse::Map(map) => map);
    handshake_func!(aquire(bytes: Vec<u8>) -> Vec<u8>, Request::Aquire(bytes), OkResponse::Bytes(bytes) => bytes);
    handshake_func!(release(id: Vec<u8>) -> (), Request::Release(id), OkResponse::Void => ());
    handshake_func!(read_header() -> Vec<u8>, Request::ReadHeader, OkResponse::Bytes(bytes) => bytes);
    handshake_func!(write_header(bytes: Vec<u8>) -> (), Request::WriteHeader(bytes), OkResponse::Void => ());
    handshake_func!(read(id: Vec<u8>) -> Vec<u8>, Request::Read(id), OkResponse::Bytes(bytes) => bytes);
    handshake_func!(write(id: Vec<u8>, bytes: Vec<u8>) -> usize, Request::Write(id, bytes), OkResponse::Usize(num) => num);
    handshake_func!(delete() -> (), Request::Delete, OkResponse::Void => ());
    handshake_func!(quit() -> (), Request::Quit, OkResponse::Void => ());

    fn handshake(&mut self, request: Request) -> PluginResult<Response> {
        let tx = self.tx_in.as_mut().unwrap();

        tx.send(request)?;

        match self.rx_out.recv() {
            Ok(response) => Ok(response),
            Err(err) => {
                error!("{}", err);
                self.shutdown();
                Err(PluginError::ChannelClosed)
            }
        }
    }

    fn shutdown(&mut self) {
        if let Some(tx_in) = self.tx_in.take() {
            debug!("shutdown plugin connection");

            // Drop the channel in the stdin-thread,
            // let the thread leave its loop
            drop(tx_in);

            match self.child.wait() {
                Ok(exit_status) => {
                    let level = match exit_status.success() {
                        true => Level::Debug,
                        false => Level::Error,
                    };

                    log!(level, "plugin exited with {}", exit_status);
                }
                Err(err) => error!("could not wait for plugin: {}", err),
            };

            join_thread!("stdin", self.t_stdin);
            join_thread!("stdout", self.t_stdout);
            join_thread!("stderr", self.t_stderr);
        } else {
            debug!("already shutdown");
        }
    }
}

impl Drop for PluginConnection {
    fn drop(&mut self) {
        self.shutdown()
    }
}
