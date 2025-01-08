
use TokenType::*;
enum TokenType {
    int, // for numbers with no decimal.
    fpoint, // for numbers with decimal point.
    op, // for general operators, e.g. any elementary function, +,-,*,/,^,int,sin etc.
    uscore, // for underscores; need special treatment bcos either variable names or parameters
            // (e.g. int_a^b)
    bracket(bool),
}


struct TokenMatch {
    regex_match: String,
    type_name: String,
}

const const_tokens[Token;
