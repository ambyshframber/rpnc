use argparse::{ArgumentParser, Store, StoreTrue, StoreFalse};
use shell::Shell;
use std::fs::read_to_string;
use std::env::var;
use std::io::ErrorKind;
use std::path::PathBuf;

mod shell;
mod utils;

fn main() {
    let mut s = Shell::new();
    let mut continue_run = false;
    let mut run_anyway = false;
    let mut init_line = String::new();
    let mut file = String::from("-");
    let mut use_rc = true;

    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut init_line)
            .add_option(&["-e"],
            Store,
            "a line to run on program init"
        );
        ap.refer(&mut continue_run).add_option(
            &["-r"],
            StoreTrue,
            "continue the interpreter after init line or file",
        );
        ap.refer(&mut use_rc).add_option(
            &["-c"], 
            StoreFalse,
            "do not use the ~/.rpnrc file to init"
        );
        ap.refer(&mut file).add_argument(
            "file",
            Store,
            "the file to use. '-' refers to standard input",
        );
        ap.parse_args_or_exit();
    }

    if init_line.is_empty() || continue_run {
        run_anyway = true
    }
    let mut is_tty = false;
    if atty::is(atty::Stream::Stdin) {
        s.interactive = true;
        is_tty = true;
    }
    if let Ok(d) = var("HOME") {
        match read_to_string(PathBuf::from(&d).join(".rpncrc")) {
            Ok(f) => {
                for l in f.split('\n') {
                    s.do_line(l);
                }
            }
            Err(e) => {
                match e.kind() {
                    ErrorKind::NotFound => {},
                    _ => panic!()
                }
            }
        }
    }
    s.do_line(&init_line);
    if file != "-" {
        // NOT stdin
        s.interactive = false;
        let program = read_to_string(file).unwrap();
        for l in program.split('\n') {
            if s.do_line(l) {
                break;
            }
        }
        if continue_run {
            s.interactive = true;
            s.run().unwrap()
        }
    } else {
        // YES stdin
        if run_anyway || !is_tty {
            s.run().unwrap()
        }
    }
}
