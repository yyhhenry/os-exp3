use crate::pcb::PCB;
use prettytable::{row, Table};
pub fn print_pcb_table<'a, T>(pcb_list: T)
where
    T: IntoIterator<Item = &'a PCB>,
{
    let mut pcb_list: Vec<&PCB> = pcb_list.into_iter().collect();
    pcb_list.sort_by_key(|pcb| &pcb.state);
    let mut table = Table::new();
    table.add_row(row![
        "PID",
        "Name",
        "State",
        "Priority",
        "Type",
        "Running Time",
        "Total Time",
        "Resource Request Time"
    ]);
    for pcb in pcb_list {
        table.add_row(row![
            pcb.pid,
            pcb.name,
            format!("{:?}", pcb.state),
            pcb.priority,
            format!("{:?}", pcb.process_type),
            pcb.running_time,
            pcb.total_time,
            pcb.resource_request_time
        ]);
    }
    table.printstd();
}
