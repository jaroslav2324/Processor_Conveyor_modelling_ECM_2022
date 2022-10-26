
use crate::status;
use status::Status;

pub struct Writer{
    left_clocks: u8,
    status: Status,
}

impl Writer {

    pub fn new() ->Writer{
        Writer{
            left_clocks: 0,
            status: Status::Idle,
        }
    }

    pub fn get_status(&self) -> Status{
        return match self.status {
            Status::Done =>  Status::Done,
            Status::Executing =>  Status::Executing,
            Status::Idle => Status::Idle
        }
    }

    pub fn load_data(&mut self, clocks: &u8) -> (){
        self.left_clocks = *clocks;
        self.status = Status::Executing;
    }

    pub fn set_clocks(&mut self, clocks: u8) -> (){
        self.left_clocks = clocks;
        if self.left_clocks <= 0 && self.status == Status::Executing {
            self.status = Status::Done;
        }
    }

    pub fn get_clocks(&self) -> u8{
        return self.left_clocks;
    }

}