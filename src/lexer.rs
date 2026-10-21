use std::fmt;

//Tipos de Tokens soportados por brust
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Palabras clave (Keywords)
    Let,
    Fn,
    If,
    Else,
    While,
    Return,
    True,
    False,

    // Identificadores y Literales
    Identifier(String),
    Number(i64),
    StringLiteral(String),

    // Operadores
    Assign,       // =
    Equal,        // ==
    Plus,         // +
    Minus,        // -
    Star,         // *
    Slash,        // /
    Greater,      // >
    Less,         // <

    // Delimitadores
    LParen,       // (
    RParen,       // )
    LBrace,       // {
    RBrace,       // }
    Comma,        // ,
    Semicolon,    // ;

    // Control
    Eof,
}

// Estructura que representa un Token con su ubicación
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize, column: usize) -> Self {
        Self { token_type, line, column }
    }
}

impl fmt::Display for Token {
    // Para imprimir el token
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[L:{}:C:{}] {:?}", self.line, self.column, self.token_type)
    }
}





// Implementación del Lexer
pub struct Lexer {
    source: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            source: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    // Devuelve el carácter actual sin avanzar
    fn peek(&self) -> Option<char> {
        if self.position < self.source.len() {
            Some(self.source[self.position])
        } else {
            None
        }
    }

    // Devuelve el siguiente carácter sin avanzar
    fn peek_next(&self) -> Option<char> {
        if self.position + 1 < self.source.len() {
            Some(self.source[self.position + 1])
        } else {
            None
        }
    }

    // Consume el carácter actual y avanza el puntero
    fn advance(&mut self) -> Option<char> {
        if self.position >= self.source.len() {
            return None;
        }

        let ch = self.source[self.position];
        self.position += 1;

        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        Some(ch)
    }

    // Genera la lista completa de tokens
    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while let Some(&ch) = self.source.get(self.position) {
            let start_col = self.column;
            let start_line = self.line;

            match ch {
                // Saltar espacios en blanco y saltos de línea
                ' ' | '\r' | '\t' | '\n' => {
                    self.advance();
                }

                // Manejo de comentarios de una línea: // ...
                '/' if self.peek_next() == Some('/') => {
                    while let Some(c) = self.peek() {
                        if c == '\n' { break; }
                        self.advance();
                    }
                }

                // Delimitadores
                '(' => { self.advance(); tokens.push(Token::new(TokenType::LParen, start_line, start_col)); }
                ')' => { self.advance(); tokens.push(Token::new(TokenType::RParen, start_line, start_col)); }
                '{' => { self.advance(); tokens.push(Token::new(TokenType::LBrace, start_line, start_col)); }
                '}' => { self.advance(); tokens.push(Token::new(TokenType::RBrace, start_line, start_col)); }
                ',' => { self.advance(); tokens.push(Token::new(TokenType::Comma, start_line, start_col)); }
                ';' => { self.advance(); tokens.push(Token::new(TokenType::Semicolon, start_line, start_col)); }

                // Operadores de un o dos caracteres
                '+' => { self.advance(); tokens.push(Token::new(TokenType::Plus, start_line, start_col)); }
                '-' => { self.advance(); tokens.push(Token::new(TokenType::Minus, start_line, start_col)); }
                '*' => { self.advance(); tokens.push(Token::new(TokenType::Star, start_line, start_col)); }
                '/' => { self.advance(); tokens.push(Token::new(TokenType::Slash, start_line, start_col)); }
                '=' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        tokens.push(Token::new(TokenType::Equal, start_line, start_col));
                    } else {
                        tokens.push(Token::new(TokenType::Assign, start_line, start_col));
                    }
                }
                '>' => { self.advance(); tokens.push(Token::new(TokenType::Greater, start_line, start_col)); }
                '<' => { self.advance(); tokens.push(Token::new(TokenType::Less, start_line, start_col)); }

                // Cadenas de texto ("...")
                '"' => {
                    self.advance(); // Consumir comilla inicial
                    let mut string_val = String::new();

                    while let Some(c) = self.peek() {
                        if c == '"' { break; }
                        string_val.push(c);
                        self.advance();
                    }

                    if self.peek() != Some('"') {
                        return Err(format!("Error Léxico [L:{}:C{}]: Cadena de texto no cerrada", start_line, start_col));
                    }

                    self.advance(); // Consumir comilla final
                    tokens.push(Token::new(TokenType::StringLiteral(string_val), start_line, start_col));
                }

                // Números enteros
                c if c.is_ascii_digit() => {
                    let mut num_str = String::new();
                    while let Some(digit) = self.peek() {
                        if digit.is_ascii_digit() {
                            num_str.push(digit);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    let value = num_str.parse::<i64>().unwrap();
                    tokens.push(Token::new(TokenType::Number(value), start_line, start_col));
                }

                // Identificadores y Palabras Clave
                c if c.is_alphabetic() || c == '_' => {
                    let mut ident = String::new();
                    while let Some(letter) = self.peek() {
                        if letter.is_alphanumeric() || letter == '_' {
                            ident.push(letter);
                            self.advance();
                        } else {
                            break;
                        }
                    }

                    let token_type = match ident.as_str() {
                        "let"    => TokenType::Let,
                        "fn"     => TokenType::Fn,
                        "if"     => TokenType::If,
                        "else"   => TokenType::Else,
                        "while"  => TokenType::While,
                        "return" => TokenType::Return,
                        "true"   => TokenType::True,
                        "false"  => TokenType::False,
                        _        => TokenType::Identifier(ident),
                    };

                    tokens.push(Token::new(token_type, start_line, start_col));
                }

                // Carácter no reconocido
                _ => {
                    return Err(format!(
                        "Error Léxico [L:{}:C{}]: Carácter no reconocido '{}'",
                        self.line, self.column, ch
                    ));
                }
            }
        }

        tokens.push(Token::new(TokenType::Eof, self.line, self.column));
        Ok(tokens)
    }
}

pub fn main() {
    let code = r#"
        // Ejemplo en brust
        let mensaje = "Calculando...";
        let contador = 0;

        fn es_mayor(limite) {
            if limite > 10 {
                return true;
            } else {
                return false;
            }
        }

        while contador < 5 {
            contador = contador + 1;
        }
    "#;

    let mut lexer = Lexer::new(code);
    match lexer.tokenize() {
        Ok(tokens) => {
            println!("--- Tokens de brust Generados ---");
            for token in tokens {
                println!("{}", token);
            }
        }
        Err(err) => eprintln!("{}", err),
    }
}