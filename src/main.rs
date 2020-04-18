use clap::{App, Arg};
use regex::{Regex, RegexBuilder};
use std::{thread, time};
use std::process::exit;
use sysinfo::{ProcessExt, System, SystemExt, Signal, Process};

fn main() {
    let matches = App::new("process-killer")
        .version("0.3.0")
        .about("A simple utility for for terminating processes quickly and cleanly.")
        .arg(Arg::with_name("pattern")
                 .help("All processes that contain this pattern will be killed. Case insensitive by default.")
                 .index(1)
                 .required(true))
        .arg(Arg::with_name("wait-ms")
                .help("How many milliseconds to wait for the processes to gracefully terminate before force killing them.")
                .takes_value(true)
                .long("wait-ms")
                .short("w")
                .default_value("3000")
            )
        .arg(Arg::with_name("regex")
                .help("Interpret the pattern as a regular expression")
                .long("regex")
                .short("r"))
        .arg(Arg::with_name("case-sensitive")
                .help("Make pattern to be case sensitive")
                .long("case-sensitive")
                .short("c"))
                .get_matches();
    let pattern = matches
        .value_of("pattern")
        .expect("Error while getting pattern.");
    if pattern == "" {
        eprintln!("Pattern should not be empty.");
        exit(-1);
    }

    let regular_expression = matches.is_present("regex");
    let case_sensitive = matches.is_present("case-sensitive");

    let wait_ms: u64 = matches
        .value_of("wait-ms")
        .expect("Error while getting the value of wait-ms.")
        .parse()
        .expect("Wait_ms was not a number");

    let s = System::new_all();

    let processes_iterator = s.get_processes().iter().map(|(_pid, process)| process);

    let matching: Vec<&Process> = if regular_expression {
        let regex: Regex = RegexBuilder::new(pattern)
            .case_insensitive(!case_sensitive)
            .build()
            .expect("Regular expression was not valid");
        processes_iterator
            .filter(|process| regex.is_match(process.name()))
            .collect()
    } else {
        processes_iterator
            .filter(|process| {
                let mut name: String = process.name().to_string();
                let mut filter_pattern = pattern.to_string();
                if !case_sensitive {
                    name = name.to_lowercase();
                    filter_pattern = filter_pattern.to_lowercase();
                }
                return name.contains(&filter_pattern);
            })
            .collect()
    };

    matching
        .iter()
        .for_each(|process| {
                      println!("pid: {}, process: {:?}, status: {}",
                               process.pid(),
                               process.name(),
                               process.status())
                  });
    if matching.len() == 0 {
        eprintln!("No processes found");
        exit(-1);
    }
    println!("\nKilling processes with sigterm");
    matching
        .iter()
        .for_each(|process| { process.kill(Signal::Term); });
    thread::sleep(time::Duration::from_millis(100));

    // We're doing waiting in 100ms steps so that we can exit as soon as the
    // processes terminate.
    let max_wait_steps = wait_ms / 100;
    for _ in 1..(max_wait_steps + 1) {
        if get_alive_processes(&matching, &System::new()).len() == 0 {
            exit(0);
        }

        thread::sleep(time::Duration::from_millis(wait_ms / max_wait_steps));
    }


    let s = System::new();
    let remaining = get_alive_processes(&matching, &s);
    if remaining.len() == 0 {
        exit(0);
    }

    println!("The following processes did not terminate:");
    remaining
        .iter()
        .for_each(|process| println!("pid: {}, status: {}", process.pid(), process.status()));
    println!("\nKilling processes with sigkill");
    matching
        .iter()
        .for_each(|process| { process.kill(Signal::Kill); });
    thread::sleep(time::Duration::from_millis(1));
}

fn get_alive_processes<'a>(processes: &Vec<&'a Process>, system: &'a System) -> Vec<&'a Process> {
    processes
        .into_iter()
        .filter_map(move |process| system.get_process(process.pid()))
        .collect()
}
