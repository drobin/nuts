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

#[allow(dead_code)]
mod common;
#[allow(dead_code)]
mod predicates_ext;

use assert_cmd::assert::Assert;
use assert_cmd::Command;
use assert_fs::fixture::TempDir;
use hex;
use predicates::prelude::PredicateBooleanExt;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::str;

use crate::common::{
    container_create, handle_password_args, handle_password_file, nuts_tool, plugin_add,
    plugin_path, plugin_remove, setup,
};
use crate::predicates_ext::{hash, list};

fn container_acquire(home: &Path, name: &str, pass: Option<&[u8]>) -> Command {
    let cmd = nuts_tool(home, ["container", "aquire", "--container", name]);

    handle_password_args(cmd, pass)
}

fn container_attach(home: &Path, name: &str, plugin: &str) -> Command {
    nuts_tool(home, ["container", "attach", "--container", name, plugin])
}

fn container_change_kdf(home: &Path, name: &str, kdf: &str, pass: Option<&[u8]>) -> Command {
    let cmd = nuts_tool(
        home,
        ["container", "change", "kdf", "--container", name, kdf],
    );

    handle_password_args(cmd, pass)
}

fn container_change_password(home: &Path, name: &str, pass: Option<&[u8]>) -> Command {
    let cmd = nuts_tool(
        home,
        ["container", "change", "password", "--container", name],
    );

    handle_password_args(cmd, pass)
}

fn container_delete(home: &Path, name: &str, pass: Option<&[u8]>) -> Command {
    let cmd = nuts_tool(home, ["container", "delete", "--container", name]);

    handle_password_args(cmd, pass)
}

fn container_info(home: &Path, name: &str, pass: Option<&[u8]>) -> Command {
    let cmd = nuts_tool(home, ["container", "info", "--container", name]);

    handle_password_args(cmd, pass)
}

fn container_list(home: &Path) -> Command {
    nuts_tool(home, ["container", "list"])
}

fn container_read(home: &Path, name: &str, id: &str, pass: Option<&[u8]>) -> Command {
    let cmd = nuts_tool(home, ["container", "read", "--container", name, id]);

    handle_password_args(cmd, pass)
}

fn container_release(home: &Path, name: &str, id: &str, pass: Option<&[u8]>) -> Command {
    let cmd = nuts_tool(home, ["container", "release", "--container", name, id]);

    handle_password_args(cmd, pass)
}

fn container_write(
    home: &Path,
    name: &str,
    id: Option<&str>,
    data: &[u8],
    pass: Option<&[u8]>,
) -> Command {
    let mut args = vec!["container", "write", "--container", name];

    if let Some(s) = id {
        args.push(s);
    }

    let mut cmd = nuts_tool(home, args);

    cmd.write_stdin(data);

    handle_password_file(home, cmd, "--password-from-file", pass)
}

fn default_info_with<'a>(values: HashMap<&'a str, &'a str>) -> HashMap<&'a str, &'a str> {
    let mut hash: HashMap<&str, &str> = [
        ("plugin", "directory"),
        ("revision", "2"),
        ("cipher", "aes256-gcm"),
        ("kdf", "pbkdf2:sha256:65536:16"),
        ("block size (gross)", "512"),
        ("block size (net)", "496"),
        ("block_size", "512"),
    ]
    .into();

    for (key, value) in values.iter() {
        assert!(hash.insert(key, value).is_some());
    }

    hash
}

fn id_from_acquire_stdout(assert: Assert) -> String {
    str::from_utf8(&assert.get_output().stdout[9..])
        .unwrap()
        .trim_end()
        .to_string()
}

fn setup_new_plugin(home: &Path) {
    let plugin = plugin_path("nuts-directory");
    let new_plugin = home.join("new_plugin");

    fs::copy(&plugin, &new_plugin).unwrap();
    plugin_add(home, "new_plugin", new_plugin.to_str().unwrap())
        .assert()
        .success();
}

