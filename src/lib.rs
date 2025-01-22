#![feature(test)]

/// ```rust
/// assert_eq!(99 * 321, gearbox_test::multiply_by_321(99));
/// assert_eq!(0 * 321, gearbox_test::multiply_by_321(0));
/// assert_eq!(-99 * 321, gearbox_test::multiply_by_321(-99));
/// ```
pub fn multiply_by_321(value: i32) -> i32 {
    value.wrapping_shl(8) + value.wrapping_shl(6) + value
}

/// ```rust
/// fn test_filter_array<const SIZE: usize>(input: [i32; SIZE], expected: [i32; SIZE]) {
///     let mut clear_input = input.clone();
///     gearbox_test::simple_filter_array(&mut clear_input);
///     assert_eq!(clear_input, expected);
/// }
/// test_filter_array([1, 2, 3, 4], [1, 2, 3, 4]);
/// test_filter_array([1, 2, 3, 4, 0], [1, 2, 3, 4, 0]);
/// test_filter_array([0, 1, 2, 3, 4], [1, 2, 3, 4, 0]);
/// test_filter_array([0, 1, 0, 2, 3], [1, 2, 3, 0, 0]);
/// test_filter_array([0], [0]);
/// test_filter_array([], []);
/// ```
pub fn simple_filter_array(array: &mut [i32]) {
    for index in 0..array.len() {
        // If current element is a zero rotate all elements from this index left 1 element, effectively leaving the
        // zero in the last element of the array
        if array[index] == 0 {
            array[index..].rotate_left(1);
        }
    }
}

