use chrono::DateTime;
use std;
use sysinfo::*;

fn get_disk_usage(sys: &System) {
    let mut table = tabular::Table::new("{:<}  {:<} {:<} {:<}");
    table.add_heading("Hardware Temperature\n");
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
    let mut table = tabular::Table::new("{:<}  {:>} {:>} {:>}");
    table.add_heading("Disk Usage\n");
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

fn main() {
    let mut sys = System::new_all();

    loop {
        //print!("\x1B[2J\x1B[1;1H"); //Clearing the console
        std::process::Command::new("clear").spawn().unwrap();
        sys.refresh_all();
        get_component_temperature(&sys);
        get_disk_usage(&sys);
        std::thread::sleep(std::time::Duration::from_secs(3));
    }
}
