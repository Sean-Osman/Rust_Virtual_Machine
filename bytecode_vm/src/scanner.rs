    
    
    pub enum TokenType{
        TokenLeftParen, TokenRightParen,
        TokenLeftBrace, TokenRightBrace,
        TokenComma,
        TokenDot,
        TokenSemicolon,
        TokenMinus, TokenPlus,
        TokenSlash, TokenStar,
        TokenNot, TokenNotEqual,
        TokenEqual, TokenEqualEqual,
        TokenLess, TokenLessEqual,
        TokenGreater, TokenGreaterEqual,
        TokenIdentifier,
        TokenString,
        TokenNumber,
        TokenTrue, TokenFalse,
        TokenAnd, TokenOr,
        TokenIf, TokenElse,
        TokenClass, TokenSuper, TokenThis,
        TokenFun,
        TokenVar,
        TokenReturn,
        TokenFor,
        TokenWhile,
        TokenNil,
        TokenPrint,
        TokenError,
        TokenEof,
    }


    pub struct token_type{
        token_type: TokenType,
        values: Vec<u8>,
        lines: Vec<u32>
    }


    pub struct Scanner{
        source_code: Vec<u8>,
        start_index: usize,
        current_index: usize,
        line_number: u32
    }

    impl Scanner{

        fn init_scanner(sc: &str){
            source_code = sc.as_bytes();

        }
    }

    fn is_digit(character: char) -> bool{
        chararacter.is_digit();
    }

    fn is_alpha(character: char) -> bool{
        character.is_alpha();
    }

    fn is_at_end(&self) -> bool{
        if source_code.len() <= 0{
            return true
        }
        return false
    }

    fn peek(&self) -> usize{
        if self.source_code[self.current_index] == usize{
            return self.source_code[self.current_index]

        }else{
            return '\0';
        }
    }

    fn peek_next() -> usize{
        if source_code[current_index+1] == usize{
            return source_code[current_index+1]
        }else{
            return '\0';
        }
    }

    fn advance(&self) -> usize{
       let temp: usize = self.source_code[self.current_index];
       self.current_index + 1;
       return temp
    }

    fn skip_next_character(){
        self.current_index + 1;
    }

    fn skip_whitespace(){
        match self.source_code[self.current_index]{
            " " => {
                skip_next_character();
            }
            "\n" => {
                skip_next_character();
            }

            "\t" => {
                skip_next_character();
            }

            "\r" =>{
                skip_next_character();
            }
            
        }
    }

    fn skip_comment(){
        // like skip_whitespace but checks for // 

        if self.source_code[self.current_index] == "//"{
            while self.source_code[self.current_index] != "\r"{
                skip_next_character();
            }
        }
    }

    fn match_next(character: char)-> bool{
        if character == self.source_code[self.current_index]{
            self.current_index + 1;
            true
        }
        if is_at_end(){
            false
        }
        false
    }

    fn get_identifier(){

    }

    fn make_token(token: TokenType){

    }
