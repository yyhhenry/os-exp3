use std::{collections::VecDeque, thread::sleep, time::Duration};

use crate::console::print_pcb_table;
use crate::pcb::{ProcessState, Resource, MAX_PRIORITY, MIN_PRIORITY, PCB};
use prettytable::table;

pub const TIME_SLICE: i32 = 2;

#[allow(dead_code)]
pub struct Scheduler {
    ready: VecDeque<PCB>,
    waiting: VecDeque<PCB>,
    running: Option<PCB>,
    finished: Vec<PCB>,
    resource: Resource,
}
impl Scheduler {
    pub fn new(pcb_list: Vec<PCB>) -> Self {
        assert!(
            pcb_list.iter().all(|pcb| pcb.state == ProcessState::Ready),
            "All processes must be in the ready state at the beginning"
        );
        Self {
            ready: pcb_list.into(),
            waiting: VecDeque::new(),
            running: None,
            finished: Vec::new(),
            resource: Resource::new(),
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
    fn wakeup_waiting(&mut self) {
        if let Some(mut pcb) = self.waiting.pop_front() {
            pcb.state = ProcessState::Ready;
            println!("PID: {} is woken up to the ready state", pcb.pid);
            self.ready.push_back(pcb);
        }
    }
    fn release_resource(&mut self) {
        if let Some(running) = &self.running {
            if self.resource.pid == Some(running.pid) {
                self.resource.pid = None;
                println!("Resource released by PID: {}", running.pid);
                self.wakeup_waiting();
            }
        }
    }
    fn finish_running(&mut self) {
        self.release_resource();
        if let Some(mut running) = self.running.take() {
            println!("PID: {} has finished", running.pid);
            running.state = ProcessState::Finished;
            self.finished.push(running);
        }
    }
    fn block_running(&mut self) {
        if let Some(mut running) = self.running.take() {
            println!("PID: {} is waiting for the resource", running.pid);
            running.state = ProcessState::Waiting;
            self.waiting.push_back(running);
        }
    }
    fn preempt_running(&mut self) {
        if let Some(mut running) = self.running.take() {
            println!("PID: {} has been preempted", running.pid);
            running.state = ProcessState::Ready;
            self.ready.push_back(running);
        }
    }
    fn move_highest_priority_to_front(&mut self) {
        (0..self.ready.len())
            .min_by_key(|&index| self.ready[index].priority)
            .and_then(|index| self.ready.remove(index))
            .map(|pcb| self.ready.push_front(pcb));
    }
    fn dispatch_ready(&mut self) {
        assert!(
            self.running.is_none(),
            "The running process must be detached first"
        );
        if let Some(mut ready) = self.ready.pop_front() {
            ready.state = ProcessState::Running;
            ready.running_time_in_slice = 0;
            println!("PID: {} is dispatched", ready.pid);
            self.running = Some(ready);
        }
    }
    fn occupy_resource(&mut self) {
        if let Some(running) = &mut self.running {
            assert!(self.resource.pid.is_none(), "The resource must be free");
            println!("Resource occupied by PID: {}", running.pid);
            self.resource.pid = Some(running.pid);
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
    pub fn dispatch(&mut self) {
        self.update_priority();
        self.move_highest_priority_to_front();
        if let Some(running) = &self.running {
            if running.running_time_in_slice >= TIME_SLICE {
                println!("Time slice used up by PID: {}", running.pid);
                self.preempt_running();
            } else if self
                .ready
                .front()
                .map(|ready| ready.priority < running.priority)
                .unwrap_or(false)
            {
                self.preempt_running();
            }
        }
        if self.running.is_none() {
            self.dispatch_ready();
        }
    }
    pub fn run(&mut self, fast: bool) -> i32 {
        if let Some(running) = &mut self.running {
            if running.running_time == running.resource_request_time {
                if self.resource.pid.is_none() {
                    self.occupy_resource();
                } else if self.resource.pid != Some(running.pid) {
                    self.block_running();
                }
            }
        }
        if let Some(running) = &mut self.running {
            table!([format!("[INFO] PID: {} runs for 1 tick", running.pid)]).printstd();
            if !fast {
                sleep(Duration::from_secs_f32(0.5));
            }
            running.running_time += 1;
            running.running_time_in_slice += 1;
            if running.running_time == running.total_time {
                self.finish_running();
            }
            return 1;
        }
        return 0;
    }
    pub fn run_all(&mut self, fast: bool) {
        let mut tick = 0;
        loop {
            tick += self.run(fast);
            self.dispatch();
            println!("+ PCB list (tick = {})", tick);
            self.print_table();
            if self.running.is_none() {
                break;
            }
        }
    }
}