#[test]
fn help() {
    let tmp_dir = TempDir::new().unwrap();

    for args in [
        ["container", "attach", "--help"].as_slice(),
        ["container", "list", "--help"].as_slice(),
    ] {
        let no_password_from_fd = predicates::str::contains("--password-from-fd").not();
        let no_password_from_file = predicates::str::contains("--password-from-file").not();
        let verbose = predicates::str::contains("--verbose");
        let quiet = predicates::str::contains("--quiet");

        nuts_tool(&tmp_dir, args)
            .assert()
            .success()
            .stdout(
                no_password_from_fd
                    .and(no_password_from_file)
                    .and(verbose)
                    .and(quiet),
            )
            .stderr("");
    }

    for args in [
        ["container", "--help"].as_slice(), // FIXME
        ["container", "aquire", "--help"].as_slice(),
        ["container", "change", "--help"].as_slice(), // FIXME
        ["container", "change", "password", "--help"].as_slice(),
        ["container", "change", "kdf", "--help"].as_slice(),
        ["container", "create", "--help"].as_slice(),
        ["container", "delete", "--help"].as_slice(),
        ["container", "info", "--help"].as_slice(),
        ["container", "read", "--help"].as_slice(),
        ["container", "release", "--help"].as_slice(),
        ["container", "write", "--help"].as_slice(),
    ] {
        let password_from_fd = predicates::str::contains("--password-from-fd");
        let password_from_file = predicates::str::contains("--password-from-file");
        let verbose = predicates::str::contains("--verbose");
        let quiet = predicates::str::contains("--quiet");

        nuts_tool(&tmp_dir, args)
            .assert()
            .success()
            .stdout(
                password_from_fd
                    .and(password_from_file)
                    .and(verbose)
                    .and(quiet),
            )
            .stderr("");
    }
}

#[test]
fn attach() {
    let tmp_dir = setup();

    setup_new_plugin(&tmp_dir);

    container_attach(&tmp_dir, "sample", "directory")
        .assert()
        .success()
        .stdout("")
        .stderr("");
    container_list(&tmp_dir)
        .assert()
        .success()
        .stdout(list::eq(["sample"]));

    container_attach(&tmp_dir, "sample", "new_plugin")
        .assert()
        .code(1)
        .stdout("you already have a container with the name sample\n")
        .stderr("");
    container_attach(&tmp_dir, "sample", "new_plugin")
        .arg("--force")
        .assert()
        .success()
        .stdout("")
        .stderr("");
    container_list(&tmp_dir)
        .assert()
        .success()
        .stdout(list::eq(["sample"]));
}

#[test]
fn list() {
    let tmp_dir = setup();

    setup_new_plugin(&tmp_dir);

    container_list(&tmp_dir)
        .assert()
        .success()
        .stdout("")
        .stderr("");

    container_create(&tmp_dir, "sample1", "directory", Some(b"123"))
        .assert()
        .success();
    container_list(&tmp_dir)
        .assert()
        .success()
        .stdout(list::eq(["sample1"]))
        .stderr("");

    container_create(&tmp_dir, "sample2", "new_plugin", Some(b"123"))
        .assert()
        .success();
    container_list(&tmp_dir)
        .assert()
        .success()
        .stdout(list::eq(["sample1", "sample2"]))
        .stderr("");

    plugin_remove(&tmp_dir, "new_plugin").assert().success();
    container_list(&tmp_dir)
        .assert()
        .success()
        .stdout(list::eq(["sample1"]))
        .stderr("");
    container_list(&tmp_dir)
        .arg("--all")
        .assert()
        .success()
        .stdout(list::eq(["  sample1", "! sample2"]))
        .stderr("");
}

#[test]
fn acquire() {
    let tmp_dir = setup();

    container_acquire(&tmp_dir, "sample", Some(b"123"))
        .assert()
        .code(1)
        .stdout("no such container: sample\n")
        .stderr("");

    container_create(&tmp_dir, "sample", "directory", Some(b"123"))
        .assert()
        .success();
    container_acquire(&tmp_dir, "sample", Some(b"xxx"))
        .assert()
        .code(1)
        .stdout("the plaintext is not trustworthy\n")
        .stderr("");

    let assert = container_acquire(&tmp_dir, "sample", Some(b"123"))
        .assert()
        .success()
        .stdout(predicates::str::starts_with("aquired: "))
        .stderr("");
    let id = id_from_acquire_stdout(assert);

    container_read(&tmp_dir, "sample", &id, Some(b"123"))
        .assert()
        .success()
        .stdout([b'\0'; 496].as_slice());
}

#[test]
fn change_password() {
    let tmp_dir = setup();
    let password_file = tmp_dir.join("new_password.txt");

    let mut f = File::create(&password_file).unwrap();
    f.write_all(b"new_password").unwrap();
    f.flush().unwrap();

    container_change_password(&tmp_dir, "sample", Some(b"123"))
        .assert()
        .code(1)
        .stdout("no such container: sample\n")
        .stderr("");

    container_create(&tmp_dir, "sample", "directory", Some(b"123"))
        .assert()
        .success();
    container_change_password(&tmp_dir, "sample", Some(b"xxx"))
        .assert()
        .code(1)
        .stdout("the plaintext is not trustworthy\n")
        .stderr("");
    let cmd = container_change_password(&tmp_dir, "sample", Some(b"123"));
    handle_password_file(
        &tmp_dir,
        cmd,
        "--new-password-from-file",
        Some(b"new_password"),
    )
    .assert()
    .success()
    .stdout("")
    .stderr("");

    container_info(&tmp_dir, "sample", Some(b"new_password"))
        .assert()
        .success();
}

