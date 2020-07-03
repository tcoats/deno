// Copyright 2020 the Tumu authors. All rights reserved. MIT license.
// #![deny(warnings)]

use deno::colors;
use deno::tokio_util;
use deno::GlobalState;
use deno::Worker;
use deno::DenoSubcommand;
use deno::Flags;
use deno::ops;
use deno::ops::io::get_stdio;
use deno::state::State;
use deno::startup_data;
use deno_core::ErrBox;
use deno_core::ModuleSpecifier;
use deno_core::CoreIsolate;
use futures::future::FutureExt;
use log::Level;
use log::Metadata;
use log::Record;
use std::env;

static LOGGER: Logger = Logger;

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
  let state = State::new(
    global_state.clone(),
    None,
    main_module.clone(),
    global_state.maybe_import_map.clone(),
    false,
  )?;
  let state_ = state.clone();
  let mut worker = Worker::new(
    "main".to_string(),
    startup_data::deno_isolate_init(),
    state_
  );
  {
    let isolate = &mut worker.isolate;
    ops::runtime::init(isolate, &state);
    ops::runtime_compiler::init(isolate, &state);
    ops::errors::init(isolate, &state);
    ops::fetch::init(isolate, &state);
    ops::fs::init(isolate, &state);
    ops::fs_events::init(isolate, &state);
    ops::io::init(isolate, &state);
    ops::plugin::init(isolate, &state);
    ops::net::init(isolate, &state);
    ops::tls::init(isolate, &state);
    ops::os::init(isolate, &state);
    ops::permissions::init(isolate, &state);
    ops::process::init(isolate, &state);
    ops::random::init(isolate, &state);
    ops::repl::init(isolate, &state);
    ops::resources::init(isolate, &state);
    ops::signal::init(isolate, &state);
    ops::timers::init(isolate, &state);
    ops::tty::init(isolate, &state);
    ops::worker_host::init(isolate, &state);
  }
  {
    let (stdin, stdout, stderr) = get_stdio();
    let state_rc = CoreIsolate::state(&worker.isolate);
    let state = state_rc.borrow();
    let mut t = state.resource_table.borrow_mut();
    t.add("stdin", Box::new(stdin));
    t.add("stdout", Box::new(stdout));
    t.add("stderr", Box::new(stderr));
  }
  worker.execute("bootstrap.mainRuntime()")?;
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
