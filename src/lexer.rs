use logos::Logos;

fn parse_char_literal(s: &str) -> u8 {
    let inner = &s[1..s.len() - 1];
    let mut chars = inner.chars();
    match chars.next() {
        Some('\\') => match chars.next() {
            Some('n') => b'\n',
            Some('r') => b'\r',
            Some('t') => b'\t',
            Some('\\') => b'\\',
            Some('"') => b'"',
            Some('\'') => b'\'',
            Some('0') => b'\0',
            Some('x') => {
                let hex: String = chars.by_ref().take(2).collect();
                u8::from_str_radix(&hex, 16).unwrap_or(0)
            }
            Some('u') => {
                assert_eq!(chars.next(), Some('{'));
                let scalar: String = chars.by_ref().take_while(|c| *c != '}').collect();
                let code = u32::from_str_radix(&scalar, 16).unwrap_or(0);
                if code <= 0x7F { code as u8 } else { 0 }
            }
            _ => 0,
        },
        Some(c) => c as u8,
        None => 0,
    }
}

fn parse_string_literal(s: &str) -> String {
    let inner = &s[1..s.len() - 1];
    let mut result = String::new();
    let mut chars = inner.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next().unwrap() {
                'n' => result.push('\n'),
                'r' => result.push('\r'),
                't' => result.push('\t'),
                '\\' => result.push('\\'),
                '"' => result.push('"'),
                '\'' => result.push('\''),
                '0' => result.push('\0'),
                'x' => {
                    let hex: String = chars.by_ref().take(2).collect();
                    let byte = u8::from_str_radix(&hex, 16).unwrap_or(0);
                    result.push(byte as char);
                }
                'u' => {
                    assert_eq!(chars.next().unwrap(), '{');
                    let scalar: String = chars.by_ref().take_while(|c| *c != '}').collect();
                    let code = u32::from_str_radix(&scalar, 16).unwrap_or(0);
                    result.push(std::char::from_u32(code).unwrap_or('\u{FFFD}'));
                }
                _ => {}
            }
        } else {
            result.push(c);
        }
    }
    result
}

