use std::num::IntErrorKind;

use feim::buffer::RawPixBuf;
use feim::color::Nrgba64Be;
use feim::image::{Dimensions, ImageMut};
use itertools::Itertools;

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

#[derive(Debug)]
pub enum EvalError {
    Temp(String),
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
pub enum BitDepth {
    One,
    #[allow(dead_code)]
    Sixteen,
}

impl BitDepth {
    fn transform(self, value: Num) -> Nrgba64Be {
        let y = match self {
            BitDepth::One => 0xffff * (value as u16 & 1),
            BitDepth::Sixteen => (value & 0xffff) as u16,
        };
        Nrgba64Be::be(y, y, y, 0xffff)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Context {
    depth: BitDepth,
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
    pub fn depth(&self) -> BitDepth {
        self.depth
    }

    impl_ctx_access!(x);
    impl_ctx_access!(y);
    impl_ctx_access!(w);
    impl_ctx_access!(h);
    impl_ctx_access!(t);
}

pub struct Expression {
    inner: Vec<Item>,
}

impl Expression {
    pub fn evaluate_over(
        &self,
        image: &mut RawPixBuf<Nrgba64Be>,
        depth: BitDepth,
    ) -> Result<(), EvalError> {
        let (width, height) = image.dimensions();
        let width = width as Num;
        let height = height as Num;

        // test run
        if let Err(err) = self.evaluate(Context {
            depth: BitDepth::One,
            x: 0,
            y: 0,
            w: width,
            h: height,
            t: 0,
        }) {
            return Err(err);
        }

        rayon::scope(|s| {
            let (tx, rx) = flume::bounded(32);
            for ((x, y), t) in (0..width).cartesian_product(0..height).zip(0..) {
                let tx = tx.clone();
                s.spawn(move |_| {
                    let pix = self
                        .evaluate(Context {
                            w: width,
                            h: height,
                            depth,
                            x,
                            y,
                            t,
                        })
                        .unwrap();
                    tx.send((x, y, pix)).unwrap();
                });
            }
            for _ in 0..(width * height) {
                let (x, y, pix) = rx.recv().unwrap();
                image.pixel_set(x as usize, y as usize, pix);
            }
        });

        Ok(())
    }

    fn evaluate(&self, ctx: Context) -> Result<Nrgba64Be, EvalError> {
        let mut stack = Vec::new();

        #[inline]
        fn try_pop(stk: &mut Vec<Num>) -> Result<Num, EvalError> {
            stk.pop()
                .ok_or_else(|| EvalError::Temp("No stack element to pop".into()))
        }

        #[inline]
        fn op_two<F>(stk: &mut Vec<Num>, op: F) -> Result<(), EvalError>
        where
            F: FnOnce(Num, Num) -> Num,
        {
            let b = try_pop(stk)?;
            let a = try_pop(stk)?;
            Ok(stk.push(op(a, b)))
        }

        #[inline]
        fn op_one<F>(stk: &mut Vec<Num>, op: F) -> Result<(), EvalError>
        where
            F: FnOnce(Num) -> Num,
        {
            let a = try_pop(stk)?;
            Ok(stk.push(op(a)))
        }

        #[inline]
        fn to_bool(x: Num) -> bool {
            x != 0
        }

        #[inline]
        fn from_bool(x: bool) -> Num {
            x as Num
        }

        for item in self.inner.iter().copied() {
            match item {
                Item::Add => op_two(&mut stack, |a, b| a.wrapping_add(b))?,
                Item::Sub => op_two(&mut stack, |a, b| a.wrapping_sub(b))?,
                Item::Mul => op_two(&mut stack, |a, b| a.wrapping_mul(b))?,
                Item::Div => op_two(&mut stack, |a, b| a.wrapping_div(b))?,
                Item::Mod => op_two(&mut stack, |a, b| a.wrapping_rem(b))?,
                Item::ShRight => {
                    op_two(&mut stack, |a, b| a.wrapping_shr((b & 0xffffffff) as u32))?
                }
                Item::ShLeft => op_two(&mut stack, |a, b| a.wrapping_shl((b & 0xffffffff) as u32))?,
                Item::Eq => op_two(&mut stack, |a, b| from_bool(a == b))?,
                Item::Gt => op_two(&mut stack, |a, b| from_bool(a > b))?,
                Item::Lt => op_two(&mut stack, |a, b| from_bool(a < b))?,
                Item::Ge => op_two(&mut stack, |a, b| from_bool(a >= b))?,
                Item::Le => op_two(&mut stack, |a, b| from_bool(a <= b))?,
                Item::Or => op_two(&mut stack, |a, b| from_bool(to_bool(a) || to_bool(b)))?,
                Item::And => op_two(&mut stack, |a, b| from_bool(to_bool(a) && to_bool(b)))?,
                Item::Not => op_one(&mut stack, |a| from_bool(!to_bool(a)))?,
                Item::BitOr => op_two(&mut stack, |a, b| a | b)?,
                Item::BitAnd => op_two(&mut stack, |a, b| a & b)?,
                Item::BitXor => op_two(&mut stack, |a, b| a ^ b)?,
                Item::BitNot => op_one(&mut stack, |a| !a)?,
                Item::Abs => op_one(&mut stack, |a| a.wrapping_abs())?,
                Item::VarX => stack.push(ctx.x()),
                Item::VarY => stack.push(ctx.y()),
                Item::VarW => stack.push(ctx.w()),
                Item::VarH => stack.push(ctx.h()),
                Item::VarT => stack.push(ctx.t()),
                Item::Dup => {
                    let a = try_pop(&mut stack)?;
                    stack.push(a);
                    stack.push(a);
                }
                Item::Xch => {
                    let b = try_pop(&mut stack)?;
                    let a = try_pop(&mut stack)?;
                    stack.push(b);
                    stack.push(a);
                }
                Item::Num(n) => stack.push(n),
            }
        }

        if stack.len() == 1 {
            Ok(ctx.depth().transform(stack.remove(0)))
        } else {
            Err(EvalError::Temp(format!(
                "Expect 1 element at the end, got {} stack elements",
                stack.len()
            )))
        }
    }
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
