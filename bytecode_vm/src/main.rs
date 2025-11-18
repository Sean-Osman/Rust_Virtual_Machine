use std::env;
use std::io;
use std::io::Write;
use std::fs;
use bytecode_vm::VirtualMachine; // only what's needed

fn main() {
    // Initialize the Bytecode Stack-Based Virtual Machine
    let mut vm: VirtualMachine = VirtualMachine::init_machine();

    // Get the command line arguments
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    if args.len() == 1 {
        read_eval_print_loop(&mut vm);
    } else if args.len() == 2 {
        run_file(&mut vm, &args[1]);
    } else {
        println!("Usage: ");
        println!("    cargo run");
        println!("        to use the lox read-eval-print-loop");
        println!("    cargo run -- lox_program_file_name");
        println!("        to execute a lox program file");
    }
}

fn read_eval_print_loop(vm: &mut VirtualMachine) {
    loop {
        // Print the read prompt
        print!("> ");
        io::stdout().flush().expect("flush failed");

        // Get a line from the user
        let mut lox_code = String::new();
        io::stdin().read_line(&mut lox_code).expect("Failed to read line");

        // Scanner path: scan and print tokens
        vm.interpret_source(&lox_code);
    }
}

fn run_file(vm: &mut VirtualMachine, infile: &str) {
    let lox_code = fs::read_to_string(infile).expect("Unable to read the file");
    // Scanner path: scan and print tokens (use the VM.compile token printer)
    vm.compile(&lox_code);
}
