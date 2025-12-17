use std::{collections::HashMap, sync::Arc};

use crate::{
    parser::{Expr, Param, Statement, Stmt},
    token::{Literal, NumericType, Operator},
};

#[derive(Debug, Clone, PartialEq)]
pub enum InterpretValue {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    ISize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    USize(usize),
    F32(f32),
    F64(f64),
    Boolean(bool),
    String(String),
    Void,
}

impl InterpretValue {
    pub fn from_literal(lit: Literal) -> Self {
        match lit {
            Literal::Numeric(value, num_type) => match num_type {
                NumericType::I8 => InterpretValue::I8(value.parse().unwrap()),
                NumericType::I16 => InterpretValue::I16(value.parse().unwrap()),
                NumericType::I32 => InterpretValue::I32(value.parse().unwrap()),
                NumericType::I64 => InterpretValue::I64(value.parse().unwrap()),
                NumericType::ISize => InterpretValue::ISize(value.parse().unwrap()),
                NumericType::U8 => InterpretValue::U8(value.parse().unwrap()),
                NumericType::U16 => InterpretValue::U16(value.parse().unwrap()),
                NumericType::U32 => InterpretValue::U32(value.parse().unwrap()),
                NumericType::U64 => InterpretValue::U64(value.parse().unwrap()),
                NumericType::USize => InterpretValue::USize(value.parse().unwrap()),
                NumericType::F32 => InterpretValue::F32(value.parse().unwrap()),
                NumericType::F64 => InterpretValue::F64(value.parse().unwrap()),
            },
            Literal::String(value) => InterpretValue::String(value),
            Literal::Boolean(value) => InterpretValue::Boolean(value == "true"),
        }
    }

    pub fn add(&self, other: &InterpretValue) -> anyhow::Result<InterpretValue> {
        match (self, other) {
            (InterpretValue::I8(a), InterpretValue::I8(b)) => Ok(InterpretValue::I8(a + b)),
            (InterpretValue::I16(a), InterpretValue::I16(b)) => Ok(InterpretValue::I16(a + b)),
            (InterpretValue::I32(a), InterpretValue::I32(b)) => Ok(InterpretValue::I32(a + b)),
            (InterpretValue::I64(a), InterpretValue::I64(b)) => Ok(InterpretValue::I64(a + b)),
            (InterpretValue::ISize(a), InterpretValue::ISize(b)) => {
                Ok(InterpretValue::ISize(a + b))
            }
            (InterpretValue::U8(a), InterpretValue::U8(b)) => Ok(InterpretValue::U8(a + b)),
            (InterpretValue::U16(a), InterpretValue::U16(b)) => Ok(InterpretValue::U16(a + b)),
            (InterpretValue::U32(a), InterpretValue::U32(b)) => Ok(InterpretValue::U32(a + b)),
            (InterpretValue::U64(a), InterpretValue::U64(b)) => Ok(InterpretValue::U64(a + b)),
            (InterpretValue::USize(a), InterpretValue::USize(b)) => {
                Ok(InterpretValue::USize(a + b))
            }
            (InterpretValue::F32(a), InterpretValue::F32(b)) => Ok(InterpretValue::F32(a + b)),
            (InterpretValue::F64(a), InterpretValue::F64(b)) => Ok(InterpretValue::F64(a + b)),
            (InterpretValue::String(a), InterpretValue::String(b)) => {
                Ok(InterpretValue::String(format!("{}{}", a, b)))
            }
            x => anyhow::bail!("Addition is not supported for given value types {x:?}"),
        }
    }

    pub fn sub(&self, other: &InterpretValue) -> anyhow::Result<InterpretValue> {
        match (self, other) {
            (InterpretValue::I8(a), InterpretValue::I8(b)) => Ok(InterpretValue::I8(a - b)),
            (InterpretValue::I16(a), InterpretValue::I16(b)) => Ok(InterpretValue::I16(a - b)),
            (InterpretValue::I32(a), InterpretValue::I32(b)) => Ok(InterpretValue::I32(a - b)),
            (InterpretValue::I64(a), InterpretValue::I64(b)) => Ok(InterpretValue::I64(a - b)),
            (InterpretValue::ISize(a), InterpretValue::ISize(b)) => {
                Ok(InterpretValue::ISize(a - b))
            }
            (InterpretValue::U8(a), InterpretValue::U8(b)) => Ok(InterpretValue::U8(a - b)),
            (InterpretValue::U16(a), InterpretValue::U16(b)) => Ok(InterpretValue::U16(a - b)),
            (InterpretValue::U32(a), InterpretValue::U32(b)) => Ok(InterpretValue::U32(a - b)),
            (InterpretValue::U64(a), InterpretValue::U64(b)) => Ok(InterpretValue::U64(a - b)),
            (InterpretValue::USize(a), InterpretValue::USize(b)) => {
                Ok(InterpretValue::USize(a - b))
            }
            (InterpretValue::F32(a), InterpretValue::F32(b)) => Ok(InterpretValue::F32(a - b)),
            (InterpretValue::F64(a), InterpretValue::F64(b)) => Ok(InterpretValue::F64(a - b)),
            x => anyhow::bail!("Subtraction is not supported for given value types {x:?}"),
        }
    }