#[test]
fn change_kdf() {
    let tmp_dir = setup();

    container_change_kdf(&tmp_dir, "sample", "pbkdf2", Some(b"123"))
        .assert()
        .code(1)
        .stdout("no such container: sample\n")
        .stderr("");

    container_create(&tmp_dir, "sample", "directory", Some(b"123"))
        .assert()
        .success();
    container_change_kdf(&tmp_dir, "sample", "pbkdf2", Some(b"xxx"))
        .assert()
        .code(1)
        .stdout("the plaintext is not trustworthy\n")
        .stderr("");

    for (arg, kdf) in [
        ("pbkdf2", "pbkdf2:sha256:65536:16"),
        ("pbkdf2:::", "pbkdf2:sha256:65536:16"),
        ("pbkdf2:sha1::", "pbkdf2:sha1:65536:16"),
        ("pbkdf2::666:", "pbkdf2:sha256:666:16"),
        ("pbkdf2:::6", "pbkdf2:sha256:65536:6"),
    ] {
        container_change_kdf(&tmp_dir, "sample", arg, Some(b"123"))
            .assert()
            .success()
            .stdout("")
            .stderr("");
        container_info(&tmp_dir, "sample", Some(b"123"))
            .assert()
            .success()
            .stdout(hash::eq(default_info_with([("kdf", kdf)].into())));
    }
}

#[test]
fn create() {
    let tmp_dir = setup();
    let mut idx = 0;

    for (args, pass, infos) in [
        ([].as_slice(), Some(b"123".as_slice()), [].into()),
        (
            &["--cipher", "none"],
            None,
            [
                ("cipher", "none"),
                ("kdf", "none"),
                ("block size (net)", "512"),
            ]
            .into(),
        ),
        (
            &["--cipher", "aes128-ctr"],
            Some(b"123"),
            [("cipher", "aes128-ctr"), ("block size (net)", "512")].into(),
        ),
        (
            &["--cipher", "aes192-ctr"],
            Some(b"123"),
            [("cipher", "aes192-ctr"), ("block size (net)", "512")].into(),
        ),
        (
            &["--cipher", "aes256-ctr"],
            Some(b"123"),
            [("cipher", "aes256-ctr"), ("block size (net)", "512")].into(),
        ),
        (
            &["--cipher", "aes128-gcm"],
            Some(b"123"),
            [("cipher", "aes128-gcm")].into(),
        ),
        (
            &["--cipher", "aes192-gcm"],
            Some(b"123"),
            [("cipher", "aes192-gcm")].into(),
        ),
        (&["--cipher", "aes256-gcm"], Some(b"123"), [].into()),
        (&["--kdf", "pbkdf2"], Some(b"123"), [].into()),
        (&["--kdf", "pbkdf2:::"], Some(b"123"), [].into()),
        (
            &["--kdf", "pbkdf2:sha1::"],
            Some(b"123"),
            [("kdf", "pbkdf2:sha1:65536:16")].into(),
        ),
        (
            &["--kdf", "pbkdf2::666:"],
            Some(b"123"),
            [("kdf", "pbkdf2:sha256:666:16")].into(),
        ),
        (
            &["--kdf", "pbkdf2:::6"],
            Some(b"123"),
            [("kdf", "pbkdf2:sha256:65536:6")].into(),
        ),
        (
            &["--", "--block-size", "1024"],
            Some(b"123"),
            [
                ("block size (gross)", "1024"),
                ("block size (net)", "1008"),
                ("block_size", "1024"),
            ]
            .into(),
        ),
    ] {
        let name = format!("sample{idx}");
        idx += 1;

        container_create(&tmp_dir, &name, "directory", pass)
            .args(args)
            .assert()
            .success()
            .stdout("")
            .stderr("");
        container_info(&tmp_dir, &name, pass)
            .assert()
            .success()
            .stdout(hash::eq(default_info_with(infos)));
    }

    container_create(&tmp_dir, "sample-overwrite", "directory", Some(b"123"))
        .assert()
        .success();

    container_create(&tmp_dir, "sample-overwrite", "directory", Some(b"123"))
        .assert()
        .code(1)
        .stdout("you already have a container with the name sample-overwrite\n")
        .stderr("");
    container_create(&tmp_dir, "sample-overwrite", "directory", Some(b"blabla"))
        .arg("--overwrite")
        .assert()
        .success()
        .stdout("")
        .stderr("");

    container_info(&tmp_dir, "sample-overwrite", Some(b"blabla"))
        .assert()
        .success();
}

