use std::{ thread, time, io::{ stdin } };
use sysinfo::*;

mod processes;
use processes::*;

mod types;
use types::View;

fn main() {
    let mut sys = System::new_all();
    let mut view = View::DiskUsage;

    print!("\x1B[2J\x1B[1;1H"); //Clearing the console

    let (tx, rx) = std::sync::mpsc::channel();

    thread::spawn(move || {
        loop {
            let mut line = String::new();
            stdin().read_line(&mut line).unwrap();
            tx.send(line).unwrap();

            thread::sleep(time::Duration::from_secs(1));
        }
    });

    loop {
        print!("\x1B[2J\x1B[1;1H"); //Clearing the console
        sys.refresh_all();
        if let Ok(input_string) = rx.try_recv() {
            match input_string.trim() {
                "ct" => {
                    view = View::ComponentTemperature;
                }
                "du" => {
                    view = View::DiskUsage;
                }
                "p" => {
                    view = View::Processes;
                }
                "k" => {
                    kill_process(&sys, 1223);
                }
                _ => eprintln!("This command is not valid"),
            }
        }

        match &view {
            View::ComponentTemperature => get_component_temperature(&sys),
            View::DiskUsage => get_disk_usage(&sys),
            View::Processes => get_processes(&sys),
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}