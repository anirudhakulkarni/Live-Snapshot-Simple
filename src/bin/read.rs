extern crate kvm_ioctls;
extern crate kvm_bindings;
use savefile::prelude::*;
use kvm_bindings::kvm_userspace_memory_region;
use std::ptr::null_mut;
use kvm_ioctls::VcpuExit;
use kvm_ioctls::{Kvm, VcpuFd, VmFd};
use bincode;
use serde::{Serialize, Deserialize};
use serde_json;
use savefile::prelude::*;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::mem;
use std::slice;
mod state;

fn main() {

    let mstate = state::restore(&String::from("test"));
    println!("state_cpu: {:?}", mstate.state_cpu);
    println!("state_mem: {:?}", mstate.state_mem);
    
}