#[test]
fn delete() {
    let tmp_dir = setup();

    container_delete(&tmp_dir, "sample", Some(b"123"))
        .arg("--yes")
        .assert()
        .code(1)
        .stdout("container sample not configured\nno such container: sample\n")
        .stderr("");

    container_create(&tmp_dir, "sample", "directory", Some(b"123"))
        .assert()
        .success();
    assert!(tmp_dir.join(".nuts/container.d/sample").exists());
    container_delete(&tmp_dir, "sample", Some(b"xxx"))
        .arg("--yes")
        .assert()
        .code(1)
        .stdout("the plaintext is not trustworthy\n")
        .stderr("");
    assert!(tmp_dir.join(".nuts/container.d/sample").exists());
    container_delete(&tmp_dir, "sample", Some(b"123"))
        .arg("--yes")
        .assert()
        .success()
        .stdout("")
        .stderr("");
    assert!(!tmp_dir.join(".nuts/container.d/sample").exists());

    container_create(&tmp_dir, "sample", "directory", Some(b"123"))
        .assert()
        .success();
    assert!(tmp_dir.join(".nuts/container.d/sample").exists());
    container_delete(&tmp_dir, "sample", Some(b"xxx"))
        .arg("--yes")
        .assert()
        .code(1)
        .stdout("the plaintext is not trustworthy\n")
        .stderr("");
    assert!(tmp_dir.join(".nuts/container.d/sample").exists());
    container_delete(&tmp_dir, "sample", None)
        .args(["--force", "--yes"])
        .assert()
        .success()
        .stdout("")
        .stderr("");
    assert!(!tmp_dir.join(".nuts/container.d/sample").exists());
}

#[test]
fn info() {
    let tmp_dir = setup();

    container_info(&tmp_dir, "sample", Some(b"123"))
        .assert()
        .code(1)
        .stdout("no such container: sample\n")
        .stderr("");

    container_create(&tmp_dir, "sample", "directory", Some(b"123"))
        .assert()
        .success();
    container_info(&tmp_dir, "sample", Some(b"xxx"))
        .assert()
        .code(1)
        .stdout("the plaintext is not trustworthy\n")
        .stderr("");
    container_info(&tmp_dir, "sample", Some(b"123"))
        .assert()
        .success()
        .stdout(hash::eq(default_info_with([].into())))
        .stderr("");
}

#[test]
fn read() {
    let tmp_dir = setup();

    container_read(&tmp_dir, "sample", "xxx", Some(b"123"))
        .assert()
        .code(1)
        .stdout("no such container: sample\n")
        .stderr("");

    container_create(&tmp_dir, "sample", "directory", Some(b"123"))
        .assert()
        .success();
    let assert = container_acquire(&tmp_dir, "sample", Some(b"123"))
        .assert()
        .success();
    let id = id_from_acquire_stdout(assert);
    let reverted_id = hex::encode(
        hex::decode(&id)
            .unwrap()
            .into_iter()
            .rev()
            .collect::<Vec<_>>(),
    );
    let data = [0, 1, 2, 3, 4, 5, 6, 7].repeat(62);

    container_write(&tmp_dir, "sample", Some(&id), &data, Some(b"123"))
        .assert()
        .success();

    container_read(&tmp_dir, "sample", "xxx", Some(b"123"))
        .assert()
        .code(1)
        .stdout("could not parse id\n")
        .stderr("");
    container_read(&tmp_dir, "sample", &reverted_id, Some(b"123"))
        .assert()
        .code(1)
        .stdout("the backend created an error: No such file or directory (os error 2)\n")
        .stderr("");
    container_read(&tmp_dir, "sample", &id, Some(b"xxx"))
        .assert()
        .code(1)
        .stdout("the plaintext is not trustworthy\n")
        .stderr("");
    container_read(&tmp_dir, "sample", &id, Some(b"123"))
        .assert()
        .success()
        .stdout(data.clone())
        .stderr("");

    for (max, n) in [("0", 0), ("1", 1), ("248", 248), ("496", 496), ("497", 496)] {
        container_read(&tmp_dir, "sample", &id, Some(b"123"))
            .args(["--max-bytes", max])
            .assert()
            .success()
            .stdout(data[..n].to_vec())
            .stderr("");
    }
}

