//Sean, Nando, Bill

use bytecode_vm::{Chunk, OpCode, VirtualMachine}; //this imports all fns and structs

fn main() {
    println!("Hello, world!");

    println!("Creating a Bytecode chunk ...");

    let mut chunk: Chunk = Chunk::init_chunk();

    let cons: u8 = chunk.add_constant(42);
    chunk.write_to_chunk(OpCode::OpToBit(OpCode::OpConstant), 123);
    chunk.write_to_chunk(cons, 123);

    let cons: u8 = chunk.add_constant(18);
    chunk.write_to_chunk(OpCode::OpToBit(OpCode::OpConstant), 124);
    chunk.write_to_chunk(cons, 124);

    chunk.write_to_chunk(OpCode::OpToBit(OpCode::OpReturn), 125);
    chunk.disassemble("Test chunk");

    let mut vm: VirtualMachine = VirtualMachine::init_machine(chunk);
    vm.interpret(chunk);
}
