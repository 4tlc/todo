// SPDX-License-Identifier: Unlicense

mod command;
mod formats;
mod printer;
mod searcher;
use printer::{print_single_file, start_print_directory};
mod args;
use args::{parse_args, Config};
mod errors;
use errors::Errors;
use lazy_static::lazy_static;

lazy_static! {
    static ref CONFIG: Config = parse_args().unwrap_or_else(|e| {
        exit_error(e);
    });
}

fn main() {
    if CONFIG.is_dir {
        let top_dir = searcher::begin_search_on_directory(&CONFIG.path).map_err(|e| exit_error(e));
        let mut out = std::io::stdout().lock();
        start_print_directory(&mut out, top_dir.unwrap()).unwrap_or_else(|e| exit_error(e));
    } else {
        let mut out = std::io::stdout().lock();
        let file = searcher::search_file(&CONFIG.path).unwrap_or_else(|e| exit_error(e));
        print_single_file(&mut out, &file).unwrap_or_else(|e| exit_error(e));
    }
}

fn exit_error(e: Errors) -> ! {
    println!("{}", e);
    std::process::exit(1);
}
