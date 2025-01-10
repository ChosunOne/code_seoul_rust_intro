use crate::token::{Token, TokenType};

#[derive(Debug, Clone)]
pub struct Lexer {
    pub line: usize,
    source: String,
    current_index: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            line: 1,
            source,
            current_index: 0,
        }
    }

    /// Look at the next character of our source string
    fn iter_peek(&mut self) -> Option<char> {
        self.source[self.current_index..].chars().next()
    }

    /// Consume the next character of our source string
    fn iter_next(&mut self) -> Option<char> {
        self.current_index += 1;
        self.source[self.current_index - 1..].chars().next()
    }

    /// Consume the next character of our source string only if it matches
    /// the supplied character
    fn next_if_eq(&mut self, c: char) -> Option<char> {
        if self.iter_peek()? == c {
            return self.iter_next();
        }
        None
    }

    /// Look at the character after the next character to be consumed
    fn peek_next(&mut self) -> Option<char> {
        self.iter_peek()?;
        let mut next_iter = self.source[self.current_index..].chars().peekable();
        next_iter.next();
        let next_c = next_iter.peek()?;
        Some(*next_c)
    }

    /// Consume characters until some non-whitespace character is found
    fn skip_whitespace(&mut self) {
        loop {
            match self.iter_peek() {
                None => return,
                Some(' ' | '\t' | '\r') => {
                    self.iter_next();
                }
                Some('\n') => {
                    self.line += 1;
                    self.iter_next();
                }
                Some('/') => {
                    if self.peek_next() == Some('/') {
                        while self.iter_peek() != Some('\n') && !self.is_at_end() {
                            self.iter_next();
                        }
                    } else {
                        return;
                    }
                }
                _ => return,
            }
        }
    }

    /// Check if the iterator of the source string has reached the end
    fn is_at_end(&mut self) -> bool {
        self.iter_peek().is_none()
    }

    /// Return an identifier or keyword if present
    fn identifier_or_keyword(&mut self) -> Option<Token> {
        todo!()
    }

    /// Return a number token if present
    fn number(&mut self) -> Option<Token> {
        todo!()
    }

    /// Return a string token if present
    fn string(&mut self) -> Option<Token> {
        todo!()
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_scans_end_of_file() {
        let source = "";
        let mut lexer = Lexer::new(source.into());
        let token = lexer.next().unwrap();
        assert_eq!(
            token,
            Token {
                kind: TokenType::Eof,
                line: 1,
                lexeme: "".into(),
            },
        );
    }

    #[test]
    fn it_skips_whitespace_and_comments() {
        let source = "    \t  \n // \r\n \t   ";
        let mut lexer = Lexer::new(source.into());
        let token = lexer.next().unwrap();
        assert_eq!(
            token,
            Token {
                kind: TokenType::Eof,
                line: 3,
                lexeme: "".into(),
            },
        );
    }

    #[test]
    fn it_scans_an_identifier() {
        let source = "identifier\nidentifier1234\nidentifier_1234";
        let mut lexer = Lexer::new(source.into());
        let token = lexer.next().unwrap();
        assert_eq!(
            token,
            Token {
                kind: TokenType::Identifier,
                line: 1,
                lexeme: "identifier".into()
            }
        );
        let token = lexer.next().unwrap();
        assert_eq!(
            token,
            Token {
                kind: TokenType::Identifier,
                line: 2,
                lexeme: "identifier1234".into()
            }
        );
        let token = lexer.next().unwrap();
        assert_eq!(
            token,
            Token {
                kind: TokenType::Identifier,
                line: 3,
                lexeme: "identifier_1234".into()
            }
        );
    }

    #[test]
    fn it_scans_a_number() {
        let source = "12345.6789\n54321";
        let mut lexer = Lexer::new(source.into());
        let token = lexer.next().unwrap();
        assert_eq!(
            token,
            Token {
                kind: TokenType::Number,
                line: 1,
                lexeme: "12345.6789".into()
            }
        );

        let token = lexer.next().unwrap();
        assert_eq!(
            token,
            Token {
                kind: TokenType::Number,
                line: 2,
                lexeme: "54321".into()
            }
        );
    }

    #[test]
    fn it_scans_single_characters() {
        let source = "(){};,.-+/*! = < > $";
        let mut lexer = Lexer::new(source.into());
        let expected_tokens = vec![
            Token {
                kind: TokenType::LeftParen,
                lexeme: "(".into(),
                line: 1,
            },
            Token {
                kind: TokenType::RightParen,
                lexeme: ")".into(),
                line: 1,
            },
            Token {
                kind: TokenType::LeftBrace,
                lexeme: "{".into(),
                line: 1,
            },
            Token {
                kind: TokenType::RightBrace,
                lexeme: "}".into(),
                line: 1,
            },
            Token {
                kind: TokenType::Semicolon,
                lexeme: ";".into(),
                line: 1,
            },
            Token {
                kind: TokenType::Comma,
                lexeme: ",".into(),
                line: 1,
            },
            Token {
                kind: TokenType::Dot,
                lexeme: ".".into(),
                line: 1,
            },
            Token {
                kind: TokenType::Minus,
                lexeme: "-".into(),
                line: 1,
            },
            Token {
                kind: TokenType::Plus,
                lexeme: "+".into(),
                line: 1,
            },
            Token {
                kind: TokenType::Slash,
                lexeme: "/".into(),
                line: 1,
            },
            Token {
                kind: TokenType::Star,
                lexeme: "*".into(),
                line: 1,
            },
            Token {
                kind: TokenType::Bang,
                lexeme: "!".into(),
                line: 1,
            },
            Token {
                kind: TokenType::Equal,
                lexeme: "=".into(),
                line: 1,
            },
            Token {
                kind: TokenType::Less,
                lexeme: "<".into(),
                line: 1,
            },
            Token {
                kind: TokenType::Greater,
                lexeme: ">".into(),
                line: 1,
            },
            Token {
                kind: TokenType::Error,
                lexeme: "Unexpected character '$'".into(),
                line: 1,
            },
        ];
        for expected_token in expected_tokens {
            let token = lexer.next().unwrap();
            assert_eq!(token, expected_token);
        }
    }

    #[test]
    fn it_scans_double_tokens() {
        let source = "== <= >= !=";
        let mut lexer = Lexer::new(source.into());
        let expected_tokens = vec![
            Token {
                kind: TokenType::EqualEqual,
                lexeme: "==".into(),
                line: 1,
            },
            Token {
                kind: TokenType::LessEqual,
                lexeme: "<=".into(),
                line: 1,
            },
            Token {
                kind: TokenType::GreaterEqual,
                lexeme: ">=".into(),
                line: 1,
            },
            Token {
                kind: TokenType::BangEqual,
                lexeme: "!=".into(),
                line: 1,
            },
        ];

        for expected_token in expected_tokens {
            let token = lexer.next().unwrap();
            assert_eq!(token, expected_token);
        }
    }

    #[test]
    fn it_scans_a_string() {
        let source = "\"hello world\"";
        let mut lexer = Lexer::new(source.into());
        let token = lexer.next().unwrap();
        assert_eq!(
            token,
            Token {
                kind: TokenType::String,
                lexeme: "hello world".into(),
                line: 1
            }
        );
    }

    #[test]
    fn it_reports_unterminated_string() {
        let source = "\"hello world";
        let mut lexer = Lexer::new(source.into());
        let token = lexer.next().unwrap();
        assert_eq!(
            token,
            Token {
                kind: TokenType::Error,
                lexeme: "Unterminated string.".into(),
                line: 1
            }
        );
    }

    #[test]
    fn it_scans_a_boolean() {
        let source = "true false";
        let mut lexer = Lexer::new(source.into());
        let token = lexer.next().unwrap();
        assert_eq!(
            token,
            Token {
                kind: TokenType::True,
                lexeme: "true".into(),
                line: 1
            }
        );
        let token = lexer.next().unwrap();
        assert_eq!(
            token,
            Token {
                kind: TokenType::False,
                lexeme: "false".into(),
                line: 1
            }
        );
    }

    #[test]
    fn it_scans_a_nil() {
        let source = "nil";
        let mut lexer = Lexer::new(source.into());
        let token = lexer.next().unwrap();
        assert_eq!(
            token,
            Token {
                kind: TokenType::Nil,
                lexeme: "nil".into(),
                line: 1
            }
        );
    }

    #[test]
    fn it_scans_a_keyword() {
        let source = "and class else for fun if or print return super this var while";
        let mut lexer = Lexer::new(source.into());
        let expected_tokens = [
            Token {
                kind: TokenType::And,
                lexeme: "and".into(),
                line: 1,
            },
            Token {
                kind: TokenType::Class,
                lexeme: "class".into(),
                line: 1,
            },
            Token {
                kind: TokenType::Else,
                lexeme: "else".into(),
                line: 1,
            },
            Token {
                kind: TokenType::For,
                lexeme: "for".into(),
                line: 1,
            },
            Token {
                kind: TokenType::Fun,
                lexeme: "fun".into(),
                line: 1,
            },
            Token {
                kind: TokenType::If,
                lexeme: "if".into(),
                line: 1,
            },
            Token {
                kind: TokenType::Or,
                lexeme: "or".into(),
                line: 1,
            },
            Token {
                kind: TokenType::Print,
                lexeme: "print".into(),
                line: 1,
            },
            Token {
                kind: TokenType::Return,
                lexeme: "return".into(),
                line: 1,
            },
            Token {
                kind: TokenType::Super,
                lexeme: "super".into(),
                line: 1,
            },
            Token {
                kind: TokenType::This,
                lexeme: "this".into(),
                line: 1,
            },
            Token {
                kind: TokenType::Var,
                lexeme: "var".into(),
                line: 1,
            },
            Token {
                kind: TokenType::While,
                lexeme: "while".into(),
                line: 1,
            },
        ];

        for token in expected_tokens {
            assert_eq!(lexer.next().unwrap(), token);
        }
    }
}
