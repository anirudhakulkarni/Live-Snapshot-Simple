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

// extern crate user_lib;

// use user_lib::syscall::*;

mod state;

#[derive(Debug)]
struct StateCPU {
    regs: kvm_bindings::kvm_regs,
    sregs: kvm_bindings::kvm_sregs,
}

#[derive(Debug, Serialize, Deserialize)]
struct StateMem {
    mem: Vec<u8>
}

// fn byte_to_state(buffer: Vec<u8>)-> State{
//     for i in 0..buffer.len
// }

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {

    let mut mstate = state::restore(&String::from("test"));
    println!("state_cpu: {:?}", mstate.state_cpu);
    // println!("state_mem: {:?}", mstate.state_mem);

    let mem_size = mstate.state_mem.mem.len();
    
    let guest_addr = 0x1000;

    let asm_code: &[u8] = &[
        0xba, 0xf8, 0x03, 
        0x00, 0xd8, 
        0x04, '0' as u8,
        0xee,
        0xb0, '\n' as u8,
        0xee,
        0xc6, 0x06, 0x00, 0x80,
        0x00, /* movl $0, (0x8000); This generates a MMIO Write. */
        0x8a, 0x16, 0x00, 0x80, /* movl (0x8000), %dl; This generates a MMIO Read. */
        0xf4];
        
    // 1. Instantiate KVM.
    let kvm = Kvm::new().unwrap();

    // 2. Create a VM.
    let vm = kvm.create_vm().unwrap();

    // let fd = open("test_mmap.txt", OpenFlag::RDWR | OpenFlag::CREAT, 0).unwrap();
    // write(fd, mstate.state_mem.mem.as_bytes()).unwrap();
    // // let stat = fstat(fd).unwrap();
    // let ptr =
    //     libc::mmap(
    //         null_mut(), 
    //         mem_size as usize, 
    //         libc::PROT_READ | libc::PROT_WRITE, 
    //         libc::MAP_ANONYMOUS | libc::MAP_SHARED | libc::MAP_NORESERVE, 
    //         fd, 
    //         0
    //     );

    // let load_addr = ptr as *mut u8;

    // // 3. Initialize Guest Memory.
    let load_addr: *mut u8 = unsafe {
        libc::mmap(
            null_mut(),
            mem_size,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_ANONYMOUS | libc::MAP_SHARED | libc::MAP_NORESERVE,
            -1,
            0,
        ) as *mut u8
    };

    //copy state mem to load_addr
    unsafe {
        std::ptr::copy_nonoverlapping(mstate.state_mem.mem.as_ptr(), load_addr, mem_size);
    }

    let slot = 0;
    let mem_region = kvm_userspace_memory_region {
        slot,
        guest_phys_addr: guest_addr,
        memory_size: mem_size as u64,
        userspace_addr: load_addr as u64,
        flags: 0,
    };
    unsafe { vm.set_user_memory_region(mem_region).unwrap() };

    
    // 4. Create one vCPU.
    let vcpu_fd = vm.create_vcpu(0).unwrap();
    let mut vcpu_sregs = mstate.state_cpu.sregs;
    let mut vcpu_regs = mstate.state_cpu.regs;
    vcpu_fd.set_regs(&vcpu_regs).unwrap();
    vcpu_fd.set_sregs(&vcpu_sregs).unwrap();
    println!("hi cpu_regs {:#?}", vcpu_fd.get_regs());

    // 6. Run code on the vCPU.
    loop {
        match vcpu_fd.run().expect("run failed") {
            VcpuExit::IoOut(addr, data) => {
                println!(
                    "Received an I/O out exit. Address: {:#x}. Data: {:#x}",
                    addr, data[0],
                );
                println!("{}", data[0] as char);
            }
            VcpuExit::IoIn(addr, data) => {
                println!(
                    "Received an I/O in exit. Address: {:#x}. Data: {:#x}",
                    addr, data[0],
                );
                data[0] = 5;
            }
            VcpuExit::Hlt => {
                // let vcpu_regs = vcpu_fd.get_regs().unwrap();
                // println!("cpu_regs {:#?}", vcpu_regs);
                break;
            }
            r =>{
                println!("exit: {:?}", r);
                
                println!("cpu_regs {:#?}", vcpu_fd.get_regs());
                
                break;               
            } 
        }
    }
}        

