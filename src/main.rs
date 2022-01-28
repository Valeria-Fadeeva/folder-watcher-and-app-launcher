extern crate dotenv;
extern crate notify;

use dotenv::dotenv;
use notify::{raw_watcher, RawEvent, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::{env, process, thread, time};

use sysinfo::{ProcessExt, Signal, System, SystemExt};

fn main() {
    dotenv().ok();
    let watcher_execute = env::var("WATCHER_EXECUTE").unwrap();
    let _app: &str = watcher_execute.as_str();

    let par: &str = "-s";

    let mut child_id: u32 = 0;

    let _watch_path: &str = "";

    let _watch_path: &str = if cfg!(target_os = "windows") {
        "d:/pdf"
    } else if cfg!(target_os = "linux") {
        "/tmp/pdf"
    } else {
        "/tmp/pdf"
    };

    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Create a watcher object, delivering raw events.
    // The notification back-end is selected based on the platform.
    let mut watcher = raw_watcher(tx).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.

    watcher
        .watch(_watch_path, RecursiveMode::Recursive)
        .unwrap();

    println!("_watch_path {}", _watch_path);
    println!("_app (file viewer) {}", _app);

    loop {
        match rx.recv() {
            Ok(RawEvent {
                path: Some(path),
                op: Ok(op),
                cookie,
            }) => {
                println!("{:?} {:?} ({:?})", op, path, cookie);

                if op == notify::op::WRITE {
                    if path.to_string_lossy().ends_with(".pdf") {
                        if child_id > 0 {
                            println!("Child's ID is {}", child_id);

                            #[cfg(target_os = "windows")]
                            let c: usize = child_id as usize;
                            #[cfg(target_os = "linux")]
                            let c: i32 = child_id as i32;

                            let s = System::new_all();
                            /*for process in s.process_by_name("chrome") {
                                println!("{} {}", process.pid(), process.name());
                                if process.pid() == c {
                                    process.kill(Signal::Kill);
                                    println!("Process {} killed", child_id);
                                }
                            }*/

                            if let Some(process) = s.process(c) {
                                process.kill(Signal::Kill);
                                println!("Process {} killed", child_id);
                            }
                        }

                        thread::sleep(time::Duration::from_millis(500));

                        let child = process::Command::new(_app)
                            .args([
                                par,
                                path.into_os_string()
                                    .into_string()
                                    .unwrap()
                                    .to_string()
                                    .as_str(),
                            ])
                            .stderr(process::Stdio::null()) // don't care about stderr
                            .stdout(process::Stdio::inherit()) // set up stdout so we can read it
                            .stdin(process::Stdio::inherit()) // set up stdin so we can write on it
                            .spawn()
                            .expect("failed to execute child");

                        child_id = child.id();
                        println!("Child's ID is {}", child_id);
                    }
                }
            }
            Ok(event) => println!("broken event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
