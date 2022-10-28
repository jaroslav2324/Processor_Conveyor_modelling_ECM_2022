use std::collections::VecDeque;

use std::io::{BufRead, BufReader};
use std::fs::File;


mod memory_accessor;
mod operand;
mod writer;
mod calculator;
mod command_executor;
mod status;
mod file_generator;

use memory_accessor::MemoryAccessor;
use operand::Operand;
use writer::Writer;
use calculator::Calculator;
use status::Status;
use crate::command_executor::CommandExecutor;
use crate::file_generator::generate_file;

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

    let amount_commands = 1000;

    let p_register_addressing = 0.6;
    let p_command_first_type = 0.5;

    generate_file("/home/jaros/CLionProjects/Processor_Conveyor_modelling_ECM_2022/input.txt",
                  amount_commands,
                  p_register_addressing,
                  p_command_first_type);

    let memory_access_amount_clocks: u8 = 2;
    let second_command_amount_clocks: u8 = 4;

    // generate list of entering commands

    let mut accessor = MemoryAccessor::new();

    let mut command_executor = CommandExecutor::new();

    let mut operand1 = Operand::new();
    let mut operand2 = Operand::new();
    let mut writer = Writer::new();

    let mut calculator = Calculator::new();

    // load list of commands
    let mut vec_command_types: VecDeque<u8> = VecDeque::new();
    let mut vec_op1_types: VecDeque<String> = VecDeque::new();
    let mut vec_op2_types: VecDeque<String> = VecDeque::new();
    let mut vec_writer_types: VecDeque<String>;

    // read text from file
    let reader = BufReader::new(File::open("/home/jaros/CLionProjects/Processor_Conveyor_modelling_ECM_2022/input.txt")
        .expect("Cannot open input.txt"));

    // fill lists with command types and operand types
    let mut cnt;
    for line in reader.lines() {
        cnt = 0;
        for word in line.unwrap().split_whitespace() {
            if cnt == 0 { vec_command_types.push_back(word.to_string().parse::<u8>().unwrap())}
            else if cnt == 1 { vec_op1_types.push_back(word.to_string())}
            else if cnt == 2 { vec_op2_types.push_back(word.to_string())}
            cnt += 1;
        }
    }

    vec_writer_types = vec_op2_types.clone();

    // calculate amount of clocks if it was sequential processing of commands
    let mut seq_proc_clocks: i32 = 0;

    // clocks for decoding command
    for _elm in &vec_command_types{
        seq_proc_clocks += 1;
    }
    // clocks for accessing first operand
    for _elm in &vec_op1_types{
        match _elm.as_str() {
            "REG" => seq_proc_clocks += 1,
            "MEM" => seq_proc_clocks += memory_access_amount_clocks as i32,
            _ => panic!("No such memory type: {}", _elm)
        }
    }
    // clocks for accessing second operand
    for _elm in &vec_op2_types{
        match _elm.as_str() {
            "REG" => seq_proc_clocks += 1,
            "MEM" => seq_proc_clocks += memory_access_amount_clocks as i32,
            _ => panic!("No such memory type: {}", _elm)
        }
    }
    // clocks for command executing
    for _elm in &vec_command_types{
        match _elm {
            1 => seq_proc_clocks += 1,
            2 => seq_proc_clocks += second_command_amount_clocks as i32,
            _ => panic!("No such command type: {}", _elm)
        }
    }
    // clocks for writing result using the address of the second operand
    for _elm in &vec_writer_types{
        match _elm.as_str() {
            "REG" => seq_proc_clocks += 1,
            "MEM" => seq_proc_clocks += memory_access_amount_clocks as i32,
            _ => panic!("No such memory type: {}", _elm)
        }
    }

    let mut input_available: bool = true;

    let mut clocks_counter = 0;

    // loop of clocks
    loop {

        // load data to blocks
        // 5 statuses: readCmd, readOp1, readOp2, Calc, Writer.
        // if all statuses "Done" and no more input => break
        if  is_time_to_shift_conveyor(&command_executor,
                                      &calculator, &accessor)
            && !input_available
            && vec_op1_types.is_empty()
            && vec_op2_types.is_empty()
            && vec_command_types.is_empty()
            && vec_writer_types.is_empty(){
            break;
        }
        // if all statuses "Done" => shift conveyor belt
        if  is_time_to_shift_conveyor(&command_executor,
                                      &calculator, &accessor){

            // shift conveyor

            // if calculator passes value to writer
            if calculator.is_done() && !vec_writer_types.is_empty(){

                let write_object_type = vec_writer_types
                                              .pop_front()
                                              .unwrap();
                let write_object_type = write_object_type.as_str();

                let clocks: u8 = match write_object_type {
                    "REG" => 1,
                    "MEM" => memory_access_amount_clocks,
                    _ => panic!("No such operand type!")
                };

                writer.load_data(&clocks.clone());
                accessor.add_device_to_queue(2, clocks);
            }

            // if operand2(both 1 and 2 operands) passes value to calculator
            if operand2.is_done() && !vec_command_types.is_empty(){

                let command_type = vec_command_types.pop_front().unwrap();

                // load calculator
                let clocks = match command_type {
                    1 => 1,
                    2 => second_command_amount_clocks,
                    _ => panic!()
                };

                calculator.load_data(clocks);
            }

            // load 2 operand
            if operand1.is_done() && !vec_op2_types.is_empty(){

                let op2_type = vec_op2_types
                    .pop_front()
                    .unwrap();
                let op2_type = op2_type.as_str();

                let clocks: u8 = match op2_type {
                    "REG" => 1,
                    "MEM" => memory_access_amount_clocks,
                    _ => panic!()
                };
                operand2.load_data(clocks.clone());
                accessor.add_device_to_queue(1, clocks);
            }

            // load 1 operand
            if command_executor.is_done() && !vec_op1_types.is_empty(){

                let op1_type = vec_op1_types
                    .pop_front()
                    .unwrap();
                let op1_type = op1_type.as_str();

                let clocks: u8 = match op1_type {
                    "REG" => 1,
                    "MEM" => memory_access_amount_clocks,
                    _ => panic!()
                };
                operand1.load_data(clocks.clone());
                accessor.add_device_to_queue(0, clocks);
            }

            if vec_op1_types.len() <= 0{ input_available = false}
            // load command executor
            if input_available {
                command_executor.load_data(1);
            }
                    }


        println!("Clocks left:                             {} {} {} {} {}", command_executor.get_clocks(), operand1.get_clocks(),
                 operand2.get_clocks(), calculator.get_clocks(), writer.get_clocks());

        command_executor.clock_once();
        accessor.clock_once(&mut operand1, &mut operand2, &mut writer);
        calculator.clock_once();
        clocks_counter += 1;

        println!("Serial number of clock:                  {}", clocks_counter);
        println!("Status of conveyor stages after clock:   {} {} {} {} {}", command_executor.get_status(), operand1.get_status(),
                 operand2.get_status(), calculator.get_status(), writer.get_status());
    }

    println!("\nClocks end in {}", clocks_counter);
    let average_clocks: f64 =  (clocks_counter as f64) / (amount_commands as f64);
    println!("Average command execution time: {} clocks", average_clocks);
    let seq_proc_average_clocks: f64 = (seq_proc_clocks as f64) / (amount_commands as f64);
    println!("Average command execution time using sequential processing: {} clocks", seq_proc_average_clocks);
}