    pub fn mul(&self, other: &InterpretValue) -> anyhow::Result<InterpretValue> {
        match (self, other) {
            (InterpretValue::I8(a), InterpretValue::I8(b)) => Ok(InterpretValue::I8(a * b)),
            (InterpretValue::I16(a), InterpretValue::I16(b)) => Ok(InterpretValue::I16(a * b)),
            (InterpretValue::I32(a), InterpretValue::I32(b)) => Ok(InterpretValue::I32(a * b)),
            (InterpretValue::I64(a), InterpretValue::I64(b)) => Ok(InterpretValue::I64(a * b)),
            (InterpretValue::ISize(a), InterpretValue::ISize(b)) => {
                Ok(InterpretValue::ISize(a * b))
            }
            (InterpretValue::U8(a), InterpretValue::U8(b)) => Ok(InterpretValue::U8(a * b)),
            (InterpretValue::U16(a), InterpretValue::U16(b)) => Ok(InterpretValue::U16(a * b)),
            (InterpretValue::U32(a), InterpretValue::U32(b)) => Ok(InterpretValue::U32(a * b)),
            (InterpretValue::U64(a), InterpretValue::U64(b)) => Ok(InterpretValue::U64(a * b)),
            (InterpretValue::USize(a), InterpretValue::USize(b)) => {
                Ok(InterpretValue::USize(a * b))
            }
            (InterpretValue::F32(a), InterpretValue::F32(b)) => Ok(InterpretValue::F32(a * b)),
            (InterpretValue::F64(a), InterpretValue::F64(b)) => Ok(InterpretValue::F64(a * b)),
            x => anyhow::bail!("Multiplication is not supported for given value types {x:?}"),
        }
    }

    pub fn div(&self, other: &InterpretValue) -> anyhow::Result<InterpretValue> {
        match (self, other) {
            (InterpretValue::I8(a), InterpretValue::I8(b)) => Ok(InterpretValue::I8(a / b)),
            (InterpretValue::I16(a), InterpretValue::I16(b)) => Ok(InterpretValue::I16(a / b)),
            (InterpretValue::I32(a), InterpretValue::I32(b)) => Ok(InterpretValue::I32(a / b)),
            (InterpretValue::I64(a), InterpretValue::I64(b)) => Ok(InterpretValue::I64(a / b)),
            (InterpretValue::ISize(a), InterpretValue::ISize(b)) => {
                Ok(InterpretValue::ISize(a / b))
            }
            (InterpretValue::U8(a), InterpretValue::U8(b)) => Ok(InterpretValue::U8(a / b)),
            (InterpretValue::U16(a), InterpretValue::U16(b)) => Ok(InterpretValue::U16(a / b)),
            (InterpretValue::U32(a), InterpretValue::U32(b)) => Ok(InterpretValue::U32(a / b)),
            (InterpretValue::U64(a), InterpretValue::U64(b)) => Ok(InterpretValue::U64(a / b)),
            (InterpretValue::USize(a), InterpretValue::USize(b)) => {
                Ok(InterpretValue::USize(a / b))
            }
            (InterpretValue::F32(a), InterpretValue::F32(b)) => Ok(InterpretValue::F32(a / b)),
            (InterpretValue::F64(a), InterpretValue::F64(b)) => Ok(InterpretValue::F64(a / b)),
            x => anyhow::bail!("Division is not supported for given value types {x:?}"),
        }
    }

    pub fn rem(&self, other: &InterpretValue) -> anyhow::Result<InterpretValue> {
        match (self, other) {
            (InterpretValue::I8(a), InterpretValue::I8(b)) => Ok(InterpretValue::I8(a % b)),
            (InterpretValue::I16(a), InterpretValue::I16(b)) => Ok(InterpretValue::I16(a % b)),
            (InterpretValue::I32(a), InterpretValue::I32(b)) => Ok(InterpretValue::I32(a % b)),
            (InterpretValue::I64(a), InterpretValue::I64(b)) => Ok(InterpretValue::I64(a % b)),
            (InterpretValue::ISize(a), InterpretValue::ISize(b)) => {
                Ok(InterpretValue::ISize(a % b))
            }
            (InterpretValue::U8(a), InterpretValue::U8(b)) => Ok(InterpretValue::U8(a % b)),
            (InterpretValue::U16(a), InterpretValue::U16(b)) => Ok(InterpretValue::U16(a % b)),
            (InterpretValue::U32(a), InterpretValue::U32(b)) => Ok(InterpretValue::U32(a % b)),
            (InterpretValue::U64(a), InterpretValue::U64(b)) => Ok(InterpretValue::U64(a % b)),
            (InterpretValue::USize(a), InterpretValue::USize(b)) => {
                Ok(InterpretValue::USize(a % b))
            }
            x => anyhow::bail!("Modulus is not supported for given value types {x:?}"),
        }
    }

