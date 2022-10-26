use std::collections::VecDeque;
use crate::operand::Operand;

use crate::Status;
use crate::writer::Writer;


pub struct MemoryAccessor {
    index_current_device: i8,
    array_left_clock_devices: [u8; 3],
    queue_index_devices: VecDeque<i8>,
    status: Status,
}

impl MemoryAccessor {
    pub fn new() -> MemoryAccessor {
        MemoryAccessor {
            index_current_device: -1,
            array_left_clock_devices: [0, 0, 0],
            queue_index_devices: VecDeque::new(),
            status: Status::Idle
        }
    }

    pub fn add_device_to_queue(&mut self, index_device: i8, amount_clocks_for_memory_accessing: u8) -> () {
        if self.array_left_clock_devices[index_device as usize] > 0 {
            panic!()
        }

        self.queue_index_devices.push_back(index_device);
        self.array_left_clock_devices[index_device as usize] = amount_clocks_for_memory_accessing;
        self.status = Status::Executing
    }

    pub fn clock_once(&mut self,  op1: &mut Operand, op2: &mut Operand, writer: &mut Writer) -> () {
        if self.index_current_device == -1 {
            self.choose_next_device();
        }
        // still -1 after choosing next device
        if self.index_current_device == -1 {
            self.status = Status::Done;
            return;
        }

        self.array_left_clock_devices[self.index_current_device as usize] -= 1;
        if self.array_left_clock_devices[self.index_current_device as usize] <= 0 {
            self.choose_next_device();
        }

        if self.index_current_device == -1 {
            self.status = Status::Done;
        }

        op1.set_clocks(self.array_left_clock_devices[0]);
        op2.set_clocks(self.array_left_clock_devices[1]);
        writer.set_clocks(self.array_left_clock_devices[2]);


    }

    fn choose_next_device(&mut self) -> () {
        if self.queue_index_devices.is_empty() {
            self.index_current_device = -1;
            return;
        }

        self.index_current_device = self.queue_index_devices.pop_front().expect("Deque is empty!");
    }

    pub fn is_executing(&self) -> bool{
        if self.status == Status::Executing{
            return true;
        }
        false
    }
}