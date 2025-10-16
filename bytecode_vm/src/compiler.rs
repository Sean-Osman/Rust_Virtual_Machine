 
 
struct Parser{
    current: Token,
    previous: Token,
    had_error: bool,
    panic_mode: bool,
    parse_rules: HashMap<TokenType, ParseRule>,
}

impl Parser{


    pub type ParseFn = Option;

    struct ParseRule{
        prefix: ParseFn,
        infix: ParseFn,
        precedence: Precedence,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    enum Precedence{
        PrecNone       = 0,
        PrecAssignment = 1,
        PrecOr         = 2,
        PrecAnd        = 3,
        PrecEquality   = 4,
        PrecComparison = 5,
        PrecTerm       = 6,
        PrecFactor     = 7,
        PrecUnary      = 8,
        PrecCall       = 9,
        PrecPrimary    = 10,
        fn from_integer(int: u32) -> u32{
            
        }
    }

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
    }
#[derive(Debug, Clone)] 
struct Compiler {
    Chunk: Chunk,
    Scanner: Scanner,
    Parser: Parser,
}



fn pratt_table() -> HashMap<TokenType, ParseRule>{
    (TokenLeftParen, {grouping, None, PrecNone})
    (TokenRightParen, {None, None, PrecNone})
    (TokenLeftBrace, {None, None, PrecNone})
    (TokenRightBrace, {None, None, PrecNone})
    (TokenComma, {None, None, PrecNone})
    (TokenDot, {None, None, PrecNone})
    (TokenMinus, {unary, binary, PrecTerm})
    (TokenPlus, {None, binary, PrecTerm})
    (TokenSemicolon, {None, None, PrecTerm})
    (TokenSlash, {None, binary, PrecFactor})
    (TokenStar, {None, binary, PrecFactor})
    (TokenNot, {None, None, PrecFactor})
    (TokenNotEqual, {None, None, PrecFactor})
    (TokenEqual, {None, None, PrecFactor})
    (TokenEqualEqual, {None, None, PrecFactor})
    (TokenGreater, {None, None, PrecFactor})
    (TokenGreaterEqual, {None, None, PrecFactor})
    (TokenLess, {None, None, PrecFactor})
    (TokenLessEqual, {None, None, PrecFactor})
    (TokenIdentifier, {None, None, PrecFactor})
    (TokenString, {None, None, PrecFactor})
    (TokenNumber, {number, None, PrecFactor})
    (TokenAnd, {None, None, PrecFactor})
    (TokenClass, {None, None, PrecFactor})
    (TokenElse, {None, None, PrecFactor})
    (TokenFalse, {None, None, PrecFactor})
    (TokenFor, {None, None, PrecFactor})
    (TokenFun, {None, None, PrecFactor})
    (TokenIf, {None, None, PrecFactor})
    (TokenNil, {None, None, PrecFactor})
    (TokenOr, {None, None, PrecFactor})
    (TokenPrint, {None, None, PrecFactor})
    (TokenReturn, {None, None, PrecFactor})
    (TokenSuper, {None, None, PrecFactor})
    (TokenThis, {None, None, PrecFactor})
    (TokenTrue, {None, None, PrecFactor})
    (TokenVar, {None, None, PrecFactor})
    (TokenWhile, {None, None, PrecFactor})
    (TokenError, {None, None, PrecFactor})
    (TokenEof, {None, None, PrecFactor})
}

impl Compiler{


    fn get_chunk() -> Chunk{
        return Chunk.clone()
    }
}
    fn compile(source_code: &str) -> bool{
        Scanner::init_scanner();
        Compiler::advance();
        Compiler::expression();
        Compiler::consume();
        Compiler::end_compiler();
        return had_error;
    }

    fn advance(){
        Parser.previous = parser.current;
        while parser.current == TokenType:TokenError{
            parser.current = Scanner::scan_token();
            
        }
        error_at_current();
    }


    fn expression(){
        self.parse_precedence(Precedence::PrecAssignment);
    }

    fn binary(){
        parser.previous.token_type
    }








    pub fn error_at_current(self: &mut Compiler, message: &str) {
        self.error_at(self.parser.current.clone(), message);
    }

    pub fn error(self: &mut Compiler, message: &str) {
        self.error_at(self.parser.previous.clone(), message);
    }

    pub fn error_at(self: &mut Compiler, token: Token, message: &str) {
        if self.parser.panic_mode {
            return;
        }
        self.parser.panic_mode = true;
        println!("[line {}] Error", token.line);
        if token.token_type == TokenType::TokenEof {
            println!(" at end");
        }
        else {
            if token.token_type == TokenType::TokenError {
                // Nothing
            }
            else {
                println!(" at {} {:?}", token.length, token);
            }
        }
        println!(": {}\n", message);
        self.parser.had_error = true;
    }