/// ```rust
/// fn test_filter_array<const SIZE: usize>(input: [i32; SIZE], expected: [i32; SIZE]) {
///     let mut fast_input = input.clone();
///     gearbox_test::fast_filter_array(&mut fast_input);
///     assert_eq!(fast_input, expected);
/// }
/// test_filter_array([1, 2, 3, 4], [1, 2, 3, 4]);
/// test_filter_array([1, 2, 3, 4, 0], [1, 2, 3, 4, 0]);
/// test_filter_array([0, 1, 2, 3, 4], [1, 2, 3, 4, 0]);
/// test_filter_array([0, 1, 0, 2, 3], [1, 2, 3, 0, 0]);
/// test_filter_array([0], [0]);
/// test_filter_array([], []);
/// ```
pub fn fast_filter_array(array: &mut [i32]) {
    let mut write_index = 0;
    let mut read_offset = 0;

    // Fast-forward through the array until the first zero is hit
    while write_index < array.len() {
        if array[write_index] == 0 {
            break;
        } else {
            write_index += 1;
        }
    }

    while write_index < array.len() {
        if let Some(element) = array.get(write_index + read_offset) {
            // Copy the current index + read offset to the current location or move the offset if a zero is found
            if *element == 0 {
                read_offset += 1;
            } else {
                if read_offset > 0 {
                    array[write_index] = *element;
                }
                write_index += 1;
            }
        } else {
            // If the array.get fails then there are no more elements, zero the rest of the array
            array[write_index..].fill(0);
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{array, mem};

    extern crate test;

    fn simple_filter_bench<const SIZE: usize>(b: &mut test::Bencher, test_array: &[i32; SIZE]) {
        b.bytes = mem::size_of_val(test_array) as u64;
        b.iter(|| {
            let mut test_clone = test_array.clone();
            super::simple_filter_array(&mut test_clone)
        });
    }

    #[bench]
    fn filter_array(b: &mut test::Bencher) {
        let test_array: [i32; 10000] = array::from_fn(|i| (i % 10) as i32);
        simple_filter_bench(b, &test_array);
    }

    #[bench]
    fn filter_array_zeroless(b: &mut test::Bencher) {
        let test_array: [i32; 10000] = [1i32; 10000];
        simple_filter_bench(b, &test_array);
    }

    #[bench]
    fn filter_array_oops_all_zeros(b: &mut test::Bencher) {
        let test_array: [i32; 10000] = [0i32; 10000];
        simple_filter_bench(b, &test_array);
    }

    fn fast_filter_bench<const SIZE: usize>(b: &mut test::Bencher, test_array: &[i32; SIZE]) {
        b.bytes = mem::size_of_val(test_array) as u64;
        b.iter(|| {
            let mut test_clone = test_array.clone();
            super::fast_filter_array(&mut test_clone)
        });
    }

    #[bench]
    fn fast_filter_array(b: &mut test::Bencher) {
        let test_array: [i32; 10000] = array::from_fn(|i| (i % 10) as i32);
        fast_filter_bench(b, &test_array);
    }

    #[bench]
    fn fast_filter_array_zeroless(b: &mut test::Bencher) {
        let test_array: [i32; 10000] = [1i32; 10000];
        fast_filter_bench(b, &test_array);
    }

    #[bench]
    fn fast_filter_array_oops_all_zeros(b: &mut test::Bencher) {
        let test_array: [i32; 10000] = [0i32; 10000];
        fast_filter_bench(b, &test_array);
    }
}

// 3.  Instruction set reference:

#[derive(Debug)]
pub struct GearBoxCpu {
    registers: [i32; 2],
    memory: [i32; GearBoxCpu::MEMORY_SIZE],
}

impl GearBoxCpu {
    // 2kb was enough for the NES so it's enough for me
    pub const MEMORY_SIZE: usize = 2048 / size_of::<i32>();
}

// Could be simplified, but I'm simulating this like an emulator
#[derive(Clone, Copy)]
pub struct Address(u16);

impl From<Address> for usize {
    fn from(value: Address) -> Self {
        println!(
            "{} {} {} {}",
            align_of_val(&0),
            align_of_val(&1),
            align_of_val(&2),
            align_of_val(&3)
        );
        value.0 as usize / size_of::<i32>()
    }
}

impl GearBoxCpu {
    pub fn new() -> Self {
        Self {
            registers: [-1i32; 2],
            memory: [0i32; GearBoxCpu::MEMORY_SIZE],
        }
    }

    pub fn mem(&mut self, address: Address) -> &mut i32 {
        if let Some(memory) = self.memory.get_mut(<Address as Into<usize>>::into(address)) {
            memory
        } else {
            panic!("Segfault");
        }
    }

    fn read_reg(&self, register: Register) -> i32 {
        match register {
            Register::Reg1 => self.registers[0],
            Register::Reg2 => self.registers[1],
        }
    }

    fn write_reg(&mut self, value: i32, register: Register) {
        *match register {
            Register::Reg1 => &mut self.registers[0],
            Register::Reg2 => &mut self.registers[1],
        } = value;
    }

    // instruction:
    // mov mem[y], reg[x]

    // performs:
    // mem[y] <- reg[x]
    // copies value in register[x] to memory location[y]
    // NOTE:  can only copy from register to memory

    // example:
    // mov mem1, reg1
    pub fn mov(&mut self, out: Address, op: Register) {
        *self.mem(out) = self.read_reg(op);
        // self.write_mem(self.read_reg(op), out);
    }

    // instruction:
    // sub reg[x], mem[y]

    // performs:
    // reg[x] <- reg[x] - mem[y]
    // subtracts the number in memory location[y] from the number in register[x]
    // and stores the result in register[x]

    // example:
    // sub reg1, mem1
    pub fn sub(&mut self, out: Register, op: Address) {
        let value = *self.mem(op);
        self.write_reg(self.read_reg(out) - value, out);
    }
}

#[derive(Clone, Copy)]
pub enum Register {
    Reg1,
    Reg2,
}

// Given the above reference, use the mov and sub instructions to move a
// value from mem1 to mem2.

pub const TEMP: Address = Address(0);
pub const MEM1: Address = Address(4);
pub const MEM2: Address = Address(8);

/// ```rust
/// use gearbox_test::*;
/// let memory = [0i32; GearBoxCpu::MEMORY_SIZE];
/// let cpu = &mut GearBoxCpu::new();
/// *cpu.mem(MEM1) = -42;
/// *cpu.mem(MEM2) = 1999;
/// assert_eq!(*cpu.mem(MEM1), -42);
/// assert_eq!(*cpu.mem(MEM2), 1999);
/// swap_memory(cpu);
/// assert_eq!(*cpu.mem(MEM1), 1999);
/// assert_eq!(*cpu.mem(MEM2), -42);
/// ```
pub fn swap_memory(cpu: &mut GearBoxCpu) {
    fn load(cpu: &mut GearBoxCpu, out: Register, address: Address) {
        // First two ops inverts the value of the memory address
        cpu.sub(out, address);
        cpu.mov(address, out);
        // Next two ops zeros the register then inverts it
        cpu.sub(out, address);
        cpu.sub(out, address);
    }

    // Clearing the register after store, the clear step could be cleared but it seems better to leave the registers in
    // a zeroed state
    fn store_and_clear(cpu: &mut GearBoxCpu, out: Address, register: Register) {
        cpu.mov(out, register);
        // clear
        cpu.sub(register, out);
    }

    // This isn't explicitly called out in the requirements, but the initial state of memory is often undefined and
    // registers can contain unexpected values from previous executions. This method will zero out a register using a
    // temporary memory location.
    store_and_clear(cpu, TEMP, Register::Reg2);
    store_and_clear(cpu, TEMP, Register::Reg1);

    // Actual work to swap the memory locations
    load(cpu, Register::Reg1, MEM1);
    load(cpu, Register::Reg2, MEM2);
    store_and_clear(cpu, MEM1, Register::Reg2);
    store_and_clear(cpu, MEM2, Register::Reg1);
}