    pub fn eq(&self, other: &InterpretValue) -> anyhow::Result<InterpretValue> {
        match (self, other) {
            (InterpretValue::I8(a), InterpretValue::I8(b)) => Ok(InterpretValue::Boolean(a == b)),
            (InterpretValue::I16(a), InterpretValue::I16(b)) => Ok(InterpretValue::Boolean(a == b)),
            (InterpretValue::I32(a), InterpretValue::I32(b)) => Ok(InterpretValue::Boolean(a == b)),
            (InterpretValue::I64(a), InterpretValue::I64(b)) => Ok(InterpretValue::Boolean(a == b)),
            (InterpretValue::ISize(a), InterpretValue::ISize(b)) => {
                Ok(InterpretValue::Boolean(a == b))
            }
            (InterpretValue::U8(a), InterpretValue::U8(b)) => Ok(InterpretValue::Boolean(a == b)),
            (InterpretValue::U16(a), InterpretValue::U16(b)) => Ok(InterpretValue::Boolean(a == b)),
            (InterpretValue::U32(a), InterpretValue::U32(b)) => Ok(InterpretValue::Boolean(a == b)),
            (InterpretValue::U64(a), InterpretValue::U64(b)) => Ok(InterpretValue::Boolean(a == b)),
            (InterpretValue::USize(a), InterpretValue::USize(b)) => {
                Ok(InterpretValue::Boolean(a == b))
            }
            (InterpretValue::F32(a), InterpretValue::F32(b)) => Ok(InterpretValue::Boolean(a == b)),
            (InterpretValue::F64(a), InterpretValue::F64(b)) => Ok(InterpretValue::Boolean(a == b)),
            (InterpretValue::Boolean(a), InterpretValue::Boolean(b)) => {
                Ok(InterpretValue::Boolean(a == b))
            }
            (InterpretValue::String(a), InterpretValue::String(b)) => {
                Ok(InterpretValue::Boolean(a == b))
            }
            x => anyhow::bail!("Equality comparison is not supported for given value types {x:?}"),
        }
    }

    pub fn neq(&self, other: &InterpretValue) -> anyhow::Result<InterpretValue> {
        match (self, other) {
            (InterpretValue::I8(a), InterpretValue::I8(b)) => Ok(InterpretValue::Boolean(a != b)),
            (InterpretValue::I16(a), InterpretValue::I16(b)) => Ok(InterpretValue::Boolean(a != b)),
            (InterpretValue::I32(a), InterpretValue::I32(b)) => Ok(InterpretValue::Boolean(a != b)),
            (InterpretValue::I64(a), InterpretValue::I64(b)) => Ok(InterpretValue::Boolean(a != b)),
            (InterpretValue::ISize(a), InterpretValue::ISize(b)) => {
                Ok(InterpretValue::Boolean(a != b))
            }
            (InterpretValue::U8(a), InterpretValue::U8(b)) => Ok(InterpretValue::Boolean(a != b)),
            (InterpretValue::U16(a), InterpretValue::U16(b)) => Ok(InterpretValue::Boolean(a != b)),
            (InterpretValue::U32(a), InterpretValue::U32(b)) => Ok(InterpretValue::Boolean(a != b)),
            (InterpretValue::U64(a), InterpretValue::U64(b)) => Ok(InterpretValue::Boolean(a != b)),
            (InterpretValue::USize(a), InterpretValue::USize(b)) => {
                Ok(InterpretValue::Boolean(a != b))
            }
            (InterpretValue::F32(a), InterpretValue::F32(b)) => Ok(InterpretValue::Boolean(a != b)),
            (InterpretValue::F64(a), InterpretValue::F64(b)) => Ok(InterpretValue::Boolean(a != b)),
            (InterpretValue::Boolean(a), InterpretValue::Boolean(b)) => {
                Ok(InterpretValue::Boolean(a != b))
            }
            (InterpretValue::String(a), InterpretValue::String(b)) => {
                Ok(InterpretValue::Boolean(a != b))
            }
            x => {
                anyhow::bail!("Inequality comparison is not supported for given value types {x:?}")
            }
        }
    }

