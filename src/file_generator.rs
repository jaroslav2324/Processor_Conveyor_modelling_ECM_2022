use std::cmp::Ordering;
use std::fs::File;
use std::io::Write;
use rand::Rng;


pub fn generate_file(file_path: &str,
                     amount_commands_in_file: i32,
                     prob_reg_adresation: f64,
                     prob_1_type_command: f64) -> (){

    let prob_reg_adresation = prob_reg_adresation * 1000.0;
    let prob_reg_adresation = prob_reg_adresation as i64;

    let prob_1_type_command = prob_1_type_command * 1000.0;
    let prob_1_type_command = prob_1_type_command as i64;

    let mut file = File::create(file_path).expect("File was not found");
    let mut rng = rand::thread_rng();

    for _ in 0..amount_commands_in_file {

        let p_1_command = (rng.gen::<f64>() * 1000.0) as i64;

        let mut msg: &str;
        match p_1_command.cmp(&prob_1_type_command) {
            Ordering::Less => msg = "1 ",
            Ordering::Equal | Ordering::Greater => msg = "2 "
        }

        file.write(msg.as_ref()).expect("Error during writing to file");

        let op1_reg_prob = (rng.gen::<f64>() * 1000.0) as i64;
        match op1_reg_prob.cmp(&prob_reg_adresation) {
            Ordering::Less => msg = "REG ",
            Ordering::Equal | Ordering::Greater => msg ="MEM "
        }

        file.write(msg.as_ref()).expect("Error during writing to file");

        let op2_reg_prob = (rng.gen::<f64>() * 1000.0) as i64;
        match op2_reg_prob.cmp(&prob_reg_adresation) {
            Ordering::Less => msg ="REG\n",
            Ordering::Equal | Ordering::Greater => msg ="MEM\n"
        }

        file.write(msg.as_ref()).expect("Error during writing to file");
    }
}
