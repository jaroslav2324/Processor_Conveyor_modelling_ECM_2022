use std::fmt;
use std::fmt::{Formatter, write};

#[derive(PartialEq)]
pub enum Status{
    Done,
    Executing,
    Idle
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let status = match *self {
            Status::Done => "Done",
            Status::Executing => "Executing",
            Status::Idle => "Idle"
        };
        write!(f, "{}", status)
    }
}