    pub fn gt(&self, other: &InterpretValue) -> anyhow::Result<InterpretValue> {
        match (self, other) {
            (InterpretValue::I8(a), InterpretValue::I8(b)) => Ok(InterpretValue::Boolean(a > b)),
            (InterpretValue::I16(a), InterpretValue::I16(b)) => Ok(InterpretValue::Boolean(a > b)),
            (InterpretValue::I32(a), InterpretValue::I32(b)) => Ok(InterpretValue::Boolean(a > b)),
            (InterpretValue::I64(a), InterpretValue::I64(b)) => Ok(InterpretValue::Boolean(a > b)),
            (InterpretValue::ISize(a), InterpretValue::ISize(b)) => {
                Ok(InterpretValue::Boolean(a > b))
            }
            (InterpretValue::U8(a), InterpretValue::U8(b)) => Ok(InterpretValue::Boolean(a > b)),
            (InterpretValue::U16(a), InterpretValue::U16(b)) => Ok(InterpretValue::Boolean(a > b)),
            (InterpretValue::U32(a), InterpretValue::U32(b)) => Ok(InterpretValue::Boolean(a > b)),
            (InterpretValue::U64(a), InterpretValue::U64(b)) => Ok(InterpretValue::Boolean(a > b)),
            (InterpretValue::USize(a), InterpretValue::USize(b)) => {
                Ok(InterpretValue::Boolean(a > b))
            }
            (InterpretValue::F32(a), InterpretValue::F32(b)) => Ok(InterpretValue::Boolean(a > b)),
            (InterpretValue::F64(a), InterpretValue::F64(b)) => Ok(InterpretValue::Boolean(a > b)),
            x => anyhow::bail!(
                "Greater-than comparison is not supported for given value types {x:?}"
            ),
        }
    }

    pub fn lt(&self, other: &InterpretValue) -> anyhow::Result<InterpretValue> {
        match (self, other) {
            (InterpretValue::I8(a), InterpretValue::I8(b)) => Ok(InterpretValue::Boolean(a < b)),
            (InterpretValue::I16(a), InterpretValue::I16(b)) => Ok(InterpretValue::Boolean(a < b)),
            (InterpretValue::I32(a), InterpretValue::I32(b)) => Ok(InterpretValue::Boolean(a < b)),
            (InterpretValue::I64(a), InterpretValue::I64(b)) => Ok(InterpretValue::Boolean(a < b)),
            (InterpretValue::ISize(a), InterpretValue::ISize(b)) => {
                Ok(InterpretValue::Boolean(a < b))
            }
            (InterpretValue::U8(a), InterpretValue::U8(b)) => Ok(InterpretValue::Boolean(a < b)),
            (InterpretValue::U16(a), InterpretValue::U16(b)) => Ok(InterpretValue::Boolean(a < b)),
            (InterpretValue::U32(a), InterpretValue::U32(b)) => Ok(InterpretValue::Boolean(a < b)),
            (InterpretValue::U64(a), InterpretValue::U64(b)) => Ok(InterpretValue::Boolean(a < b)),
            (InterpretValue::USize(a), InterpretValue::USize(b)) => {
                Ok(InterpretValue::Boolean(a < b))
            }
            (InterpretValue::F32(a), InterpretValue::F32(b)) => Ok(InterpretValue::Boolean(a < b)),
            (InterpretValue::F64(a), InterpretValue::F64(b)) => Ok(InterpretValue::Boolean(a < b)),
            x => anyhow::bail!("Less-than comparison is not supported for given value types {x:?}"),
        }
    }

    pub fn gte(&self, other: &InterpretValue) -> anyhow::Result<InterpretValue> {
        match (self, other) {
            (InterpretValue::I8(a), InterpretValue::I8(b)) => Ok(InterpretValue::Boolean(a >= b)),
            (InterpretValue::I16(a), InterpretValue::I16(b)) => Ok(InterpretValue::Boolean(a >= b)),
            (InterpretValue::I32(a), InterpretValue::I32(b)) => Ok(InterpretValue::Boolean(a >= b)),
            (InterpretValue::I64(a), InterpretValue::I64(b)) => Ok(InterpretValue::Boolean(a >= b)),
            (InterpretValue::ISize(a), InterpretValue::ISize(b)) => {
                Ok(InterpretValue::Boolean(a >= b))
            }
            (InterpretValue::U8(a), InterpretValue::U8(b)) => Ok(InterpretValue::Boolean(a >= b)),
            (InterpretValue::U16(a), InterpretValue::U16(b)) => Ok(InterpretValue::Boolean(a >= b)),
            (InterpretValue::U32(a), InterpretValue::U32(b)) => Ok(InterpretValue::Boolean(a >= b)),
            (InterpretValue::U64(a), InterpretValue::U64(b)) => Ok(InterpretValue::Boolean(a >= b)),
            (InterpretValue::USize(a), InterpretValue::USize(b)) => {
                Ok(InterpretValue::Boolean(a >= b))
            }
            (InterpretValue::F32(a), InterpretValue::F32(b)) => Ok(InterpretValue::Boolean(a >= b)),
            (InterpretValue::F64(a), InterpretValue::F64(b)) => Ok(InterpretValue::Boolean(a >= b)),
            x => anyhow::bail!(
                "Greater-than-or-equal comparison is not supported for given value types {x:?}"
            ),
        }
    }

