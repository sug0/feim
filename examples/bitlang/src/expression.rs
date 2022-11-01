use std::num::IntErrorKind;

pub type Num = isize;

#[derive(Copy, Clone, Debug)]
pub enum Item {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    ShRight,
    ShLeft,
    Eq,
    Gt,
    Lt,
    Ge,
    Le,
    Or,
    And,
    Not,
    BitOr,
    BitAnd,
    BitXor,
    BitNot,
    Abs,
    VarX,
    VarY,
    VarW,
    VarH,
    VarT,
    Dup,
    Xch,
    Num(Num),
}

#[derive(Debug)]
pub enum CompileError<'a> {
    UnknownToken(&'a str),
    InvalidInteger(&'a str, IntErrorKind),
}

pub fn compile<'a>(s: &'a str) -> Result<Expression, CompileError<'a>> {
    let mut expression = Vec::new();

    for tok in s.split_whitespace() {
        match tok {
            "+" => expression.push(Item::Add),
            "-" => expression.push(Item::Sub),
            "*" => expression.push(Item::Mul),
            "/" => expression.push(Item::Div),
            "%" => expression.push(Item::Mod),
            ">>" => expression.push(Item::ShRight),
            "<<" => expression.push(Item::ShLeft),
            "=" => expression.push(Item::Eq),
            ">" => expression.push(Item::Gt),
            "<" => expression.push(Item::Lt),
            ">=" => expression.push(Item::Ge),
            "<=" => expression.push(Item::Le),
            "||" => expression.push(Item::Or),
            "&&" => expression.push(Item::And),
            "!" => expression.push(Item::Not),
            "|" => expression.push(Item::BitOr),
            "&" => expression.push(Item::BitAnd),
            "^" => expression.push(Item::BitXor),
            "~" => expression.push(Item::BitNot),
            "abs" => expression.push(Item::Abs),
            "x" => expression.push(Item::VarX),
            "y" => expression.push(Item::VarY),
            "w" => expression.push(Item::VarW),
            "h" => expression.push(Item::VarH),
            "t" => expression.push(Item::VarT),
            "dup" => expression.push(Item::Dup),
            "xch" => expression.push(Item::Xch),
            tok => expression.push({
                let (sign, num) = match tok.chars().by_ref().next() {
                    Some('+') => (1, &tok[1..]),
                    Some('-') => (-1, &tok[1..]),
                    Some('0'..='9') => (1, tok),
                    _ => return Err(CompileError::UnknownToken(tok)),
                };
                let (num, radix) = if num.starts_with("0b") {
                    (&num[2..], 2)
                } else if num.starts_with("0o") {
                    (&num[2..], 8)
                } else if num.starts_with("0x") {
                    (&num[2..], 16)
                } else {
                    (num, 10)
                };
                Num::from_str_radix(num, radix)
                    .map(|n| Item::Num(n * sign))
                    .map_err(|e| match e.kind().clone() {
                        IntErrorKind::InvalidDigit => CompileError::UnknownToken(tok),
                        err_kind => CompileError::InvalidInteger(tok, err_kind),
                    })?
            }),
        }
    }

    Ok(Expression { inner: expression })
}

#[derive(Debug, Copy, Clone)]
pub struct Context {
    x: Num,
    y: Num,
    w: Num,
    h: Num,
    t: Num,
}

macro_rules! impl_ctx_access {
    ($field:ident) => {
        pub fn $field(&self) -> Num {
            self.$field
        }
    };
}

impl Context {
    impl_ctx_access!(x);
    impl_ctx_access!(y);
    impl_ctx_access!(w);
    impl_ctx_access!(h);
    impl_ctx_access!(t);
}

pub struct Expression {
    inner: Vec<Item>,
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    proptest! {
        #[test]
        fn test_parse_num(num in r"(-|\+)?(([0-9]{7})|(0b[01]{7})|(0o[0-7]{7})|(0x[a-fA-F0-9]{7}))") {
            let expr = compile(&num).unwrap();
            assert_eq!(expr.inner.len(), 1);
            assert!(matches!(&expr.inner[0], Item::Num(_)));
        }
    }
}
