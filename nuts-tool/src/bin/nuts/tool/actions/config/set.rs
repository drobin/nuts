// MIT License
//
// Copyright (c) 2023 Robin Doer
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

use anyhow::{anyhow, Result};
use clap::{App, Arg, ArgMatches};

use crate::tool::config::Config;

pub fn command<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    app.about("Updates the value of an option.")
        .long_about(
            "Updates the value of an option.\n\n \
             Without any further flags set, it replaces the current value \
             with VALUE. If NAME is an array then the whole array is replaced \
             with a new array which only contains VALUE.",
        )
        .arg(
            Arg::with_name("NAME")
                .required(true)
                .index(1)
                .help("The name of the config option."),
        )
        .arg(
            Arg::with_name("VALUE")
                .index(2)
                .help("The new value of the config option."),
        )
        .arg(Arg::with_name("add").long("add").help(
            "Appends VALUE to an array. The flag is ignored, \
                       if NAME is not an array.",
        ))
        .arg(Arg::with_name("delete").long("delete").help(
            "Removes VALUE from an array. If NAME is not an array, \
                       the entire option is removed and the default value is \
                       used.",
        ))
        .arg(
            Arg::with_name("clear")
                .long("clear")
                .help("Removes the entire option."),
        )
}

pub fn run(args: &ArgMatches) -> Result<()> {
    let name = args.value_of("NAME").unwrap();
    let value = args.value_of("VALUE");

    let mut config = Config::parse()?;

    if args.is_present("add") {
        config_add(&mut config, name, value)?;
    } else if args.is_present("delete") {
        config_del(&mut config, name, value)?;
    } else if args.is_present("clear") {
        config_clear(&mut config, name)?;
    } else {
        config_set(&mut config, name, value)?;
    }

    config.save()
}

fn mandatory_value<'a>(name: &str, value: Option<&'a str>) -> Result<&'a str> {
    value.ok_or(anyhow!("Missing value for option {}", name))
}

fn find_pos(vec: &[String], value: &str) -> Option<usize> {
    vec.iter().position(|s| s == value)
}

fn config_add(config: &mut Config, name: &str, value: Option<&str>) -> Result<()> {
    match name {
        "search_path" => {
            let value = mandatory_value(name, value)?;

            match find_pos(&config.search_path, value) {
                Some(pos) => {
                    let value = config.search_path.remove(pos);
                    config.search_path.insert(0, value);
                }
                None => {
                    config.search_path.insert(0, value.to_string());
                }
            }

            Ok(())
        }
        _ => Err(anyhow!("No such option: {}", name)),
    }
}

fn config_del(config: &mut Config, name: &str, value: Option<&str>) -> Result<()> {
    match name {
        "search_path" => {
            let value = mandatory_value(name, value)?;

            if let Some(pos) = find_pos(&config.search_path, value) {
                config.search_path.remove(pos);
            }

            Ok(())
        }
        _ => Err(anyhow!("No such option: {}", name)),
    }
}

fn config_clear(config: &mut Config, name: &str) -> Result<()> {
    match name {
        "search_path" => {
            config.search_path = Vec::default();
            Ok(())
        }
        _ => Err(anyhow!("No such option: {}", name)),
    }
}

fn config_set(config: &mut Config, name: &str, value: Option<&str>) -> Result<()> {
    match name {
        "search_path" => {
            let value = mandatory_value(name, value)?;
            config.search_path = vec![value.to_string()];

            Ok(())
        }
        _ => Err(anyhow!("No such option: {}", name)),
    }
}