    pub fn lte(&self, other: &InterpretValue) -> anyhow::Result<InterpretValue> {
        match (self, other) {
            (InterpretValue::I8(a), InterpretValue::I8(b)) => Ok(InterpretValue::Boolean(a <= b)),
            (InterpretValue::I16(a), InterpretValue::I16(b)) => Ok(InterpretValue::Boolean(a <= b)),
            (InterpretValue::I32(a), InterpretValue::I32(b)) => Ok(InterpretValue::Boolean(a <= b)),
            (InterpretValue::I64(a), InterpretValue::I64(b)) => Ok(InterpretValue::Boolean(a <= b)),
            (InterpretValue::ISize(a), InterpretValue::ISize(b)) => {
                Ok(InterpretValue::Boolean(a <= b))
            }
            (InterpretValue::U8(a), InterpretValue::U8(b)) => Ok(InterpretValue::Boolean(a <= b)),
            (InterpretValue::U16(a), InterpretValue::U16(b)) => Ok(InterpretValue::Boolean(a <= b)),
            (InterpretValue::U32(a), InterpretValue::U32(b)) => Ok(InterpretValue::Boolean(a <= b)),
            (InterpretValue::U64(a), InterpretValue::U64(b)) => Ok(InterpretValue::Boolean(a <= b)),
            (InterpretValue::USize(a), InterpretValue::USize(b)) => {
                Ok(InterpretValue::Boolean(a <= b))
            }
            (InterpretValue::F32(a), InterpretValue::F32(b)) => Ok(InterpretValue::Boolean(a <= b)),
            (InterpretValue::F64(a), InterpretValue::F64(b)) => Ok(InterpretValue::Boolean(a <= b)),
            _ => anyhow::bail!(
                "Less-than-or-equal comparison is not supported for given value types"
            ),
        }
    }

    pub fn and(&self, other: &InterpretValue) -> anyhow::Result<InterpretValue> {
        match (self, other) {
            (InterpretValue::Boolean(a), InterpretValue::Boolean(b)) => {
                Ok(InterpretValue::Boolean(*a && *b))
            }
            _ => anyhow::bail!("Logical AND is not supported for given value types"),
        }
    }

    pub fn or(&self, other: &InterpretValue) -> anyhow::Result<InterpretValue> {
        match (self, other) {
            (InterpretValue::Boolean(a), InterpretValue::Boolean(b)) => {
                Ok(InterpretValue::Boolean(*a || *b))
            }
            _ => anyhow::bail!("Logical OR is not supported for given value types"),
        }
    }

    pub fn not(&self) -> anyhow::Result<InterpretValue> {
        match self {
            InterpretValue::Boolean(a) => Ok(InterpretValue::Boolean(!a)),
            _ => anyhow::bail!("Logical NOT is not supported for given value type"),
        }
    }

    pub fn bitand(&self, other: &InterpretValue) -> anyhow::Result<InterpretValue> {
        match (self, other) {
            (InterpretValue::I8(a), InterpretValue::I8(b)) => Ok(InterpretValue::I8(a & b)),
            (InterpretValue::I16(a), InterpretValue::I16(b)) => Ok(InterpretValue::I16(a & b)),
            (InterpretValue::I32(a), InterpretValue::I32(b)) => Ok(InterpretValue::I32(a & b)),
            (InterpretValue::I64(a), InterpretValue::I64(b)) => Ok(InterpretValue::I64(a & b)),
            (InterpretValue::ISize(a), InterpretValue::ISize(b)) => {
                Ok(InterpretValue::ISize(a & b))
            }
            (InterpretValue::U8(a), InterpretValue::U8(b)) => Ok(InterpretValue::U8(a & b)),
            (InterpretValue::U16(a), InterpretValue::U16(b)) => Ok(InterpretValue::U16(a & b)),
            (InterpretValue::U32(a), InterpretValue::U32(b)) => Ok(InterpretValue::U32(a & b)),
            (InterpretValue::U64(a), InterpretValue::U64(b)) => Ok(InterpretValue::U64(a & b)),
            (InterpretValue::USize(a), InterpretValue::USize(b)) => {
                Ok(InterpretValue::USize(a & b))
            }
            _ => anyhow::bail!("Bitwise AND is not supported for given value types"),
        }
    }

