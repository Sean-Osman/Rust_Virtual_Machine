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

    chunk.write_to_chunk(OpCode::OpToBit(OpCode::OpAdd), 125);
    

    let cons: u8 = chunk.add_constant(32);
    chunk.write_to_chunk(OpCode::OpToBit(OpCode::OpConstant), 126);
    chunk.write_to_chunk(cons, 126);

    chunk.write_to_chunk(OpCode::OpToBit(OpCode::OpDivide), 127);
    

    let cons: u8 = chunk.add_constant(37);
    chunk.write_to_chunk(OpCode::OpToBit(OpCode::OpConstant), 128);
    chunk.write_to_chunk(cons, 128);

    chunk.write_to_chunk(OpCode::OpToBit(OpCode::OpMultiply), 129);
    

    let cons: u8 = chunk.add_constant(85);
    chunk.write_to_chunk(OpCode::OpToBit(OpCode::OpConstant), 130);
    chunk.write_to_chunk(cons, 130);

    chunk.write_to_chunk(OpCode::OpToBit(OpCode::OpModulo), 131);
    

    let cons: u8 = chunk.add_constant(12);
    chunk.write_to_chunk(OpCode::OpToBit(OpCode::OpConstant), 132);
    chunk.write_to_chunk(cons, 132);

    chunk.write_to_chunk(OpCode::OpToBit(OpCode::OpNegate), 133);
    

    chunk.write_to_chunk(OpCode::OpToBit(OpCode::OpReturn), 134);
    chunk.disassemble("Test chunk");

    let mut vm: VirtualMachine = VirtualMachine::init_machine();
    vm.interpret(chunk);

    //println!("Final stack: {:?}", vm.stack);
}