fn parse_byte_string_literal(s: &str) -> Vec<u8> {
    let inner = &s[2..s.len() - 1];
    let mut result = Vec::new();
    let mut chars = inner.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next().unwrap() {
                'n' => result.push(b'\n'),
                'r' => result.push(b'\r'),
                't' => result.push(b'\t'),
                '\\' => result.push(b'\\'),
                '"' => result.push(b'"'),
                '\'' => result.push(b'\''),
                '0' => result.push(b'\0'),
                'x' => {
                    let hex: String = chars.by_ref().take(2).collect();
                    let byte = u8::from_str_radix(&hex, 16).unwrap_or(0);
                    result.push(byte);
                }
                _ => {}
            }
        } else {
            result.push(c as u8);
        }
    }
    result
}

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[regex("[ \t\n\x0C]+", logos::skip)]
    #[regex("//[^\n]*", logos::skip, allow_greedy = true)]
    #[regex("/\\*[^\\*]*\\*+(?:[^/\\*][^\\*]*\\*+)*/", logos::skip)]
    WhitespaceOrComment,

    #[regex("///[^\n]*", |lex| lex.slice()[3..].trim().to_string(), allow_greedy = true)]
    DocComment(String),

    #[regex("//![^\n]*", |lex| lex.slice()[3..].trim().to_string(), allow_greedy = true)]
    ModuleDocComment(String),

    #[token("def")]
    Def,
    #[token("set")]
    Set,
    #[token("type")]
    Type,
    #[token("with")]
    With,
    #[token("default")]
    Default,
    #[token("return")]
    Return,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("for")]
    For,
    #[token("in")]
    In,
    #[token("while")]
    While,
    #[token("loop")]
    Loop,
    #[token("leave")]
    Leave,
    #[token("continue")]
    Continue,
    #[token("comptime")]
    Comptime,
    #[token("import")]
    Import,
    #[token("from")]
    From,
    #[token("as")]
    As,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("auto")]
    Auto,
    #[token("and")]
    And,
    #[token("or")]
    Or,
    #[token("not")]
    Not,
    #[token("sizeof")]
    Sizeof,
    #[token("alignof")]
    Alignof,
    #[token("catch")]
    Catch,
    #[token("panic")]
    Panic,
    #[token("unsafe")]
    Unsafe,
    #[token("let")]
    Let,
    #[token("finally")]
    Finally,
    #[token("where")]
    Where,
    #[token("requires")]
    Requires,
    #[token("ensures")]
    Ensures,
    #[token("invariant")]
    Invariant,
    #[token("constraint")]
    Constraint,
    #[token("move")]
    Move,
    #[token("dyn")]
    Dyn,
    #[token("by")]
    By,
    #[token("copy")]
    Copy,
    #[token("ref")]
    Ref,
    #[token("mut")]
    Mut,
    #[token("wrap")]
    Wrap,
    #[token("saturate")]
    Saturate,
    #[token("trap")]
    Trap,
    #[token("Self")]
    SelfKw,
    #[token("no_default")]
    NoDefault,
    #[token("extern")]
    Extern,
    #[token("pub")]
    Pub,
    #[token("edition")]
    Edition,
    #[token("deprecated")]
    Deprecated,
    #[token("experimental")]
    Experimental,
    #[token("endian")]
    Endian,
    #[token("bit_order")]
    BitOrder,
    #[token("align")]
    Align,
    #[token("pad")]
    Pad,
    #[token("packed")]
    Packed,
    #[token("async")]
    Async,
    #[token("await")]
    Await,
    #[token("task")]
    Task,
    #[token("channel")]
    Channel,
    #[token("linear")]
    Linear,
    #[token("consume")]
    Consume,
    #[token("pure")]
    Pure,
    #[token("io")]
    Io,
    #[token("trusted")]
    Trusted,
    #[token("ghost")]
    Ghost,
    #[token("scope_cleanup")]
    ScopeCleanup,
    #[token("trigger")]
    Trigger,
    #[token("validate")]
    Validate,
    #[token("missing_match")]
    MissingMatch,
    #[token("apply_lemma")]
    ApplyLemma,
    #[token("exists")]
    Exists,
    #[token("forall")]
    Forall,
    #[token("on")]
    On,
    #[token("trait")]
    Trait,
    #[token("impl")]
    Impl,
    #[token("decreases")]
    Decreases,
    #[token("terminates")]
    Terminates,
    #[token("cfg")]
    Cfg,
    #[token("isolate")]
    Isolate,
    #[token("hint")]
    Hint,
    #[token("must_use")]
    MustUse,
    #[token("must_handle")]
    MustHandle,
    #[token("link_proof")]
    LinkProof,
    #[token("exhaustive")]
    Exhaustive,
    #[token("no_alloc_error")]
    NoAllocError,
    #[token("no_panic")]
    NoPanic,
    #[token("debug_info")]
    DebugInfo,
    #[token("old")]
    Old,
    #[token("audit_log")]
    AuditLog,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Ident(String),

    #[regex("[0-9][0-9_]*i[0-9]+", |lex| lex.slice().to_string())]
    IntSuffix(String),
    #[regex("[0-9][0-9_]*u[0-9]+", |lex| lex.slice().to_string())]
    UIntSuffix(String),
    #[regex("0x[0-9a-fA-F][0-9a-fA-F_]*i[0-9]+", |lex| lex.slice().to_string())]
    HexIntSuffix(String),
    #[regex("0x[0-9a-fA-F][0-9a-fA-F_]*u[0-9]+", |lex| lex.slice().to_string())]
    HexUIntSuffix(String),
    #[regex("0b[01][01_]*i[0-9]+", |lex| lex.slice().to_string())]
    BinIntSuffix(String),
    #[regex("0b[01][01_]*u[0-9]+", |lex| lex.slice().to_string())]
    BinUIntSuffix(String),

    #[regex("[0-9][0-9_]*\\.[0-9][0-9_]*([eE][+-]?[0-9][0-9_]*)?", |lex| lex.slice().replace('_', "").parse::<f64>().unwrap())]
    #[regex("[0-9][0-9_]*[eE][+-]?[0-9][0-9_]*", |lex| lex.slice().replace('_', "").parse::<f64>().unwrap())]
    FloatLiteral(f64),

    #[regex("[0-9][0-9_]*", |lex| lex.slice().replace('_', "").parse::<i64>().unwrap())]
    IntLiteral(i64),
    #[regex("0x[0-9a-fA-F][0-9a-fA-F_]*", |lex| i64::from_str_radix(&lex.slice()[2..].replace('_', ""), 16).unwrap())]
    HexLiteral(i64),
    #[regex("0b[01][01_]*", |lex| i64::from_str_radix(&lex.slice()[2..].replace('_', ""), 2).unwrap())]
    BinLiteral(i64),

    #[regex("'(([^'\\\\]|\\\\.)*)'", |lex| parse_char_literal(lex.slice()))]
    CharLiteral(u8),
    #[regex("b\"(\\\\.|[^\"\\\\])*\"", |lex| parse_byte_string_literal(lex.slice()))]
    ByteStringLiteral(Vec<u8>),
    #[regex("\"(\\\\.|[^\"\\\\])*\"", |lex| parse_string_literal(lex.slice()))]
    StringLiteral(String),

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,
    #[token("+%")]
    PlusWrap,
    #[token("-%")]
    MinusWrap,
    #[token("*%")]
    StarWrap,
    #[token("+?")]
    PlusSaturate,
    #[token("-?")]
    MinusSaturate,
    #[token("*?")]
    StarSaturate,
    #[token("+!")]
    PlusTrap,
    #[token("-!")]
    MinusTrap,
    #[token("*!")]
    StarTrap,
    #[token("&")]
    Ampersand,
    #[token("|")]
    Pipe,
    #[token("^")]
    Caret,
    #[token("<<")]
    Shl,
    #[token(">>")]
    Shr,
    #[token("~")]
    Tilde,
    #[token("==")]
    EqEq,
    #[token("!=")]
    Neq,
    #[token("<")]
    Lt,
    #[token(">")]
    Gt,
    #[token("<=")]
    Le,
    #[token(">=")]
    Ge,
    #[token("=")]
    Assign,
    #[token("+=")]
    PlusEq,
    #[token("-=")]
    MinusEq,
    #[token("*=")]
    StarEq,
    #[token("/=")]
    SlashEq,
    #[token("!")]
    Bang,
    #[token("?")]
    Question,
    #[token(".")]
    Dot,
    #[token("..")]
    DotDot,
    #[token("..=")]
    DotDotEq,
    #[token("::")]
    ColonColon,
    #[token(":")]
    Colon,
    #[token(";")]
    Semicolon,
    #[token(",")]
    Comma,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token("->")]
    Arrow,
    #[token("@")]
    At,
    #[token("=>")]
    FatArrow,
    #[token("...")]
    Ellipsis,

    Error,
}

