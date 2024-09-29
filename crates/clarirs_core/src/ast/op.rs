use serde::Serialize;

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AstOp<'c> {
    // Primitive ops
    BoolS(String),
    BoolV(bool),
    BVS(String, u32),
    BVV(BitVec),
    FPS(String, FSort),
    FPV(Float),
    StringS(String, u32),
    StringV(String),

    // Bit ops
    Not(AstRef<'c>),
    And(AstRef<'c>, AstRef<'c>),
    Or(AstRef<'c>, AstRef<'c>),
    Xor(AstRef<'c>, AstRef<'c>),

    // Arithmetic ops
    Add(AstRef<'c>, AstRef<'c>),
    Sub(AstRef<'c>, AstRef<'c>),
    Mul(AstRef<'c>, AstRef<'c>),
    UDiv(AstRef<'c>, AstRef<'c>),
    SDiv(AstRef<'c>, AstRef<'c>),
    URem(AstRef<'c>, AstRef<'c>),
    SRem(AstRef<'c>, AstRef<'c>),
    Pow(AstRef<'c>, AstRef<'c>),

    // Bitvector ops
    LShL(AstRef<'c>, AstRef<'c>),
    LShR(AstRef<'c>, AstRef<'c>),
    AShL(AstRef<'c>, AstRef<'c>),
    AShR(AstRef<'c>, AstRef<'c>),
    RotateLeft(AstRef<'c>, AstRef<'c>),
    RotateRight(AstRef<'c>, AstRef<'c>),
    ZeroExt(AstRef<'c>, u32),
    SignExt(AstRef<'c>, u32),
    Extract(AstRef<'c>, u32, u32),
    Concat(AstRef<'c>, AstRef<'c>),
    Reverse(AstRef<'c>),

    // Bitvector comparison ops
    Eq(AstRef<'c>, AstRef<'c>),
    Neq(AstRef<'c>, AstRef<'c>),
    ULT(AstRef<'c>, AstRef<'c>),
    ULE(AstRef<'c>, AstRef<'c>),
    UGT(AstRef<'c>, AstRef<'c>),
    UGE(AstRef<'c>, AstRef<'c>),
    SLT(AstRef<'c>, AstRef<'c>),
    SLE(AstRef<'c>, AstRef<'c>),
    SGT(AstRef<'c>, AstRef<'c>),
    SGE(AstRef<'c>, AstRef<'c>),

    // Floating point ops
    FpToFp(AstRef<'c>, FSort, FPRM), // FpToFp(AstRef<'c>, FSort, FPRM)
    BvToFpUnsigned(AstRef<'c>, FSort, FPRM), // Check is this is correct
    FpToIEEEBV(AstRef<'c>),          // Check is this is correct

    FpToUBV(AstRef<'c>, u32, FPRM),
    FpToSBV(AstRef<'c>, u32, FPRM),

    // Floating point arithmetic ops
    FpNeg(AstRef<'c>, FPRM),
    FpAbs(AstRef<'c>, FPRM),
    FpAdd(AstRef<'c>, AstRef<'c>, FPRM),
    FpSub(AstRef<'c>, AstRef<'c>, FPRM),
    FpMul(AstRef<'c>, AstRef<'c>, FPRM),
    FpDiv(AstRef<'c>, AstRef<'c>, FPRM),
    FpSqrt(AstRef<'c>, FPRM),

    // Floating point comparison ops
    FpEq(AstRef<'c>, AstRef<'c>),
    FpNeq(AstRef<'c>, AstRef<'c>),
    FpLt(AstRef<'c>, AstRef<'c>),
    FpLeq(AstRef<'c>, AstRef<'c>),
    FpGt(AstRef<'c>, AstRef<'c>),
    FpGeq(AstRef<'c>, AstRef<'c>),
    FpIsNan(AstRef<'c>),
    FpIsInf(AstRef<'c>),

    // String ops
    StrLen(AstRef<'c>),
    StrConcat(AstRef<'c>, AstRef<'c>), // StrConcat(Vec<AstRef<'c>>) To allow for any number of args,
    StrSubstr(AstRef<'c>, AstRef<'c>, AstRef<'c>),
    StrContains(AstRef<'c>, AstRef<'c>),
    StrIndexOf(AstRef<'c>, AstRef<'c>, AstRef<'c>), // String, String, BV (offset)
    StrReplace(AstRef<'c>, AstRef<'c>, AstRef<'c>),
    StrPrefixOf(AstRef<'c>, AstRef<'c>),
    StrSuffixOf(AstRef<'c>, AstRef<'c>),
    StrToBV(AstRef<'c>),
    BVToStr(AstRef<'c>),
    StrIsDigit(AstRef<'c>),

    // String comparison ops
    StrEq(AstRef<'c>, AstRef<'c>),
    StrNeq(AstRef<'c>, AstRef<'c>),

    // Function ops
    If(AstRef<'c>, AstRef<'c>, AstRef<'c>),

    // Annotation ops
    Annotated(AstRef<'c>, Annotation<'c>),
}

impl<'c> AstOp<'c> {
    pub fn valid_args(&self) -> bool {
        match self {
            AstOp::BoolS(name)
            | AstOp::BVS(name, ..)
            | AstOp::FPS(name, ..)
            | AstOp::StringS(name, ..) => !name.is_empty(),
            AstOp::BoolV(..) | AstOp::BVV(..) | AstOp::FPV(..) | AstOp::StringV(..) => true,
            AstOp::Not(ast) => ast.kind().is_bool() || ast.kind().is_bitvec(),
            AstOp::And(lhs, rhs) | AstOp::Or(lhs, rhs) | AstOp::Xor(lhs, rhs) => {
                (lhs.kind().is_bool() || lhs.kind().is_bitvec()) && lhs.kind() == rhs.kind()
            }

            // Bitvector ops and Bitvector comparison ops
            AstOp::Add(lhs, rhs)
            | AstOp::Sub(lhs, rhs)
            | AstOp::Mul(lhs, rhs)
            | AstOp::UDiv(lhs, rhs)
            | AstOp::SDiv(lhs, rhs)
            | AstOp::URem(lhs, rhs)
            | AstOp::SRem(lhs, rhs)
            | AstOp::Pow(lhs, rhs)
            | AstOp::LShL(lhs, rhs)
            | AstOp::LShR(lhs, rhs)
            | AstOp::AShL(lhs, rhs)
            | AstOp::AShR(lhs, rhs)
            | AstOp::RotateLeft(lhs, rhs)
            | AstOp::RotateRight(lhs, rhs)
            | AstOp::Concat(lhs, rhs)
            | AstOp::Eq(lhs, rhs)
            | AstOp::Neq(lhs, rhs)
            | AstOp::ULT(lhs, rhs)
            | AstOp::ULE(lhs, rhs)
            | AstOp::UGT(lhs, rhs)
            | AstOp::UGE(lhs, rhs)
            | AstOp::SLT(lhs, rhs)
            | AstOp::SLE(lhs, rhs)
            | AstOp::SGT(lhs, rhs)
            | AstOp::SGE(lhs, rhs) => lhs.kind().is_bitvec() && rhs.kind().is_bitvec(),
            AstOp::ZeroExt(ast, _)
            | AstOp::SignExt(ast, _)
            | AstOp::Extract(ast, _, _)
            | AstOp::Reverse(ast) => ast.kind().is_bitvec(),

            // Floating point ops
            AstOp::FpToFp(ast, _, _)
            | AstOp::BvToFpUnsigned(ast, _, _)
            | AstOp::FpToIEEEBV(ast)
            | AstOp::FpToSBV(ast, _, _) => ast.kind().is_float(),
            AstOp::FpToUBV(ast, size, _) => ast.kind().is_float() && *size > 0,

            // Floating point arithmetic ops
            AstOp::FpNeg(ast, _) | AstOp::FpAbs(ast, _) | AstOp::FpSqrt(ast, _) => {
                ast.kind().is_float()
            }
            AstOp::FpAdd(lhs, rhs, _)
            | AstOp::FpSub(lhs, rhs, _)
            | AstOp::FpMul(lhs, rhs, _)
            | AstOp::FpDiv(lhs, rhs, _) => lhs.kind().is_float() && rhs.kind().is_float(),

            // Floating point comparison ops
            AstOp::FpEq(lhs, rhs)
            | AstOp::FpNeq(lhs, rhs)
            | AstOp::FpLt(lhs, rhs)
            | AstOp::FpLeq(lhs, rhs)
            | AstOp::FpGt(lhs, rhs)
            | AstOp::FpGeq(lhs, rhs) => lhs.kind().is_float() && rhs.kind().is_float(),
            AstOp::FpIsNan(ast) | AstOp::FpIsInf(ast) => ast.kind().is_float(),

            // String ops
            AstOp::StrLen(ast) => ast.kind().is_string(),
            AstOp::StrConcat(lhs, rhs) => lhs.kind().is_string() && rhs.kind().is_string(),
            AstOp::StrSubstr(lhs, rhs, str) => {
                lhs.kind().is_bitvec() && rhs.kind().is_bitvec() && str.kind().is_string()
            }
            AstOp::StrReplace(lhs, rhs, str) => {
                lhs.kind().is_string() && rhs.kind().is_string() && str.kind().is_string()
            }
            AstOp::StrContains(lhs, rhs)
            | AstOp::StrPrefixOf(lhs, rhs)
            | AstOp::StrSuffixOf(lhs, rhs) => lhs.kind().is_string() && rhs.kind().is_string(),
            AstOp::StrIndexOf(base, substr, offset) => {
                base.kind().is_string() && substr.kind().is_string() && offset.kind().is_bitvec()
            }
            AstOp::StrToBV(ast) => ast.kind().is_string(),
            AstOp::BVToStr(ast) => ast.kind().is_bitvec(),
            AstOp::StrIsDigit(ast) => ast.kind().is_string(),

            // String comparison ops
            AstOp::StrEq(lhs, rhs) | AstOp::StrNeq(lhs, rhs) => {
                lhs.kind().is_string() && rhs.kind().is_string()
            }

            AstOp::If(_, _, _) => todo!(),
            AstOp::Annotated(_, _) => todo!(),
        }
    }

    pub fn kind(&self) -> AstKind {
        match self {
            AstOp::BoolS(..) | AstOp::BoolV(..) => AstKind::Bool,
            AstOp::BVS(..) | AstOp::BVV(..) => AstKind::BitVec,
            AstOp::FPS(..) | AstOp::FPV(..) => AstKind::Float,
            AstOp::StringS(..) | AstOp::StringV(..) => AstKind::String,
            AstOp::Not(ast)
            | AstOp::And(ast, ..)
            | AstOp::Or(ast, ..)
            | AstOp::Xor(ast, ..)
            | AstOp::If(.., ast) => ast.kind(),
            AstOp::Add(..)
            | AstOp::Sub(..)
            | AstOp::Mul(..)
            | AstOp::UDiv(..)
            | AstOp::SDiv(..)
            | AstOp::URem(..)
            | AstOp::SRem(..)
            | AstOp::Pow(..)
            | AstOp::LShL(..)
            | AstOp::LShR(..)
            | AstOp::AShL(..)
            | AstOp::AShR(..)
            | AstOp::RotateLeft(..)
            | AstOp::RotateRight(..)
            | AstOp::ZeroExt(..)
            | AstOp::SignExt(..)
            | AstOp::Extract(..)
            | AstOp::Concat(..)
            | AstOp::Reverse(..) => AstKind::BitVec,
            AstOp::Eq(..)
            | AstOp::Neq(..)
            | AstOp::ULT(..)
            | AstOp::ULE(..)
            | AstOp::UGT(..)
            | AstOp::UGE(..)
            | AstOp::SLT(..)
            | AstOp::SLE(..)
            | AstOp::SGT(..)
            | AstOp::SGE(..) => AstKind::Bool,
            AstOp::FpToFp(..) | AstOp::BvToFpUnsigned(..) => AstKind::Float,
            AstOp::FpToIEEEBV(..) | AstOp::FpToUBV(..) | AstOp::FpToSBV(..) => AstKind::BitVec,
            AstOp::FpNeg(..)
            | AstOp::FpAbs(..)
            | AstOp::FpAdd(..)
            | AstOp::FpSub(..)
            | AstOp::FpMul(..)
            | AstOp::FpDiv(..)
            | AstOp::FpSqrt(..) => AstKind::Float,
            AstOp::FpEq(..)
            | AstOp::FpNeq(..)
            | AstOp::FpLt(..)
            | AstOp::FpLeq(..)
            | AstOp::FpGt(..)
            | AstOp::FpGeq(..)
            | AstOp::FpIsNan(..)
            | AstOp::FpIsInf(..) => AstKind::Bool,
            AstOp::StrLen(..) => AstKind::BitVec,
            AstOp::StrConcat(..) | AstOp::StrSubstr(..) => AstKind::String,
            AstOp::StrContains(..) => AstKind::Bool,
            AstOp::StrIndexOf(..) => AstKind::BitVec,
            AstOp::StrReplace(..) => AstKind::String,
            AstOp::StrPrefixOf(..) | AstOp::StrSuffixOf(..) => AstKind::Bool,
            AstOp::StrToBV(..) => AstKind::BitVec,
            AstOp::BVToStr(..) => AstKind::String,
            AstOp::StrIsDigit(..) | AstOp::StrEq(..) | AstOp::StrNeq(..) => AstKind::Bool,
            AstOp::Annotated(ast, ..) => ast.kind(),
        }
    }

    pub fn child_iter(&self) -> impl Iterator<Item = &AstRef<'c>> {
        match self {
            AstOp::BoolS(..)
            | AstOp::BoolV(..)
            | AstOp::BVS(..)
            | AstOp::BVV(..)
            | AstOp::FPS(..)
            | AstOp::FPV(..)
            | AstOp::StringS(..)
            | AstOp::StringV(..) => Vec::new().into_iter(),
            AstOp::Not(a)
            | AstOp::Reverse(a)
            | AstOp::ZeroExt(a, ..)
            | AstOp::SignExt(a, ..)
            | AstOp::Extract(a, ..)
            | AstOp::FpToFp(a, ..)
            | AstOp::BvToFpUnsigned(a, ..)
            | AstOp::FpToIEEEBV(a)
            | AstOp::FpToUBV(a, ..)
            | AstOp::FpToSBV(a, ..)
            | AstOp::FpNeg(a, ..)
            | AstOp::FpAbs(a, ..)
            | AstOp::FpSqrt(a, ..)
            | AstOp::FpIsNan(a)
            | AstOp::FpIsInf(a)
            | AstOp::StrLen(a, ..)
            | AstOp::StrToBV(a, ..)
            | AstOp::BVToStr(a)
            | AstOp::StrIsDigit(a)
            | AstOp::Annotated(a, ..) => vec![a].into_iter(),
            AstOp::And(a, b)
            | AstOp::Or(a, b)
            | AstOp::Xor(a, b)
            | AstOp::Add(a, b)
            | AstOp::Sub(a, b)
            | AstOp::Mul(a, b)
            | AstOp::UDiv(a, b)
            | AstOp::SDiv(a, b)
            | AstOp::URem(a, b)
            | AstOp::SRem(a, b)
            | AstOp::Pow(a, b)
            | AstOp::LShL(a, b)
            | AstOp::LShR(a, b)
            | AstOp::AShL(a, b)
            | AstOp::AShR(a, b)
            | AstOp::RotateLeft(a, b)
            | AstOp::RotateRight(a, b)
            | AstOp::Concat(a, b)
            | AstOp::Eq(a, b)
            | AstOp::Neq(a, b)
            | AstOp::ULT(a, b)
            | AstOp::ULE(a, b)
            | AstOp::UGT(a, b)
            | AstOp::UGE(a, b)
            | AstOp::SLT(a, b)
            | AstOp::SLE(a, b)
            | AstOp::SGT(a, b)
            | AstOp::SGE(a, b)
            | AstOp::FpAdd(a, b, ..)
            | AstOp::FpSub(a, b, ..)
            | AstOp::FpMul(a, b, ..)
            | AstOp::FpDiv(a, b, ..)
            | AstOp::FpEq(a, b)
            | AstOp::FpNeq(a, b)
            | AstOp::FpLt(a, b)
            | AstOp::FpLeq(a, b)
            | AstOp::FpGt(a, b)
            | AstOp::FpGeq(a, b)
            | AstOp::StrConcat(a, b)
            | AstOp::StrContains(a, b)
            | AstOp::StrIndexOf(a, b, _)
            | AstOp::StrPrefixOf(a, b)
            | AstOp::StrSuffixOf(a, b)
            | AstOp::StrEq(a, b)
            | AstOp::StrNeq(a, b) => vec![a, b].into_iter(),
            AstOp::StrSubstr(a, b, c) | AstOp::StrReplace(a, b, c) | AstOp::If(a, b, c) => {
                vec![a, b, c].into_iter()
            }
        }
    }

    pub fn children(&self) -> Vec<&AstRef<'c>> {
        self.child_iter().collect()
    }
}
