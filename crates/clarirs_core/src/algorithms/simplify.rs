use std::sync::Arc;

use crate::prelude::*;

pub trait Simplify<'c> {
    fn simplify(&self) -> Result<AstRef<'c>, ClarirsError>;
}

impl<'c> Simplify<'c> for AstRef<'c> {
    fn simplify(&self) -> Result<AstRef<'c>, ClarirsError> {
        simplify(self)
    }
}

pub fn simplify<'c>(ast: &AstRef<'c>) -> Result<AstRef<'c>, ClarirsError> {
    let ctx = ast.context();

    if let Some(ast) = ctx.simplification_cache.read()?.get(&ast.hash()) {
        if let Some(ast) = ast.upgrade() {
            return Ok(ast);
        }
    }

    macro_rules! simplify {
        ($($var:ident),*) => {
            $(let $var = simplify(&$var)?;)*
        };
    }

    let ast: AstRef = match &ast.op() {
        AstOp::BoolS(..)
        | AstOp::BoolV(..)
        | AstOp::BVS(..)
        | AstOp::BVV(..)
        | AstOp::SI(..)
        | AstOp::FPS(..)
        | AstOp::FPV(..)
        | AstOp::StringS(..)
        | AstOp::StringV(..) => ast.clone(),
        AstOp::Not(ast) => {
            simplify!(ast);
            match &ast.op() {
                AstOp::Not(ast) => ast.clone(),
                AstOp::BoolV(v) => ctx.boolv(!v)?,
                AstOp::BVV(v) => ctx.bvv(!v.clone())?,
                _ => ctx.not(&ast)?,
            }
        }
        AstOp::And(lhs, rhs) => {
            simplify!(lhs, rhs);
            match (lhs.op(), rhs.op()) {
                (AstOp::BoolV(lhs), AstOp::BoolV(rhs)) => ctx.boolv(*lhs && *rhs)?,
                (AstOp::BoolV(true), v) | (v, AstOp::BoolV(true)) => ctx.make_ast(v.clone())?,
                (AstOp::BoolV(false), _) | (_, AstOp::BoolV(false)) => ctx.false_()?,
                (AstOp::Not(lhs), AstOp::Not(rhs)) => ctx.not(&ctx.or(lhs, rhs)?)?,
                (AstOp::BVV(lhs), AstOp::BVV(rhs)) => ctx.bvv(lhs.clone() & rhs.clone())?,
                _ => ctx.and(&lhs, &rhs)?,
            }
        }
        AstOp::Or(lhs, rhs) => {
            simplify!(lhs, rhs);
            match (lhs.op(), rhs.op()) {
                (AstOp::BoolV(lhs), AstOp::BoolV(rhs)) => ctx.boolv(*lhs || *rhs)?,
                (AstOp::BoolV(true), _) | (_, AstOp::BoolV(true)) => ctx.true_()?,
                (AstOp::BoolV(false), v) | (v, AstOp::BoolV(false)) => ctx.make_ast(v.clone())?,
                (AstOp::Not(lhs), AstOp::Not(rhs)) => ctx.not(&ctx.and(lhs, rhs)?)?,
                (AstOp::BVV(lhs), AstOp::BVV(rhs)) => ctx.bvv(lhs.clone() | rhs.clone())?,
                _ => ctx.or(&lhs, &rhs)?,
            }
        }
        AstOp::Xor(lhs, rhs) => {
            simplify!(lhs, rhs);
            match (lhs.op(), rhs.op()) {
                (AstOp::BoolV(lhs), AstOp::BoolV(rhs)) => ctx.boolv(*lhs ^ *rhs)?,
                (AstOp::BoolV(true), v) | (v, AstOp::BoolV(true)) => {
                    ctx.not(&ctx.make_ast(v.clone())?)?
                }
                (AstOp::BoolV(false), v) | (v, AstOp::BoolV(false)) => ctx.make_ast(v.clone())?,
                (AstOp::BVV(lhs), AstOp::BVV(rhs)) => ctx.bvv(lhs.clone() ^ rhs.clone())?,
                _ => ctx.xor(&lhs, &rhs)?,
            }
        }
        AstOp::Add(lhs, rhs) => {
            simplify!(lhs, rhs);
            match (lhs.op(), rhs.op()) {
                (AstOp::BVV(lhs), AstOp::BVV(rhs)) => ctx.bvv(lhs.clone() + rhs.clone())?,
                _ => ctx.add(&lhs, &rhs)?,
            }
        }
        AstOp::Sub(lhs, rhs) => {
            simplify!(lhs, rhs);
            match (lhs.op(), rhs.op()) {
                (AstOp::BVV(lhs), AstOp::BVV(rhs)) => ctx.bvv(lhs.clone() - rhs.clone())?,
                _ => ctx.sub(&lhs, &rhs)?,
            }
        }
        AstOp::Mul(lhs, rhs) => {
            simplify!(lhs, rhs);
            match (lhs.op(), rhs.op()) {
                (AstOp::BVV(lhs), AstOp::BVV(rhs)) => ctx.bvv(lhs.clone() * rhs.clone())?,
                _ => ctx.mul(&lhs, &rhs)?,
            }
        }
        AstOp::UDiv(lhs, rhs) => {
            simplify!(lhs, rhs);
            match (lhs.op(), rhs.op()) {
                (AstOp::BVV(lhs), AstOp::BVV(rhs)) => ctx.bvv(lhs.clone() / rhs.clone())?,
                _ => ctx.udiv(&lhs, &rhs)?,
            }
        }
        AstOp::SDiv(_, _) => todo!(),
        AstOp::URem(lhs, rhs) => {
            simplify!(lhs, rhs);
            match (lhs.op(), rhs.op()) {
                (AstOp::BVV(lhs), AstOp::BVV(rhs)) => ctx.bvv(lhs.clone() % rhs.clone())?,
                _ => ctx.urem(&lhs, &rhs)?,
            }
        }
        AstOp::SRem(_, _) => todo!(),
        AstOp::Pow(_, _) => todo!(),
        AstOp::LShL(_, _) => todo!(),
        AstOp::LShR(_, _) => todo!(),
        AstOp::AShL(_, _) => todo!(),
        AstOp::AShR(_, _) => todo!(),
        AstOp::RotateLeft(_, _) => todo!(),
        AstOp::RotateRight(_, _) => todo!(),
        AstOp::ZeroExt(_, _) => todo!(),
        AstOp::SignExt(_, _) => todo!(),
        AstOp::Extract(_, _, _) => todo!(),
        AstOp::Concat(_, _) => todo!(),
        AstOp::Reverse(ast) => {
            simplify!(ast);
            match &ast.op() {
                AstOp::BVV(v) => ctx.bvv(v.clone().reverse())?,
                AstOp::Reverse(ast) => ast.clone(),
                _ => ctx.reverse(&ast)?,
            }
        }
        AstOp::Eq(lhs, rhs) => {
            simplify!(lhs, rhs);
            match (lhs.op(), rhs.op()) {
                (AstOp::BVV(lhs), AstOp::BVV(rhs)) => ctx.boolv(lhs == rhs)?,
                _ => ctx.eq_(&lhs, &rhs)?,
            }
        }
        AstOp::Neq(lhs, rhs) => {
            simplify!(lhs, rhs);
            match (lhs.op(), rhs.op()) {
                (AstOp::BVV(lhs), AstOp::BVV(rhs)) => ctx.boolv(lhs != rhs)?,
                _ => ctx.neq(&lhs, &rhs)?,
            }
        }
        AstOp::ULT(lhs, rhs) => {
            simplify!(lhs, rhs);
            match (lhs.op(), rhs.op()) {
                (AstOp::BVV(lhs), AstOp::BVV(rhs)) => ctx.boolv(lhs < rhs)?,
                _ => ctx.ult(&lhs, &rhs)?,
            }
        }
        AstOp::ULE(lhs, rhs) => {
            simplify!(lhs, rhs);
            match (lhs.op(), rhs.op()) {
                (AstOp::BVV(lhs), AstOp::BVV(rhs)) => ctx.boolv(lhs <= rhs)?,
                _ => ctx.ule(&lhs, &rhs)?,
            }
        }
        AstOp::UGT(lhs, rhs) => {
            simplify!(lhs, rhs);
            match (lhs.op(), rhs.op()) {
                (AstOp::BVV(lhs), AstOp::BVV(rhs)) => ctx.boolv(lhs > rhs)?,
                _ => ctx.ugt(&lhs, &rhs)?,
            }
        }
        AstOp::UGE(lhs, rhs) => {
            simplify!(lhs, rhs);
            match (lhs.op(), rhs.op()) {
                (AstOp::BVV(lhs), AstOp::BVV(rhs)) => ctx.boolv(lhs >= rhs)?,
                _ => ctx.uge(&lhs, &rhs)?,
            }
        }
        AstOp::SLT(lhs, rhs) => {
            simplify!(lhs, rhs);
            match (lhs.op(), rhs.op()) {
                (AstOp::BVV(lhs), AstOp::BVV(rhs)) => ctx.boolv(lhs < rhs)?,
                _ => ctx.slt(&lhs, &rhs)?,
            }
        }
        AstOp::SLE(lhs, rhs) => {
            simplify!(lhs, rhs);
            match (lhs.op(), rhs.op()) {
                (AstOp::BVV(lhs), AstOp::BVV(rhs)) => ctx.boolv(lhs <= rhs)?,
                _ => ctx.sle(&lhs, &rhs)?,
            }
        }
        AstOp::SGT(lhs, rhs) => {
            simplify!(lhs, rhs);
            match (lhs.op(), rhs.op()) {
                (AstOp::BVV(lhs), AstOp::BVV(rhs)) => ctx.boolv(lhs > rhs)?,
                _ => ctx.sgt(&lhs, &rhs)?,
            }
        }
        AstOp::SGE(lhs, rhs) => {
            simplify!(lhs, rhs);
            match (lhs.op(), rhs.op()) {
                (AstOp::BVV(lhs), AstOp::BVV(rhs)) => ctx.boolv(lhs >= rhs)?,
                _ => ctx.sge(&lhs, &rhs)?,
            }
        }
        AstOp::FpToFp(_, _) => todo!(),
        AstOp::BvToFpUnsigned(_, _, _) => todo!(),
        AstOp::FpToIEEEBV(_) => todo!(),
        AstOp::FpToUBV(_, _, _) => todo!(),
        AstOp::FpToSBV(_, _, _) => todo!(),
        AstOp::FpNeg(_, _) => todo!(),
        AstOp::FpAbs(_, _) => todo!(),
        AstOp::FpAdd(_, _, _) => todo!(),
        AstOp::FpSub(_, _, _) => todo!(),
        AstOp::FpMul(_, _, _) => todo!(),
        AstOp::FpDiv(_, _, _) => todo!(),
        AstOp::FpSqrt(_, _) => todo!(),
        AstOp::FpEq(lhs, rhs) => {
            simplify!(lhs, rhs);
            match (lhs.op(), rhs.op()) {
                (AstOp::FPV(lhs), AstOp::FPV(rhs)) => ctx.boolv(lhs == rhs)?,
                _ => ctx.fp_eq(&lhs, &rhs)?,
            }
        }
        AstOp::FpNeq(lhs, rhs) => {
            simplify!(lhs, rhs);
            match (lhs.op(), rhs.op()) {
                (AstOp::FPV(lhs), AstOp::FPV(rhs)) => ctx.boolv(lhs != rhs)?,
                _ => ctx.fp_neq(&lhs, &rhs)?,
            }
        }
        AstOp::FpLt(_, _) => todo!(),
        AstOp::FpLeq(_, _) => todo!(),
        AstOp::FpGt(_, _) => todo!(),
        AstOp::FpGeq(_, _) => todo!(),
        AstOp::FpIsNan(_) => todo!(),
        AstOp::FpIsInf(_) => todo!(),
        AstOp::StrLen(_) => todo!(),
        AstOp::StrConcat(_, _) => todo!(),
        AstOp::StrSubstr(_, _, _) => todo!(),
        AstOp::StrContains(_, _) => todo!(),
        AstOp::StrIndexOf(_, _) => todo!(),
        AstOp::StrReplace(_, _, _) => todo!(),
        AstOp::StrPrefixOf(_, _) => todo!(),
        AstOp::StrSuffixOf(_, _) => todo!(),
        AstOp::StrToBV(_, _) => todo!(),
        AstOp::BVToStr(_) => todo!(),
        AstOp::StrIsDigit(_) => todo!(),
        AstOp::StrEq(lhs, rhs) => {
            simplify!(lhs, rhs);
            match (lhs.op(), rhs.op()) {
                (AstOp::StringV(lhs), AstOp::StringV(rhs)) => ctx.boolv(lhs == rhs)?,
                _ => ctx.streq(&lhs, &rhs)?,
            }
        }
        AstOp::StrNeq(lhs, rhs) => {
            simplify!(lhs, rhs);
            match (lhs.op(), rhs.op()) {
                (AstOp::StringV(lhs), AstOp::StringV(rhs)) => ctx.boolv(lhs != rhs)?,
                _ => ctx.strneq(&lhs, &rhs)?,
            }
        }
        AstOp::If(cond, then, else_) => {
            simplify!(cond, then, else_);
            match &cond.op() {
                AstOp::BoolV(true) => then.clone(),
                AstOp::BoolV(false) => else_.clone(),
                _ => ctx.if_(&cond, &then, &else_)?,
            }
        }
        AstOp::Annotated(ast, anno) => {
            simplify!(ast);
            if anno.eliminatable() {
                ast.clone()
            } else {
                ctx.annotated(&ast, anno.clone())?
            }
        }
    };

    ctx.simplification_cache
        .write()?
        .insert(ast.hash(), Arc::downgrade(&ast));
    Ok(ast)
}
