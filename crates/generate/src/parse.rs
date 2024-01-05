use eyre::{bail, ContextCompat, OptionExt, Result};
use logos::Logos;

#[derive(Debug)]
pub(crate) enum Stmt {
    FnDef,
    Assign {
        lhs: Expr,
        rhs: Expr,
    },
    If {
        cond: Expr,
        then: Expr,
        els: Option<Expr>,
    },
}

#[derive(Debug)]
pub(crate) enum Expr {
    Int(u64),
    Ident(String),
    Index {
        lhs: Box<Expr>,
        idx: Box<Expr>,
    },
    Range {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Call {
        function: String,
        args: Vec<Expr>,
    },
    BinaryOp {
        op: BinaryOpKind,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
}

#[derive(Debug)]
pub(crate) enum BinaryOpKind {}

pub(crate) fn parse_operation(op: &str) -> Result<Vec<Stmt>> {
    let tokens = Token::lexer(op.trim())
        .collect::<std::result::Result<Vec<_>, _>>()
        .map_err(|()| eyre::eyre!("failed to lex"))?;
    let stmts = parse(tokens)?;

    Ok(stmts)
}

struct Parser {
    tokens: std::iter::Peekable<std::vec::IntoIter<Token>>,
}

impl Parser {
    fn next(&mut self) -> Result<Token> {
        self.tokens.next().ok_or_eyre("reached EOF")
    }
    fn peek(&mut self) -> Result<&Token> {
        self.tokens.peek().ok_or_eyre("reached EOF")
    }
    fn expect(&mut self, token: Token) -> Result<()> {
        let n = self.next()?;
        if n != token {
            bail!("expected {token:?}, found {n:?}");
        }
        Ok(())
    }
    fn eat(&mut self, token: Token) -> bool {
        if self.tokens.peek() == Some(&token) {
            self.tokens.next();
            true
        } else {
            false
        }
    }
}

fn parse(tokens: Vec<Token>) -> Result<Vec<Stmt>> {
    let parser = &mut Parser {
        tokens: tokens.into_iter().peekable(),
    };
    let mut stmts = Vec::new();

    while parser.tokens.peek().is_some() {
        let stmt = parse_stmt(parser)?;
        stmts.push(stmt);
    }

    Ok(stmts)
}

fn parse_stmt(parser: &mut Parser) -> Result<Stmt> {
    let stmt = match parser.peek() {
        _ => {
            let expr = parse_expr(parser)?;

            if parser.eat(Token::Assign) {
                let rhs = parse_expr(parser)?;

                Stmt::Assign { lhs: expr, rhs }
            } else {
                todo!()
            }
        }
    };
    parser.eat(Token::Newline);
    Ok(stmt)
}

fn parse_expr(parser: &mut Parser) -> Result<Expr> {
    parse_expr_range(parser)
}

fn parse_expr_range(parser: &mut Parser) -> Result<Expr> {
    let expr = parse_expr_call(parser)?;
    if parser.eat(Token::Colon) {
        let rhs = parse_expr_call(parser)?;
        Ok(Expr::Range {
            left: Box::new(expr),
            right: Box::new(rhs),
        })
    } else {
        Ok(expr)
    }
}

fn parse_expr_call(parser: &mut Parser) -> Result<Expr> {
    let mut lhs = parse_expr_atom(parser)?;

    loop {
        if parser.eat(Token::ParenOpen) {
            let arg = parse_expr(parser)?;
            parser.expect(Token::ParenClose);
            let Expr::Ident(ident) = lhs else {
                panic!("lhs of function must be ident");
            };
            lhs = Expr::Call {
                function: ident,
                args: vec![arg],
            };
            continue;
        }
        if parser.eat(Token::BracketOpen) {
            let arg = parse_expr(parser)?;
            parser.expect(Token::BracketClose);
            lhs = Expr::Index {
                lhs: Box::new(lhs),
                idx: Box::new(arg),
            };
            continue;
        }
        break;
    }

    Ok(lhs)
}

fn parse_expr_atom(parser: &mut Parser) -> Result<Expr> {
    let token = parser.next()?;
    Ok(match token {
        Token::Ident(ident) => Expr::Ident(ident),
        Token::Integer(int) => Expr::Int(int),
        _ => panic!("unexpected token in expr position: {token:?}"),
    })
}

#[derive(Debug, Logos, PartialEq, Eq)]
enum Token {
    #[regex("//[^\n]*\n?", logos::skip)]
    #[regex(r"[ \t\r]+", logos::skip)]
    Comment,

    #[token("(")]
    ParenOpen,
    #[token("[")]
    BracketOpen,
    #[token(")")]
    ParenClose,
    #[token("]")]
    BracketClose,

    #[token("DEFINE")]
    Define,
    #[token("CASE")]
    Case,
    #[token("ESAC")]
    Esac,
    #[token("IF")]
    If,
    #[token("FI")]
    Fi,
    #[token("RETURN")]
    Return,
    #[token("OF")]
    Of,

    #[token("\n")]
    Newline,

    #[token(",")]
    Comma,
    #[token(":=")]
    Assign,
    #[token(":")]
    Colon,

    #[regex(r"[a-zA-Z_]\w*", |lex| lex.slice().to_owned())]
    Ident(String),

    #[regex(r"\d+", |lex| lex.slice().parse().ok())]
    Integer(u64),
}
