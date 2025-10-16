 pub fn init_parser() -> Self {
            Self {       
                current: Token {token_type: TokenType::TokenError,
                                value: Vec::new(),
                                length: 0,
                                line: 0,
                               },
                previous: Token {token_type: TokenType::TokenError,
                                 value: Vec::new(),
                                 length: 0,
                                 line: 0,
                               },
                had_error: false,
                panic_mode: false,
                parse_rules: Parser::pratt_table(),
            }
        }
#[derive(Debug, Clone)] 
struct Compiler {
    Chunk: Chunk,
    Scanner: Scanner,
    Parser: Parser,
}


impl Compiler{


    fn get_chunk() -> Chunk{
        return Chunk.clone()
    }
}