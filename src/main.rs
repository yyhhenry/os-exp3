mod console;
pub mod pcb;
use std::collections::VecDeque;

use anyhow::Result;
use console::print_pcb_table;
use pcb::{PCBListFile, MAX_PRIORITY, MIN_PRIORITY, PCB};

pub const TIME_SLICE: i32 = 2;

#[allow(dead_code)]
pub struct Scheduler {
    ready: VecDeque<PCB>,
    waiting: VecDeque<PCB>,
    running: Option<PCB>,
    finished: Vec<PCB>,
    resource: pcb::Resource,
}
impl Scheduler {
    pub fn new(pcb_list: Vec<PCB>) -> Self {
        assert!(
            pcb_list
                .iter()
                .all(|pcb| pcb.state == pcb::ProcessState::Ready),
            "All processes must be in the ready state at the beginning"
        );
        Self {
            ready: pcb_list.into(),
            waiting: VecDeque::new(),
            running: None,
            finished: Vec::new(),
            resource: pcb::Resource::new(),
        }
    }
    pub fn print_table(&self) {
        let iter = self
            .running
            .iter()
            .chain(self.ready.iter())
            .chain(self.waiting.iter())
            .chain(self.finished.iter());
        print_pcb_table(iter);
    }
    fn ready_from_waiting(&mut self) {
        if let Some(mut pcb) = self.waiting.pop_front() {
            pcb.state = pcb::ProcessState::Ready;
            self.ready.push_back(pcb);
        }
    }
    fn release_resource(&mut self, pid: i32) {
        if self.resource.pid == Some(pid) {
            self.resource.pid = None;
            self.ready_from_waiting();
        }
    }
    fn finish_running(&mut self) {
        if let Some(mut running) = self.running.take() {
            self.release_resource(running.pid);
            running.state = pcb::ProcessState::Finished;
            self.finished.push(running);
        }
    }
    fn running_to_waiting(&mut self) {
        if let Some(mut running) = self.running.take() {
            running.state = pcb::ProcessState::Waiting;
            self.waiting.push_back(running);
        }
    }
    fn detach_running(&mut self) {
        if let Some(mut running) = self.running.take() {
            running.state = pcb::ProcessState::Ready;
            self.ready.push_back(running);
        }
    }
    fn move_highest_priority_to_front(&mut self) {
        if let Some(index) = (0..self.ready.len()).min_by_key(|&i| self.ready[i].priority) {
            self.ready.swap(0, index);
        }
    }
    fn attach_ready(&mut self) {
        assert!(self.running.is_none());
        // You should move the process with the highest priority to the front of the queue before attaching it
        if let Some(mut ready) = self.ready.pop_front() {
            ready.state = pcb::ProcessState::Running;
            ready.running_time_in_slice = 0;
            self.running = Some(ready);
        }
    }
    fn update_priority(&mut self) {
        if let Some(running) = &mut self.running {
            running.priority = (running.priority + 1).min(MAX_PRIORITY);
        }
        for pcb in &mut self.ready {
            pcb.priority = (pcb.priority - 1).max(MIN_PRIORITY);
        }
    }
    pub fn next_tick(&mut self) {
        // Update counters
        if let Some(running) = &mut self.running {
            running.running_time += 1;
            running.running_time_in_slice += 1;
        }
        self.update_priority();
        self.move_highest_priority_to_front();
        if let Some(running) = &self.running {
            if running.running_time >= running.total_time {
                // If the process has finished
                self.finish_running();
            } else if running.running_time >= running.resource_request_time {
                // If the process needs to request resources
                if self.resource.pid.is_none() {
                    self.resource.pid = Some(running.pid);
                } else {
                    self.running_to_waiting();
                }
            } else if running.running_time_in_slice >= TIME_SLICE {
                // If the time slice has been used up
                self.detach_running();
            } else if self
                .ready
                .front()
                .map(|ready| ready.priority < running.priority)
                .unwrap_or(false)
            {
                // If there is a process with a higher priority
                self.detach_running();
            }
        }
        if self.running.is_none() {
            self.attach_ready();
        }
    }
    pub fn run_all(&mut self) {
        loop {
            self.next_tick();
            self.print_table();
            if self.running.is_none() {
                break;
            }
        }
    }
}
fn main() -> Result<()> {
    let pcb_list: Vec<PCB> = PCBListFile::from_file("mock_pcb.json")?.into();
    let mut scheduler = Scheduler::new(pcb_list);
    scheduler.run_all();
    Ok(())
}
