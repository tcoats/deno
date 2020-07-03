// Copyright 2018-2020 the Deno authors. All rights reserved. MIT license.
// #![deny(warnings)]

use deno::colors;
use deno::tokio_util;
use deno::GlobalState;
use deno::MainWorker;
use deno::DenoSubcommand;
use deno::Flags;
use deno_core::ErrBox;
use deno_core::ModuleSpecifier;
use futures::future::FutureExt;
use log::Level;
use log::Metadata;
use log::Record;
use std::env;

static LOGGER: Logger = Logger;

// TODO(ry) Switch to env_logger or other standard crate.
struct Logger;

impl log::Log for Logger {
  fn enabled(&self, metadata: &Metadata) -> bool {
    metadata.level() <= log::max_level()
  }

  fn log(&self, record: &Record) {
    if self.enabled(record.metadata()) {
      let mut target = record.target().to_string();

      if let Some(line_no) = record.line() {
        target.push_str(":");
        target.push_str(&line_no.to_string());
      }

      if record.level() >= Level::Info {
        eprintln!("{}", record.args());
      } else {
        eprintln!("{} RS - {} - {}", record.level(), target, record.args());
      }
    }
  }
  fn flush(&self) {}
}

async fn exec(flags: Flags, script: String) -> Result<(), ErrBox> {
  let global_state = GlobalState::new(flags.clone())?;
  let main_module = ModuleSpecifier::resolve_url_or_path(&script).unwrap();
  let mut worker =
    MainWorker::create(global_state.clone(), main_module.clone())?;

  worker.execute_module(&main_module).await?;
  worker.execute("window.dispatchEvent(new Event('load'))")?;
  (&mut *worker).await?;
  worker.execute("window.dispatchEvent(new Event('unload'))")?;
  Ok(())
}

fn main() {
  #[cfg(windows)]
  colors::enable_ansi(); // For Windows 10

  log::set_logger(&LOGGER).unwrap();
  let args: Vec<String> = env::args().collect();
  if args.len() != 2 {
    println!("tumu {}
A secure JavaScript and TypeScript runtime

USAGE:
    tumu [SCRIPT]", env!("CARGO_PKG_VERSION"));
    std::process::exit(1);
  }

  let flags = Flags {
    subcommand: DenoSubcommand::Run { script: String::from(&args[1]) },
    allow_env: false,
    allow_hrtime: false,
    allow_net: false,
    allow_plugin: false,
    allow_read: false,
    allow_run: false,
    allow_write: false,
    ..Flags::default()
  };

  log::set_max_level(Level::Info.to_level_filter());

  let result = tokio_util::run_basic(exec(flags, String::from(&args[1])).boxed_local());

  if let Err(err) = result {
    let msg = format!("{}: {}", colors::red_bold("error"), err.to_string(),);
    eprintln!("{}", msg);
    std::process::exit(1);
  }
}
