use sysinfo::{System, get_current_pid, Pid};

use crate::error::Error;
use crate::numeric::{Scalar, Value};

use super::*;
use super::optypes::VmOp;


struct Vminfo_ {
    system: System,
    pid: Pid,
}

pub struct Vminfo(Option<Vminfo_>);

impl Vminfo {
    pub fn new() -> Self {
        match get_current_pid() {
            Err(s) => {
                eprintln!("Unable to read process info {}", s);
                Self(None)
            },
            Ok(pid) => Self(Some(Vminfo_ {system: System::new(), pid})),
        }
    }
}
                

pub fn vmstatus(_: Vec<Frame>, vm: &mut Vm) -> Result<Vec<Frame>, Error> {
    let(max, used) = match vm.vminfo.0 {
        None => (-1, -1),
        Some(ref mut vminfo) => {
            vminfo.system.refresh_pids(&[vminfo.pid]);
            if let Some(process) = vminfo.system.process(vminfo.pid) {
                (process.virtual_memory() as i64, process.memory() as i64)
            } else {
                (-1, -1)
            }
        },
    };

    Ok(vec![Num::Int(Scalar(Value(max))).into(), Num::Int(Scalar(Value(used))).into()])
}
pub const VMSTATUS: VmOp = VmOp::new("vmstats", vmstatus, 0);