    pub fn bitor(&self, other: &InterpretValue) -> anyhow::Result<InterpretValue> {
        match (self, other) {
            (InterpretValue::I8(a), InterpretValue::I8(b)) => Ok(InterpretValue::I8(a | b)),
            (InterpretValue::I16(a), InterpretValue::I16(b)) => Ok(InterpretValue::I16(a | b)),
            (InterpretValue::I32(a), InterpretValue::I32(b)) => Ok(InterpretValue::I32(a | b)),
            (InterpretValue::I64(a), InterpretValue::I64(b)) => Ok(InterpretValue::I64(a | b)),
            (InterpretValue::ISize(a), InterpretValue::ISize(b)) => {
                Ok(InterpretValue::ISize(a | b))
            }
            (InterpretValue::U8(a), InterpretValue::U8(b)) => Ok(InterpretValue::U8(a | b)),
            (InterpretValue::U16(a), InterpretValue::U16(b)) => Ok(InterpretValue::U16(a | b)),
            (InterpretValue::U32(a), InterpretValue::U32(b)) => Ok(InterpretValue::U32(a | b)),
            (InterpretValue::U64(a), InterpretValue::U64(b)) => Ok(InterpretValue::U64(a | b)),
            (InterpretValue::USize(a), InterpretValue::USize(b)) => {
                Ok(InterpretValue::USize(a | b))
            }
            _ => anyhow::bail!("Bitwise OR is not supported for given value types"),
        }
    }

    pub fn bitnot(&self) -> anyhow::Result<InterpretValue> {
        match self {
            InterpretValue::I8(a) => Ok(InterpretValue::I8(!a)),
            InterpretValue::I16(a) => Ok(InterpretValue::I16(!a)),
            InterpretValue::I32(a) => Ok(InterpretValue::I32(!a)),
            InterpretValue::I64(a) => Ok(InterpretValue::I64(!a)),
            InterpretValue::ISize(a) => Ok(InterpretValue::ISize(!a)),
            InterpretValue::U8(a) => Ok(InterpretValue::U8(!a)),
            InterpretValue::U16(a) => Ok(InterpretValue::U16(!a)),
            InterpretValue::U32(a) => Ok(InterpretValue::U32(!a)),
            InterpretValue::U64(a) => Ok(InterpretValue::U64(!a)),
            InterpretValue::USize(a) => Ok(InterpretValue::USize(!a)),
            _ => anyhow::bail!("Bitwise NOT is not supported for given value type"),
        }
    }

    pub fn neg(&self) -> anyhow::Result<InterpretValue> {
        match self {
            InterpretValue::I8(a) => Ok(InterpretValue::I8(-a)),
            InterpretValue::I16(a) => Ok(InterpretValue::I16(-a)),
            InterpretValue::I32(a) => Ok(InterpretValue::I32(-a)),
            InterpretValue::I64(a) => Ok(InterpretValue::I64(-a)),
            InterpretValue::ISize(a) => Ok(InterpretValue::ISize(-a)),
            InterpretValue::F32(a) => Ok(InterpretValue::F32(-a)),
            InterpretValue::F64(a) => Ok(InterpretValue::F64(-a)),
            _ => anyhow::bail!("Negation is not supported for given value type"),
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            InterpretValue::I8(v) => v.to_string(),
            InterpretValue::I16(v) => v.to_string(),
            InterpretValue::I32(v) => v.to_string(),
            InterpretValue::I64(v) => v.to_string(),
            InterpretValue::ISize(v) => v.to_string(),
            InterpretValue::U8(v) => v.to_string(),
            InterpretValue::U16(v) => v.to_string(),
            InterpretValue::U32(v) => v.to_string(),
            InterpretValue::U64(v) => v.to_string(),
            InterpretValue::USize(v) => v.to_string(),
            InterpretValue::F32(v) => v.to_string(),
            InterpretValue::F64(v) => v.to_string(),
            InterpretValue::Boolean(v) => v.to_string(),
            InterpretValue::String(v) => v.clone(),
            InterpretValue::Void => "void".to_string(),
        }
    }

    pub fn as_integer(&self) -> isize {
        match self {
            InterpretValue::I8(v) => *v as isize,
            InterpretValue::I16(v) => *v as isize,
            InterpretValue::I32(v) => *v as isize,
            InterpretValue::I64(v) => *v as isize,
            InterpretValue::ISize(v) => *v,
            InterpretValue::U8(v) => *v as isize,
            InterpretValue::U16(v) => *v as isize,
            InterpretValue::U32(v) => *v as isize,
            InterpretValue::U64(v) => *v as isize,
            InterpretValue::USize(v) => *v as isize,
            _ => 0,
        }
    }
}

#[derive(Clone)]
pub enum Function {
    Interpreted {
        params: Vec<Param>,
        body: Box<Statement>,
    },
    Native {
        func: Arc<dyn NativeFunction>,
    },
}

pub trait NativeFunction: Send + Sync + 'static {
    fn call(&self, args: Vec<InterpretValue>) -> anyhow::Result<InterpretValue>;
}

impl<F> NativeFunction for F
where
    F: Fn(Vec<InterpretValue>) -> anyhow::Result<InterpretValue> + Send + Sync + 'static,
{
    fn call(&self, args: Vec<InterpretValue>) -> anyhow::Result<InterpretValue> {
        (self)(args)
    }
}

