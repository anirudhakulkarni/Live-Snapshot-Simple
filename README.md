# Live Snapshot : Simple
This is a bareminimum implementation to show possibility of live snapshotting on KVM. This is not a production ready code. This is just a proof of concept. You may visit [this link](https://github.com/anirudhakulkarni/Live-Snapshot) for generic linux live snapshotting.

* Goal: run some instructions inside a VM sandbox, pause the VM, snapshot the VM, resume the VM, and run some more instructions inside the VM sandbox.


Instructions: 

* Add the initial contents of the `al` and `bl` registers (which we will
pre-initialize to 2), 
* take snapshot,
* resume snapshot,
* convert the resulting sum (4) to ASCII by adding '0',
* output it to a serial port at `0x3f8` 
* followed by a newline, and then 
* halt.

Requirements: 

* setup VM
* load the code into guest memory. 
* Emulate a simple IO device: a serial port.
* run vm


### state.rs

This file contains the state of the VM. It contains the registers, memory, and the IO device. The struct provides methods to save and resotre state
of VM.

### simple.rs

* kvm::new() does ioctl calls 
* kvm.create_vm() does ioctl calls
* create_vpu() does ioctl calls and also map kvm_run structure
* set_user_memory_region does ioctl calls
* you can get a raw pointer. writing to raw pointer is **unsafe**. Pointers 
may be null. Rust references can never be null. You have to do unsafe things at
times, but you try to create a safe abstraction over unsafe blocks. Minimize
unsafe blocks.

* get_regs, get_sregs do ioctl calls
* Let's run, we get 4.
* Let's print registers after we halt.
```
rdx : 1016 = 0x3f8
rax : 10 = int ascii value of "\n"
rbx : 2. Unchanged
rip : 4108 = 0x100C. 0x1000 starting address, C = len(asm_code) = 12. 
rflags : 2. 
```

But we stop the execution once we exit to output. We save snapshot as registers and cpu state. 

### restore.rs

* We restore the snapshot taken in `simple.rs` and run again. We get 4 again as output.
### simple_in.rs

Let's take input. Set rax to 3. Let's print the registers again: 

```
rflags : 6 = 4 + 2. 4 => Parity flag since the sum is "5". 2 bit is always 1.
```

#### simple_mm.rs

* Let's load / store stuff to / from memory and see the program loading storing
stuff. 
* Set a flag that we want to see memory reads and writes.
* See exits

#### simple_ss.rs

* Let's back the memory with a file. 
* After halt we can see what was the state of the memory.


## Running

You can run the code using `cargo run --bin simple`. You can run the other binaries using `cargo run --bin <binary_name>`.
