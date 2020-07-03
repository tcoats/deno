// Copyright 2020 the Tumu authors. All rights reserved. MIT license.
#![deny(warnings)]

extern crate dissimilar;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate futures;
#[macro_use]
extern crate serde_json;
extern crate clap;
extern crate deno_core;
extern crate indexmap;
#[cfg(unix)]
extern crate nix;
extern crate rand;
extern crate regex;
extern crate reqwest;
extern crate serde;
extern crate serde_derive;
extern crate tokio;
extern crate url;

pub mod checksum;
pub mod colors;
pub mod deno_dir;
pub mod diagnostics;
pub mod diff;
pub mod disk_cache;
pub mod doc;
pub mod file_fetcher;
pub mod flags;
pub mod flags_allow_net;
pub mod fmt;
pub mod fmt_errors;
pub mod fs;
pub mod global_state;
pub mod global_timer;
pub mod http_cache;
pub mod http_util;
pub mod import_map;
pub mod inspector;
pub mod installer;
pub mod js;
pub mod lint;
pub mod lockfile;
pub mod metrics;
pub mod module_graph;
pub mod msg;
pub mod op_error;
pub mod ops;
pub mod permissions;
pub mod repl;
pub mod resolve_addr;
pub mod signal;
pub mod source_maps;
pub mod startup_data;
pub mod state;
pub mod swc_util;
pub mod test_runner;
pub mod tokio_util;
pub mod tsc;
pub mod upgrade;
pub mod version;
pub mod web_worker;
pub mod worker;

pub use dprint_plugin_typescript::swc_common;
pub use dprint_plugin_typescript::swc_ecma_ast;
pub use dprint_plugin_typescript::swc_ecma_parser;

pub use crate::doc::parser::DocFileLoader;
pub use crate::file_fetcher::SourceFile;
pub use crate::file_fetcher::SourceFileFetcher;
pub use crate::fs as deno_fs;
pub use crate::global_state::GlobalState;
pub use crate::msg::MediaType;
pub use crate::op_error::OpError;
pub use crate::permissions::Permissions;
pub use crate::tsc::TargetLib;
pub use crate::worker::Worker;
pub use crate::worker::MainWorker;
use deno_core::ErrBox;
pub use flags::DenoSubcommand;
pub use flags::Flags;
pub use upgrade::upgrade_command;
