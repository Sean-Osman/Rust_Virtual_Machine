

type Value = u8;


pub enum OpCode{

    OpReturn,
    OpConstant,
    OpNegate,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide
}

#[derive(Debug)]
struct Chunk{

    code: Vec<u8>, //what opcode
    lines: Vec<u32>, //specific line location
    values: Vec<Value> // the actual number

}

impl Chunk{
    fn init_chunk(self:Chunk)-> Chunk{

        return Chunk { code:Vec::new(),lines:Vec::new(),values:Vec::new() }

    }

    

    fn write_to_chunk(&mut self, byte:u8, linenum:u32){
        self.code.push(byte);
        self.lines.push(linenum);
    }

    fn disassemble(&self, name: &str){
        println!("{} = {:?}", name, self.code);
        // for i in 0..self.code.len(){
        //     println!("{:?}", self.code);
        // }


    }
    fn disassemble_instruction(&self, offset: usize){
        println!("{:?}", self.code.get(offset..));
    }


}
 impl OpCode{


    fn OpToBit(name: OpCode) -> u8{

        match(name){
            OpCode::OpReturn => 0,
            OpCode::OpConstant => 1,
            OpCode::OpNegate => 2,
            OpCode::OpAdd => 3,
            OpCode::OpSubtract => 4,
            OpCode::OpMultiply => 5,
            OpCode::OpDivide => 6
        }
    }

    fn BitToOp(num: u8) -> OpCode{

        match(num){
            0 => OpCode::OpReturn,
            1 => OpCode::OpConstant,
            2 => OpCode::OpNegate,
            3 => OpCode::OpAdd,
            4 => OpCode::OpSubtract,
            5 => OpCode::OpMultiply,
            6 => OpCode::OpDivide,
            _ => unreachable!()
        }

    }

    fn add_constant(num: u8){

    }
 }