#[cfg(test)]
mod tests {
    use super::*;
    use logos::Logos;

    fn check_tokens(source: &str, expected: Vec<Token>) {
        let mut lexer = Token::lexer(source);
        for exp in expected {
            loop {
                let tok = lexer
                    .next()
                    .expect("unexpected end of token stream")
                    .expect("lexer error");
                if tok == Token::WhitespaceOrComment {
                    continue;
                }
                assert_eq!(
                    tok,
                    exp,
                    "unexpected token at '{}'",
                    &source[lexer.span().start..lexer.span().end]
                );
                break;
            }
        }
        while let Some(t) = lexer.next() {
            match t {
                Ok(Token::WhitespaceOrComment) => {}
                Ok(_) => panic!("extra tokens after expected end"),
                Err(_) => panic!("extra error tokens after expected end"),
            }
        }
    }

    #[test]
    fn test_parse_char_literal_fn() {
        assert_eq!(parse_char_literal(r"'\n'"), b'\n');
        assert_eq!(parse_char_literal(r"'\x41'"), b'A');
        assert_eq!(parse_char_literal(r"'\u{7F}'"), 0x7F);
        assert_eq!(parse_char_literal(r"'a'"), b'a');
    }

    #[test]
    fn test_parse_string_literal_fn() {
        assert_eq!(parse_string_literal(r#""hello\nworld""#), "hello\nworld");
        assert_eq!(parse_string_literal(r#""\u{00E9}""#), "é");
        assert_eq!(parse_string_literal(r#""\x41\x42""#), "AB");
    }

    #[test]
    fn test_parse_byte_string_literal_fn() {
        assert_eq!(
            parse_byte_string_literal(r#"b"hello\nworld""#),
            b"hello\nworld".to_vec()
        );
        assert_eq!(
            parse_byte_string_literal(r#"b"\x00\xFF""#),
            vec![0x00, 0xFF]
        );
    }

    #[test]
    fn keywords_all() {
        check_tokens(
            "def set type with default return if else for in while loop leave continue comptime import from as true false auto",
            vec![
                Token::Def,
                Token::Set,
                Token::Type,
                Token::With,
                Token::Default,
                Token::Return,
                Token::If,
                Token::Else,
                Token::For,
                Token::In,
                Token::While,
                Token::Loop,
                Token::Leave,
                Token::Continue,
                Token::Comptime,
                Token::Import,
                Token::From,
                Token::As,
                Token::True,
                Token::False,
                Token::Auto,
            ],
        );
    }

    #[test]
    fn more_keywords() {
        check_tokens(
            "and or not sizeof alignof catch panic unsafe let finally where requires ensures invariant constraint move dyn by copy ref mut wrap saturate trap Self no_default extern pub edition deprecated experimental endian bit_order align pad packed async await task channel linear consume pure io trusted ghost scope_cleanup trigger validate missing_match apply_lemma exists forall on trait impl decreases terminates cfg isolate hint must_use must_handle link_proof exhaustive no_alloc_error no_panic debug_info old audit_log",
            vec![
                Token::And,
                Token::Or,
                Token::Not,
                Token::Sizeof,
                Token::Alignof,
                Token::Catch,
                Token::Panic,
                Token::Unsafe,
                Token::Let,
                Token::Finally,
                Token::Where,
                Token::Requires,
                Token::Ensures,
                Token::Invariant,
                Token::Constraint,
                Token::Move,
                Token::Dyn,
                Token::By,
                Token::Copy,
                Token::Ref,
                Token::Mut,
                Token::Wrap,
                Token::Saturate,
                Token::Trap,
                Token::SelfKw,
                Token::NoDefault,
                Token::Extern,
                Token::Pub,
                Token::Edition,
                Token::Deprecated,
                Token::Experimental,
                Token::Endian,
                Token::BitOrder,
                Token::Align,
                Token::Pad,
                Token::Packed,
                Token::Async,
                Token::Await,
                Token::Task,
                Token::Channel,
                Token::Linear,
                Token::Consume,
                Token::Pure,
                Token::Io,
                Token::Trusted,
                Token::Ghost,
                Token::ScopeCleanup,
                Token::Trigger,
                Token::Validate,
                Token::MissingMatch,
                Token::ApplyLemma,
                Token::Exists,
                Token::Forall,
                Token::On,
                Token::Trait,
                Token::Impl,
                Token::Decreases,
                Token::Terminates,
                Token::Cfg,
                Token::Isolate,
                Token::Hint,
                Token::MustUse,
                Token::MustHandle,
                Token::LinkProof,
                Token::Exhaustive,
                Token::NoAllocError,
                Token::NoPanic,
                Token::DebugInfo,
                Token::Old,
                Token::AuditLog,
            ],
        );
    }

    #[test]
    fn integer_literals() {
        check_tokens(
            "42 0xFF 0b1010 42i32 0xFFu8 0b1101u4",
            vec![
                Token::IntLiteral(42),
                Token::HexLiteral(255),
                Token::BinLiteral(10),
                Token::IntSuffix("42i32".into()),
                Token::HexUIntSuffix("0xFFu8".into()),
                Token::BinUIntSuffix("0b1101u4".into()),
            ],
        );
    }

    #[test]
    fn float_literals() {
        check_tokens(
            "3.14 2.5e-3 1_000.5 1e10",
            vec![
                Token::FloatLiteral(3.14),
                Token::FloatLiteral(0.0025),
                Token::FloatLiteral(1000.5),
                Token::FloatLiteral(1e10),
            ],
        );
    }

    #[test]
    fn char_literals() {
        check_tokens(
            r"'\n' '\t' '\\' '\'' '\x41' 'a'",
            vec![
                Token::CharLiteral(b'\n'),
                Token::CharLiteral(b'\t'),
                Token::CharLiteral(b'\\'),
                Token::CharLiteral(b'\''),
                Token::CharLiteral(b'A'),
                Token::CharLiteral(b'a'),
            ],
        );
    }

    #[test]
    fn string_literals() {
        let source = r#""hello" "\nworld\t" "\u{00E9}""#;
        let expected = vec![
            Token::StringLiteral("hello".into()),
            Token::StringLiteral("\nworld\t".into()),
            Token::StringLiteral("é".into()),
        ];
        check_tokens(source, expected);
    }

    #[test]
    fn byte_string_literals() {
        let source = r#"b"hello" b"\n\x00\xFF""#;
        let expected = vec![
            Token::ByteStringLiteral(b"hello".to_vec()),
            Token::ByteStringLiteral(vec![b'\n', 0x00, 0xFF]),
        ];
        check_tokens(source, expected);
    }

    #[test]
    fn operators_and_delimiters() {
        check_tokens(
            "+ - * / % +% -? *! & | ^ << >> ~ == != < > <= >= = += -= *= /= ! ? . .. ..= :: : ; , ( ) { } [ ] -> @ => ...",
            vec![
                Token::Plus,
                Token::Minus,
                Token::Star,
                Token::Slash,
                Token::Percent,
                Token::PlusWrap,
                Token::MinusSaturate,
                Token::StarTrap,
                Token::Ampersand,
                Token::Pipe,
                Token::Caret,
                Token::Shl,
                Token::Shr,
                Token::Tilde,
                Token::EqEq,
                Token::Neq,
                Token::Lt,
                Token::Gt,
                Token::Le,
                Token::Ge,
                Token::Assign,
                Token::PlusEq,
                Token::MinusEq,
                Token::StarEq,
                Token::SlashEq,
                Token::Bang,
                Token::Question,
                Token::Dot,
                Token::DotDot,
                Token::DotDotEq,
                Token::ColonColon,
                Token::Colon,
                Token::Semicolon,
                Token::Comma,
                Token::LParen,
                Token::RParen,
                Token::LBrace,
                Token::RBrace,
                Token::LBracket,
                Token::RBracket,
                Token::Arrow,
                Token::At,
                Token::FatArrow,
                Token::Ellipsis,
            ],
        );
    }

    #[test]
    fn comments_and_docs() {
        let source = "// line comment\n/// doc comment\n//! module doc\nx";
        let mut lex = Token::lexer(source);
        loop {
            let tok = lex.next().unwrap().unwrap();
            if tok == Token::WhitespaceOrComment {
                continue;
            }
            assert_eq!(tok, Token::DocComment("doc comment".into()));
            break;
        }
        loop {
            let tok = lex.next().unwrap().unwrap();
            if tok == Token::WhitespaceOrComment {
                continue;
            }
            assert_eq!(tok, Token::ModuleDocComment("module doc".into()));
            break;
        }
        loop {
            let tok = lex.next().unwrap().unwrap();
            if tok == Token::WhitespaceOrComment {
                continue;
            }
            assert_eq!(tok, Token::Ident("x".into()));
            break;
        }
        assert!(lex.next().is_none());
    }

    #[test]
    fn block_comment_skip() {
        let source = "a/* block comment */b";
        let mut lex = Token::lexer(source);
        let mut toks = Vec::new();
        while let Some(t) = lex.next() {
            match t {
                Ok(Token::WhitespaceOrComment) => {}
                Ok(other) => toks.push(other),
                Err(_) => panic!("lexer error"),
            }
        }
        assert_eq!(
            toks,
            vec![Token::Ident("a".into()), Token::Ident("b".into())]
        );
    }

    #[test]
    fn invalid_char_error() {
        let source = "` hello";
        let mut lex = Token::lexer(source);
        assert!(lex.next().unwrap().is_err());
        loop {
            let tok = lex.next().unwrap().unwrap();
            if tok == Token::WhitespaceOrComment {
                continue;
            }
            assert_eq!(tok, Token::Ident("hello".into()));
            break;
        }
    }

    #[test]
    fn empty_input() {
        let source = "";
        let mut lex = Token::lexer(source);
        assert!(lex.next().is_none());
    }

    #[test]
    fn ascii_identifier() {
        check_tokens(
            "hello world",
            vec![Token::Ident("hello".into()), Token::Ident("world".into())],
        );
    }

    #[test]
    fn comprehensive_small_example() {
        let source = r#"
edition = "2026";
type Age = exists n: UInt<8> invariant n >= 18;
def main() -> Result<(), AppError> {
    set x: Int<32> = 42 + 15;
    // line comment
    /// doc comment
    let y = "hello\nworld";
    return Ok(());
}
"#;
        let expected = vec![
            Token::Edition,
            Token::Assign,
            Token::StringLiteral("2026".into()),
            Token::Semicolon,
            Token::Type,
            Token::Ident("Age".into()),
            Token::Assign,
            Token::Exists,
            Token::Ident("n".into()),
            Token::Colon,
            Token::Ident("UInt".into()),
            Token::Lt,
            Token::IntLiteral(8),
            Token::Gt,
            Token::Invariant,
            Token::Ident("n".into()),
            Token::Ge,
            Token::IntLiteral(18),
            Token::Semicolon,
            Token::Def,
            Token::Ident("main".into()),
            Token::LParen,
            Token::RParen,
            Token::Arrow,
            Token::Ident("Result".into()),
            Token::Lt,
            Token::LParen,
            Token::RParen,
            Token::Comma,
            Token::Ident("AppError".into()),
            Token::Gt,
            Token::LBrace,
            Token::Set,
            Token::Ident("x".into()),
            Token::Colon,
            Token::Ident("Int".into()),
            Token::Lt,
            Token::IntLiteral(32),
            Token::Gt,
            Token::Assign,
            Token::IntLiteral(42),
            Token::Plus,
            Token::IntLiteral(15),
            Token::Semicolon,
            Token::DocComment("doc comment".into()),
            Token::Let,
            Token::Ident("y".into()),
            Token::Assign,
            Token::StringLiteral("hello\nworld".into()),
            Token::Semicolon,
            Token::Return,
            Token::Ident("Ok".into()),
            Token::LParen,
            Token::LParen,
            Token::RParen,
            Token::RParen,
            Token::Semicolon,
            Token::RBrace,
        ];
        check_tokens(source, expected);
    }
}