pub struct Environment {
    scopes: Vec<HashMap<String, InterpretValue>>,
    functions: HashMap<String, Function>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            scopes: vec![HashMap::new()],
            functions: HashMap::new(),
        }
    }

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    fn set(&mut self, name: String, value: InterpretValue) -> anyhow::Result<()> {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, value);
            Ok(())
        } else {
            anyhow::bail!("No scope available to set variable");
        }
    }

    fn get(&self, name: &str) -> anyhow::Result<InterpretValue> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Ok(value.clone());
            }
        }
        anyhow::bail!("Variable '{}' not found", name);
    }

    fn get_mut(&mut self, name: &str) -> anyhow::Result<&mut InterpretValue> {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(value) = scope.get_mut(name) {
                return Ok(value);
            }
        }
        anyhow::bail!("Variable '{}' not found", name);
    }

    fn update(&mut self, name: String, value: InterpretValue) -> anyhow::Result<()> {
        *self.get_mut(&name)? = value;
        Ok(())
    }

    fn define_function(
        &mut self,
        name: String,
        params: Vec<Param>,
        body: Box<Statement>,
    ) -> anyhow::Result<()> {
        if self.functions.contains_key(&name) {
            anyhow::bail!("Function '{}' is already defined", name);
        }
        self.functions
            .insert(name, Function::Interpreted { params, body });
        Ok(())
    }

    fn define_rust_function<F>(&mut self, name: String, func: F) -> anyhow::Result<()>
    where
        F: Fn(Vec<InterpretValue>) -> anyhow::Result<InterpretValue> + Send + Sync + 'static,
    {
        if self.functions.contains_key(&name) {
            anyhow::bail!("Function '{}' already defined", name);
        }

        self.functions.insert(
            name,
            Function::Native {
                func: Arc::new(func),
            },
        );

        Ok(())
    }

    fn get_function(&self, name: &str) -> anyhow::Result<&Function> {
        if let Some(function) = self.functions.get(name) {
            Ok(function)
        } else {
            anyhow::bail!("Function '{}' not found", name);
        }
    }
}

#[derive(Debug, Clone)]
pub enum ControlFlow {
    None,
    Return(InterpretValue),
    Break,
    Continue,
}

pub struct Interpreter {
    env: Environment,
}

macro_rules! native_func {
    ($env:expr, $name:ident) => {
        $env.define_rust_function(
            stringify!($name).to_string(),
            crate::native_functions::$name,
        )
        .unwrap();
    };
}

impl Interpreter {
    pub fn new() -> Self {
        let mut env = Environment::new();
        native_func!(env, print);
        native_func!(env, exit);
        Interpreter { env }
    }

    fn exec_stmt(&mut self, statement: &Statement) -> anyhow::Result<ControlFlow> {
        match &statement.stmt {
            Stmt::Let { name, value, .. } => {
                let (val, _) = self.eval_expr(value)?;
                self.env.set(name.clone(), val)?;
                Ok(ControlFlow::None)
            }
            Stmt::Func {
                name, params, body, ..
            } => {
                self.env
                    .define_function(name.clone(), params.clone(), body.clone())?;
                Ok(ControlFlow::None)
            }
            Stmt::Expr(expr) => {
                let (_, flow) = self.eval_expr(expr)?;
                Ok(flow)
            }
            Stmt::Scope { statements } => {
                self.env.push_scope();
                let mut result = ControlFlow::None;
                for stmt in statements {
                    result = self.exec_stmt(stmt)?;
                    match result {
                        ControlFlow::Return(_) | ControlFlow::Break | ControlFlow::Continue => {
                            break;
                        }
                        ControlFlow::None => {}
                    }
                }
                self.env.pop_scope();
                Ok(result)
            }
            Stmt::While { condition, body } => {
                loop {
                    let (cond, _) = self.eval_expr(condition)?;
                    if matches!(cond, InterpretValue::Boolean(false)) {
                        break;
                    }
                    match self.exec_stmt(body)? {
                        ControlFlow::Break => break,
                        ControlFlow::Continue => continue,
                        ControlFlow::Return(val) => return Ok(ControlFlow::Return(val)),
                        ControlFlow::None => {}
                    }
                }
                Ok(ControlFlow::None)
            }
            Stmt::Return { value } => {
                let val = if let Some(expr) = value {
                    let (v, _) = self.eval_expr(expr)?;
                    v
                } else {
                    InterpretValue::Void
                };
                Ok(ControlFlow::Return(val))
            }
            Stmt::Break => Ok(ControlFlow::Break),
            Stmt::Continue => Ok(ControlFlow::Continue),
        }
    }