#[test]
fn release() {
    let tmp_dir = setup();

    container_release(&tmp_dir, "sample", "xxx", Some(b"123"))
        .assert()
        .code(1)
        .stdout("no such container: sample\n")
        .stderr("");

    container_create(&tmp_dir, "sample", "directory", Some(b"123"))
        .assert()
        .success();
    let assert = container_acquire(&tmp_dir, "sample", Some(b"123"))
        .assert()
        .success();
    let id = id_from_acquire_stdout(assert);
    let reverted_id = hex::encode(
        hex::decode(&id)
            .unwrap()
            .into_iter()
            .rev()
            .collect::<Vec<_>>(),
    );

    container_release(&tmp_dir, "sample", "xxx", Some(b"123"))
        .assert()
        .code(1)
        .stdout("could not parse id\n")
        .stderr("");
    container_release(&tmp_dir, "sample", &reverted_id, Some(b"123"))
        .assert()
        .code(1)
        .stdout("the backend created an error: No such file or directory (os error 2)\n")
        .stderr("");

    container_release(&tmp_dir, "sample", &id, Some(b"123"))
        .assert()
        .success()
        .stdout("")
        .stderr("");
    container_read(&tmp_dir, "sample", &id, Some(b"123"))
        .assert()
        .code(1)
        .stdout("the backend created an error: No such file or directory (os error 2)\n")
        .stderr("");
}

#[test]
fn write() {
    let tmp_dir = setup();
    let data = [0, 1, 2, 3, 4, 5, 6, 7].repeat(63);

    container_write(&tmp_dir, "sample", Some("xxx"), &data, Some(b"123"))
        .assert()
        .code(1)
        .stdout("no such container: sample\n")
        .stderr("");

    container_create(&tmp_dir, "sample", "directory", Some(b"123"))
        .assert()
        .success();
    let assert = container_acquire(&tmp_dir, "sample", Some(b"123"))
        .assert()
        .success();
    let id = id_from_acquire_stdout(assert);
    let reverted_id = hex::encode(
        hex::decode(&id)
            .unwrap()
            .into_iter()
            .rev()
            .collect::<Vec<_>>(),
    );

    container_write(&tmp_dir, "sample", Some("xxx"), &data, Some(b"123"))
        .assert()
        .code(1)
        .stdout("could not parse id\n")
        .stderr("");
    container_write(&tmp_dir, "sample", Some(&reverted_id), &data, Some(b"123"))
        .assert()
        .code(1)
        .stdout(predicates::str::starts_with(format!(
            "the backend created an error: cannot open {reverted_id}, no related file "
        )))
        .stderr("");
    container_write(&tmp_dir, "sample", Some(&id), &data, Some(b"xxx"))
        .assert()
        .code(1)
        .stdout("the plaintext is not trustworthy\n")
        .stderr("");

    for (args, max, num) in [
        ([].as_slice(), 496, 496),
        (&["--max-bytes", "1"], 1, 1),
        (&["--max-bytes", "248"], 248, 248),
        (&["--max-bytes", "496"], 496, 496),
        (&["--max-bytes", "497"], 497, 496),
    ] {
        let assert = container_acquire(&tmp_dir, "sample", Some(b"123"))
            .assert()
            .success();
        let id = id_from_acquire_stdout(assert);
        let mut out = vec![0; 496];

        out[..num].copy_from_slice(&data[..num]);

        container_write(&tmp_dir, "sample", Some(&id), &data[..max], Some(b"123"))
            .args(args)
            .assert()
            .success()
            .stdout(format!("{num} bytes written into {id}\n"))
            .stderr("");
        container_read(&tmp_dir, "sample", &id, Some(b"123"))
            .assert()
            .success()
            .stdout(out)
            .stderr("");
    }

    let assert = container_write(&tmp_dir, "sample", None, &data, Some(b"123"))
        .assert()
        .success()
        .stdout(predicates::str::starts_with("496 bytes written into "))
        .stderr("");
    let output = assert.get_output();
    let id = str::from_utf8(output.stdout.split(|b| *b == b' ').nth(4).unwrap())
        .unwrap()
        .trim_end();
    container_read(&tmp_dir, "sample", &id, Some(b"123"))
        .assert()
        .success()
        .stdout(data[..496].to_vec())
        .stderr("");
}
