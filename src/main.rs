use std::collections::VecDeque;
use rand::Rng;

use std::io::{BufRead, BufReader};
use std::fs::File;


mod memory_accessor;
mod operand;
mod writer;
mod calculator;
mod command_executor;
mod status;

use memory_accessor::MemoryAccessor;
use operand::Operand;
use writer::Writer;
use calculator::Calculator;
use status::Status;
use crate::command_executor::CommandExecutor;

/*checks all statuses of processor parts and if they are Done return true*/
fn is_time_to_shift_conveyor(cmd_exec: &CommandExecutor,
                             calc: &Calculator, accessor: &MemoryAccessor) -> bool{

    if  !cmd_exec.is_executing() &&
        !accessor.is_executing() &&
        !calc.is_executing()
        {
        return true;
    }
    return false;
}



fn main() {


    // enter parameters

    let p_register_adresation = 0.9;

    let memory_access_amount_clocks = 2;

    let second_command_amount_clocks = 4;

    let p_command_first_type = 0.9;

    // generate list of entering commands

    let mut accessor = MemoryAccessor::new();

    let mut command_executor = CommandExecutor::new();

    let mut operand1 = Operand::new();
    let mut operand2 = Operand::new();
    let mut writer = Writer::new();

    let mut calculator = Calculator::new();

    // load list of commands
    let mut vec_command_types: VecDeque<i8> = VecDeque::new();
    let mut vec_op1_types: VecDeque<std::string::String> = VecDeque::new();
    let mut vec_op2_types: VecDeque<std::string::String> = VecDeque::new();

    let reader = BufReader::new(File::open("C://Users//HP//CLionProjects//ecm_curs2//input.txt")
        .expect("Cannot open input.txt"));

    let mut cnt = 0;
    for line in reader.lines() {
        cnt = 0;
        for word in line.unwrap().split_whitespace() {
            if cnt == 0 { vec_command_types.push_back(word.to_string().parse::<i8>().unwrap())}
            else if cnt == 1 { vec_op1_types.push_back(word.to_string())}
            else if cnt == 2 { vec_op2_types.push_back(word.to_string())}
            cnt += 1;
        }
    }

    let input_available: bool = true;
    let command_type = 1;
    let op2_type = "REG";
    let op1_type = "REG";

    // loop of clocks
    loop {

        // load data to blocks
        // 5 statuses: readCmd, readOp1, readOp2, Calc, Writer.
        // if all statuses "Done" and no more input => break
        if  is_time_to_shift_conveyor(&command_executor,
                                      &calculator, &accessor) && !input_available{
            break;
        }
        // if all statuses "Done" => shift conveyor belt
        if  is_time_to_shift_conveyor(&command_executor,
                                      &calculator, &accessor){

            // shift conveyor

            // add which type would be next
            writer.load_data(&second_command_amount_clocks);
            accessor.add_device_to_queue(2, second_command_amount_clocks);

            // load calculator
            match command_type {
                1 => calculator.load_data(1),
                2 => calculator.load_data(second_command_amount_clocks),
                _ => panic!()
            }

            // load 2 operand
            let amount_clocks = match op2_type {
                "REG" => 1,
                "MEM" => memory_access_amount_clocks,
                _ => panic!()
            };
            operand2.load_data(amount_clocks);
            accessor.add_device_to_queue(1, amount_clocks);

            // load 1 operand
            let amount_clocks = match op1_type {
                "REG" => 1,
                "MEM" => memory_access_amount_clocks,
                _ => panic!()
            };
            operand1.load_data(amount_clocks);
            accessor.add_device_to_queue(0, amount_clocks);

            // load command executor
            if input_available {
                command_executor.load_data(1);
            }
        }


        command_executor.clock_once();
        accessor.clock_once();
        writer.clock_once();
    }


}