    fn eval_expr(&mut self, expr: &Expr) -> anyhow::Result<(InterpretValue, ControlFlow)> {
        match expr {
            Expr::Literal(lit) => {
                Ok((InterpretValue::from_literal(lit.clone()), ControlFlow::None))
            }
            Expr::Variable(name) => Ok((self.env.get(name)?, ControlFlow::None)),
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let (left_val, _) = self.eval_expr(left)?;
                let (right_val, _) = self.eval_expr(right)?;
                let result = match operator {
                    Operator::Plus => left_val.add(&right_val),
                    Operator::Minus => left_val.sub(&right_val),
                    Operator::Multiply => left_val.mul(&right_val),
                    Operator::Divide => left_val.div(&right_val),
                    Operator::Modulus => left_val.rem(&right_val),
                    Operator::Equals => left_val.eq(&right_val),
                    Operator::NotEquals => left_val.neq(&right_val),
                    Operator::Greater => left_val.gt(&right_val),
                    Operator::Less => left_val.lt(&right_val),
                    Operator::GreaterEquals => left_val.gte(&right_val),
                    Operator::LessEquals => left_val.lte(&right_val),
                    Operator::LogicalAnd => left_val.and(&right_val),
                    Operator::LogicalOr => left_val.or(&right_val),
                    Operator::BitAnd => left_val.bitand(&right_val),
                    Operator::BitOr => left_val.bitor(&right_val),
                    _ => anyhow::bail!("Unknown binary operator '{:?}'", operator),
                }?;
                Ok((result, ControlFlow::None))
            }
            Expr::Unary { operator, operand } => {
                let (operand_val, _) = self.eval_expr(operand)?;
                let result = match operator {
                    Operator::Minus => operand_val.neg(),
                    Operator::LogicalNot => operand_val.not(),
                    Operator::BitNot => operand_val.bitnot(),
                    _ => anyhow::bail!("Unknown unary operator '{:?}'", operator),
                }?;
                Ok((result, ControlFlow::None))
            }
            Expr::Assignment { target, value } => {
                let target = if let Expr::Variable(name) = target.as_ref() {
                    name.clone()
                } else {
                    anyhow::bail!("Invalid assignment target");
                };
                let (val, _) = self.eval_expr(value)?;
                self.env.update(target, val.clone())?;
                Ok((val, ControlFlow::None))
            }
            Expr::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let (cond, _) = self.eval_expr(condition)?;
                if matches!(cond, InterpretValue::Boolean(true)) {
                    self.env.push_scope();
                    let mut result = ControlFlow::None;
                    for stmt in then_branch {
                        result = self.exec_stmt(stmt)?;
                        match result {
                            ControlFlow::Return(_) | ControlFlow::Break | ControlFlow::Continue => {
                                break;
                            }
                            ControlFlow::None => {}
                        }
                    }
                    self.env.pop_scope();
                    Ok((InterpretValue::Void, result))
                } else if let Some(else_branch) = else_branch {
                    self.env.push_scope();
                    let mut result = ControlFlow::None;
                    for stmt in else_branch {
                        result = self.exec_stmt(stmt)?;
                        match result {
                            ControlFlow::Return(_) | ControlFlow::Break | ControlFlow::Continue => {
                                break;
                            }
                            ControlFlow::None => {}
                        }
                    }
                    self.env.pop_scope();
                    Ok((InterpretValue::Void, result))
                } else {
                    Ok((InterpretValue::Void, ControlFlow::None))
                }
            }
            Expr::FunctionCall { name, arguments } => {
                let function = self.env.get_function(name)?.clone();

                match function {
                    Function::Native { func } => {
                        let mut args = Vec::new();
                        for arg_expr in arguments {
                            let (arg_value, _) = self.eval_expr(arg_expr)?;
                            args.push(arg_value);
                        }
                        let result = func.call(args)?;
                        Ok((result, ControlFlow::None))
                    }
                    Function::Interpreted { params, body } => {
                        if arguments.len() != params.len() {
                            anyhow::bail!(
                                "Function '{}' expected {} arguments but got {}",
                                name,
                                params.len(),
                                arguments.len()
                            );
                        }
                        self.env.push_scope();
                        for (param, arg_expr) in params.iter().zip(arguments.iter()) {
                            let (arg_value, _) = self.eval_expr(arg_expr)?;
                            self.env.set(param.name.clone(), arg_value)?;
                        }
                        let control_flow = match &body.stmt {
                            Stmt::Scope { statements } => {
                                let mut result = ControlFlow::None;
                                for stmt in statements {
                                    result = self.exec_stmt(stmt)?;
                                    match result {
                                        ControlFlow::Return(_)
                                        | ControlFlow::Break
                                        | ControlFlow::Continue => break,
                                        ControlFlow::None => {}
                                    }
                                }
                                result
                            }
                            _ => self.exec_stmt(&body)?,
                        };

                        self.env.pop_scope();
                        match control_flow {
                            ControlFlow::Return(val) => Ok((val, ControlFlow::None)),
                            _ => Ok((InterpretValue::Void, ControlFlow::None)),
                        }
                    }
                }
            }
        }
    }

    pub fn interpret(&mut self, statements: &[Statement]) -> anyhow::Result<()> {
        for stmt in statements {
            self.exec_stmt(stmt)
                .map_err(|e| anyhow::anyhow!("{} at {:?}", e, stmt.location))?;
        }
        Ok(())
    }
}
