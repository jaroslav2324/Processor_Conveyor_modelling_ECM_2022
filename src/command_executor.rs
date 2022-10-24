use crate::status;
use status::Status;

pub struct CommandExecutor{
    left_clocks: u8,
    status: Status,
}

impl CommandExecutor {

    pub fn new() -> CommandExecutor{
        CommandExecutor{
            left_clocks: 0,
            status: Status::Idle,
        }
    }

    pub fn is_clocks_zero(&self) -> bool{
        if self.left_clocks == 0{
            return true;
        }
        if self.left_clocks > 0{
            return  false;
        }
        panic!("Clocks in command executor are below zero!");

    }

    pub fn get_status(&self) -> Status{
        return match self.status {
            Status::Done =>  Status::Done,
            Status::Executing =>  Status::Executing,
            Status::Idle => Status::Idle
        }
    }

    pub fn load_data(&mut self, clocks: u8) -> (){
        self.left_clocks = clocks;
        self.status = Status::Executing;
    }

    pub fn is_executing(&self) -> bool{
        if self.status == Status::Executing{
            return true;
        }
        false
    }

    pub fn is_done(&self) -> bool{
        if self.status == Status::Done{
            return true;
        }
        false
    }

    pub fn clock_once(&mut self) -> (){

        if self.left_clocks > 0{ self.left_clocks -= 1; }
        else { self.status = Status::Done }
    }
}
