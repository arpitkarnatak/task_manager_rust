use std::io::{self, Read};
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use sysinfo::*;

#[derive(Debug)]
struct CustomProcess<'a> {
    pid: u32,
    name: &'a str,
    memory_usage: u64,
    virtual_memory_usage: u64,
    cpu_memory_usage: f32,
}
fn get_disk_usage(sys: &System) {
    let mut table = tabular::Table::new("{:<}  {:<} {:<} {:<}");
    table.add_heading("Disk Usage\n");
    table.add_row(
        tabular::Row::new()
            .with_cell("Used")
            .with_cell("Total")
            .with_cell("Used Swap")
            .with_cell("Total Swap"),
    );
    table.add_row(
        tabular::Row::new()
            .with_cell(sys.used_memory())
            .with_cell(sys.total_memory())
            .with_cell(sys.total_swap())
            .with_cell(sys.used_swap()),
    );
    print!("{}\n\n", table);
}

fn get_component_temperature(sys: &System) {
    let mut table = tabular::Table::new("{:<}\t\t{:>}\t{:>}\t\t{:>}");
    table.add_heading("Hardware Temperature\n");
    table.add_row(
        tabular::Row::new()
            .with_cell("Component Name")
            .with_cell("Temperature (F)")
            .with_cell("Max Temperature (F)")
            .with_cell("Critical Temp (F)"),
    );

    for component in sys.components() {
        table.add_row(
            tabular::Row::new()
                .with_cell(component.label())
                .with_cell(component.temperature())
                .with_cell(component.max())
                .with_cell(component.critical().unwrap_or(0.0)),
        );
        //println!("{:?} {:?}", component, component.label());
    }
    print!("{}\n\n", table);
}

fn get_processes(sys: &System) {
    let mut vector = Vec::<CustomProcess>::new();

    
    for (pid, process) in sys.processes() {
        vector.push(CustomProcess{ pid: pid.as_u32(),
            name: process.name(),
            memory_usage: process.memory(),
            virtual_memory_usage: process.virtual_memory(),
            cpu_memory_usage: process.cpu_usage()})
    }
    vector.sort_by(|a,b| b.memory_usage.cmp(&a.memory_usage));

    let mut table = tabular::Table::new("{:<}  {:>} {:>} {:>} {:>}");
    table.add_heading("Processes\n");
    table.add_row(
        tabular::Row::new()
            .with_cell("PID")
            .with_cell("Name")
            .with_cell("Memory Usage")
            .with_cell("Virtual Memory Usage")
            .with_cell("CPU Memory Usage")
    );

    for process in &vector[0..10] {
        table.add_row(tabular::Row::new()
            .with_cell(process.pid)
            .with_cell(process.name)
            .with_cell(process.memory_usage)
            .with_cell(process.virtual_memory_usage)
            .with_cell(process.cpu_memory_usage));
    }
    print!("{}\n\n", table);
}

fn main() {
    let mut sys = System::new_all();
    loop {
        print!("\x1B[2J\x1B[1;1H"); //Clearing the console
        sys.refresh_all();
        //get_component_temperature(&sys);
        //get_disk_usage(&sys);
        get_processes(&sys);
        std::thread::sleep(std::time::Duration::from_secs(3));
    }
}
