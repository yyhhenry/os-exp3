mod console;
pub mod pcb;
mod scheduler;

use anyhow::Result;
use clap::Parser;
use console::print_pcb_table;
use pcb::{PCBListFile, PCB};
use scheduler::Scheduler;
#[derive(Parser)]
struct Args {
    #[clap(short, default_value = "mock_pcb.json")]
    input_file: String,
    #[clap(short, long)]
    fast: bool,
}
fn main() -> Result<()> {
    let args = Args::parse();
    let pcb_list: Vec<PCB> = PCBListFile::from_file(args.input_file)?.into();
    print_pcb_table(&pcb_list);
    let mut scheduler = Scheduler::new(pcb_list);
    scheduler.run_all(args.fast);
    Ok(())
}
