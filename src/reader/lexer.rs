
static mut INIT: bool = false;


static token_types: Vec<TokenType> = init_token_types();

const fn init_token_types() -> Vec<TokenType> {
    Vec::new()
}

pub fn initialize() {

}

pub fn lex(input: String) {
}
