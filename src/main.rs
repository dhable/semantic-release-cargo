// Copyright 2020 Steven Bosnick
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE-2.0 or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms

use anyhow::Error;
use human_panic::setup_panic;
use log::Level;
use loggerv::{Logger, Output};
use structopt::StructOpt;

/// Run sementic-release steps in the context of a cargo based Rust project.
#[derive(StructOpt)]
struct Opt {
    /// Increases the logging level (use multiple times for more detail).
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u64,
}

fn main() -> Result<(), Error> {
    setup_panic!();

    let opt = Opt::from_args();
    Logger::new()
        .output(&Level::Trace, Output::Stderr)
        .output(&Level::Debug, Output::Stderr)
        .output(&Level::Info, Output::Stderr)
        .verbosity(opt.verbose)
        .init()?;

    println!("Hello, world!");

    Ok(())
}
