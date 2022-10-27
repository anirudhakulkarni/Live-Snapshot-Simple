extern crate kvm_ioctls;
extern crate kvm_bindings;
use std::fs::File;
use std::io::Read;
use std::mem;
use std::io::Write;
use serde::{Serialize, Deserialize};
use versionize::{VersionMap, Versionize, VersionizeResult};
use versionize_derive::Versionize;

#[derive(Debug, Versionize)]
pub struct StateCPU {
    pub regs: kvm_bindings::kvm_regs,
    pub sregs: kvm_bindings::kvm_sregs,
}

#[derive(Debug, Serialize, Deserialize, Versionize)]
pub struct StateMem {
    pub mem: Vec<u8>
}
#[derive(Debug, Versionize)]
pub struct State {
    pub state_cpu: StateCPU,
    pub state_mem: StateMem
}

pub fn save(state: State, filename: &String) {
    // save the state of the cpu

    let mut mem = vec![0u8; std::mem::size_of::<State>()];
    // state.se
    let bytes = unsafe { std::mem::transmute::<StateCPU, [u8; std::mem::size_of::<StateCPU>()]>(state.state_cpu) };
    // write the bytes to a file

    let mut name: String = filename.to_string();
    name.push_str("_cpu");
    let mut file_cpu = File::create(name).unwrap();
    file_cpu.write_all(&bytes).unwrap();

    // save the state of the memory by serializing it
    let bytes = bincode::serialize(&state.state_mem).unwrap();
    // write the bytes to a file
    name = filename.to_string();
    name.push_str("_mem");
    let mut file_mem = File::create(name).unwrap();
    file_mem.write_all(&bytes).unwrap(); 
}

pub fn restore(filename: &String) -> State {
    
    // restore the state of the cpu
    let mut name: String = filename.to_string();
    name.push_str("_cpu");
    let mut file_cpu = File::open(name).unwrap();
    let mut bytes = [0; std::mem::size_of::<StateCPU>()];
    file_cpu.read_exact(&mut bytes).unwrap();
    let state_cpu :StateCPU = unsafe { std::mem::transmute::<[u8; std::mem::size_of::<StateCPU>()], StateCPU>(bytes) };
    
    // restore the state of the memory
    name = filename.to_string();
    name.push_str("_mem");
    let mut file_mem = File::open(name).unwrap();
    let mut bytes = Vec::new();
    file_mem.read_to_end(&mut bytes).unwrap();
    let state_mem : StateMem = bincode::deserialize(&bytes).unwrap();

    State {
        state_cpu,
        state_mem
    }
}

