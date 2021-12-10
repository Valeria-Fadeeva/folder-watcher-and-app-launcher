extern crate notify;

use notify::{raw_watcher, RawEvent, RecursiveMode, Watcher};
use std::sync::mpsc::channel;

use std::{process, thread, time};

use sysinfo::{ProcessExt, Signal, System, SystemExt};

fn main() {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Create a watcher object, delivering raw events.
    // The notification back-end is selected based on the platform.
    let mut watcher = raw_watcher(tx).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.

    let mut child_id: u32 = 0;

    let mut _app: &str = "";
    let mut _watch_path: &str = "";

    #[cfg(target_os = "windows")]
    let _watch_path = "d:/pdf";
    #[cfg(target_os = "windows")]
    let _app = "C:/Program Files (x86)/Google/Chrome/Application/chrome.exe";

    #[cfg(target_os = "linux")]
    let _watch_path = "/tmp/pdf";
    #[cfg(target_os = "linux")]
    let _app = "google-chrome-stable";

    println!("_watch_path {}", _watch_path);
    println!("_app (file viewer) {}", _app);
    watcher
        .watch(_watch_path, RecursiveMode::Recursive)
        .unwrap();

    let par = "--kiosk";

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
