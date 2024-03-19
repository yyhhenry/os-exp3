use std::path::Path;

use anyhow::Result;
use serde::{Deserialize, Serialize};
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProcessState {
    Running,
    Ready,
    Waiting,
    Finished,
}
impl Default for ProcessState {
    fn default() -> Self {
        ProcessState::Ready
    }
}
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ProcessType {
    System,
    User,
}
pub static MAX_PRIORITY: i32 = 19;
pub static MIN_PRIORITY: i32 = -20;
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PCB {
    /// Process ID
    pub pid: i32,
    /// Process Name
    pub name: String,
    /// Process State
    #[serde(default)]
    pub state: ProcessState,
    /// Priority
    /// Lower number means higher priority
    /// from -20 to 19
    pub priority: i32,
    /// Process Type
    pub process_type: ProcessType,
    /// The time the process has been running
    #[serde(default)]
    pub running_time: i32,
    /// The time the process has been running in this time slice
    #[serde(default)]
    pub running_time_in_slice: i32,

    /// [Mock Value] Total time the process will run
    /// The process will be considered finished when the running time exceeds this value
    pub total_time: i32,
    /// [Mock Value] The time for the process to request resources
    /// The process needs to request resources at this time
    /// 0..total_time
    pub resource_request_time: i32,
}
impl PCB {
    pub fn new(
        pid: i32,
        name: String,
        state: ProcessState,
        priority: i32,
        process_type: ProcessType,
        total_time: i32,
        resource_request_time: i32,
    ) -> PCB {
        PCB {
            pid,
            name,
            state,
            priority,
            process_type,
            running_time: 0,
            running_time_in_slice: 0,
            total_time,
            resource_request_time,
        }
    }
}
#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct PCBListFile {
    pcb_list: Vec<PCB>,
}
impl From<Vec<PCB>> for PCBListFile {
    fn from(pcb_list: Vec<PCB>) -> Self {
        PCBListFile { pcb_list }
    }
}
impl Into<Vec<PCB>> for PCBListFile {
    fn into(self) -> Vec<PCB> {
        self.pcb_list
    }
}
impl PCBListFile {
    pub fn from_file(file_path: impl AsRef<Path>) -> Result<PCBListFile> {
        let file = std::fs::File::open(file_path)?;
        let reader = std::io::BufReader::new(file);
        serde_json::from_reader(reader).map_err(Into::into)
    }
}
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Resource {
    /// The process that is using the resource
    pub pid: Option<i32>,
}
impl Resource {
    pub fn new() -> Resource {
        Resource { pid: None }
    }
}
