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

#[cfg(test)]
#[cfg(feature = "tool")]
mod tests;

use serde::{Deserialize, Deserializer, Serialize};

/// Current revision of the exchange protocol between plugin and tool.
///
/// # History
///
/// ## Revision 0
///
/// Initial revision.
///
/// ## Revision 1
///
/// The `revision` field was added to [`PluginInfo`]. The
/// [`crate::OkResponse::Map`] response of a [`crate::Request::PluginInfo`]
/// request now contains a `revision` key. Therfore, without the `revision` key
/// in the response you will have a revision `0`.
pub const CURRENT_REVISION: u32 = 1;

fn de_revision<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u32, D::Error> {
    let rev: u32 = Deserialize::deserialize(deserializer)?;

    // Revision 0 has no revision field in PluginInfo. Therefore there cannot
    // be a such a revision.

    if rev > 0 {
        Ok(rev)
    } else {
        Err(serde::de::Error::custom(
            "revision 0 cannot be explicity specified",
        ))
    }
}

/// Information of a plugin
#[derive(Debug, Deserialize, Serialize)]
pub struct PluginInfo {
    name: String,
    version: String,
    #[serde(default)]
    #[serde(deserialize_with = "de_revision")]
    revision: u32,
}

impl PluginInfo {
    /// Creates a new `PluginInfo` instance.
    pub fn new<N: AsRef<str>, V: AsRef<str>>(name: N, version: V) -> PluginInfo {
        PluginInfo {
            name: name.as_ref().to_string(),
            version: version.as_ref().to_string(),
            revision: CURRENT_REVISION,
        }
    }

    /// Returns the name of the plugin.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the version of the plugin.
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Returns the protocol revision.
    pub fn revision(&self) -> u32 {
        self.revision
    }
}

#[cfg(feature = "tool")]
mod tool_impls {
    use std::collections::HashMap;
    use std::convert::TryFrom;

    use crate::info::PluginInfo;
    use crate::tool::PluginError::{self, InvalidResponse};

    impl TryFrom<HashMap<String, String>> for PluginInfo {
        type Error = PluginError;

        fn try_from(mut map: HashMap<String, String>) -> Result<Self, PluginError> {
            let name = map.remove("name").ok_or(InvalidResponse)?;
            let version = map.remove("version").ok_or(InvalidResponse)?;

            // In revision 0 the revision field is missing.
            let revision = map
                .remove("revision")
                .map_or(Ok(Default::default()), |s| s.parse())
                .map_err(|_| InvalidResponse)?;

            Ok(PluginInfo {
                name,
                version,
                revision,
            })
        }
    }
}
