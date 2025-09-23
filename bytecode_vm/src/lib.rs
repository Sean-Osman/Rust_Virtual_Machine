

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

    code: Vec<u8>,
    lines: Vec<u32>,
    values: Vec<Value>

}

impl Chunk{
    fn init_chunk(self:Chunk)-> Chunk{

        return Chunk { code:Vec::new(),lines:Vec::new(),values:Vec::new() }

    }

    fn add_constant(&self, num:u32){

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


    fn OpConstant(){
        
    }
 }