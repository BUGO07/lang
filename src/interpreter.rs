use std::collections::HashMap;

use crate::{
    parser::{Expr, Param, Statement, Stmt},
    token::{Literal, NumericType, Operator},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
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

impl Value {
    pub fn from_literal(lit: Literal) -> Self {
        match lit {
            Literal::Numeric(value, num_type) => match num_type {
                NumericType::I8 => Value::I8(value.parse().unwrap()),
                NumericType::I16 => Value::I16(value.parse().unwrap()),
                NumericType::I32 => Value::I32(value.parse().unwrap()),
                NumericType::I64 => Value::I64(value.parse().unwrap()),
                NumericType::ISize => Value::ISize(value.parse().unwrap()),
                NumericType::U8 => Value::U8(value.parse().unwrap()),
                NumericType::U16 => Value::U16(value.parse().unwrap()),
                NumericType::U32 => Value::U32(value.parse().unwrap()),
                NumericType::U64 => Value::U64(value.parse().unwrap()),
                NumericType::USize => Value::USize(value.parse().unwrap()),
                NumericType::F32 => Value::F32(value.parse().unwrap()),
                NumericType::F64 => Value::F64(value.parse().unwrap()),
            },
            Literal::String(value) => Value::String(value),
            Literal::Boolean(value) => Value::Boolean(value == "true"),
        }
    }

    pub fn add(&self, other: &Value) -> anyhow::Result<Value> {
        match (self, other) {
            (Value::I8(a), Value::I8(b)) => Ok(Value::I8(a + b)),
            (Value::I16(a), Value::I16(b)) => Ok(Value::I16(a + b)),
            (Value::I32(a), Value::I32(b)) => Ok(Value::I32(a + b)),
            (Value::I64(a), Value::I64(b)) => Ok(Value::I64(a + b)),
            (Value::ISize(a), Value::ISize(b)) => Ok(Value::ISize(a + b)),
            (Value::U8(a), Value::U8(b)) => Ok(Value::U8(a + b)),
            (Value::U16(a), Value::U16(b)) => Ok(Value::U16(a + b)),
            (Value::U32(a), Value::U32(b)) => Ok(Value::U32(a + b)),
            (Value::U64(a), Value::U64(b)) => Ok(Value::U64(a + b)),
            (Value::USize(a), Value::USize(b)) => Ok(Value::USize(a + b)),
            (Value::F32(a), Value::F32(b)) => Ok(Value::F32(a + b)),
            (Value::F64(a), Value::F64(b)) => Ok(Value::F64(a + b)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            _ => anyhow::bail!("Addition is not supported for given value types"),
        }
    }

    pub fn sub(&self, other: &Value) -> anyhow::Result<Value> {
        match (self, other) {
            (Value::I8(a), Value::I8(b)) => Ok(Value::I8(a - b)),
            (Value::I16(a), Value::I16(b)) => Ok(Value::I16(a - b)),
            (Value::I32(a), Value::I32(b)) => Ok(Value::I32(a - b)),
            (Value::I64(a), Value::I64(b)) => Ok(Value::I64(a - b)),
            (Value::ISize(a), Value::ISize(b)) => Ok(Value::ISize(a - b)),
            (Value::U8(a), Value::U8(b)) => Ok(Value::U8(a - b)),
            (Value::U16(a), Value::U16(b)) => Ok(Value::U16(a - b)),
            (Value::U32(a), Value::U32(b)) => Ok(Value::U32(a - b)),
            (Value::U64(a), Value::U64(b)) => Ok(Value::U64(a - b)),
            (Value::USize(a), Value::USize(b)) => Ok(Value::USize(a - b)),
            (Value::F32(a), Value::F32(b)) => Ok(Value::F32(a - b)),
            (Value::F64(a), Value::F64(b)) => Ok(Value::F64(a - b)),
            _ => anyhow::bail!("Subtraction is not supported for given value types"),
        }
    }

    pub fn mul(&self, other: &Value) -> anyhow::Result<Value> {
        match (self, other) {
            (Value::I8(a), Value::I8(b)) => Ok(Value::I8(a * b)),
            (Value::I16(a), Value::I16(b)) => Ok(Value::I16(a * b)),
            (Value::I32(a), Value::I32(b)) => Ok(Value::I32(a * b)),
            (Value::I64(a), Value::I64(b)) => Ok(Value::I64(a * b)),
            (Value::ISize(a), Value::ISize(b)) => Ok(Value::ISize(a * b)),
            (Value::U8(a), Value::U8(b)) => Ok(Value::U8(a * b)),
            (Value::U16(a), Value::U16(b)) => Ok(Value::U16(a * b)),
            (Value::U32(a), Value::U32(b)) => Ok(Value::U32(a * b)),
            (Value::U64(a), Value::U64(b)) => Ok(Value::U64(a * b)),
            (Value::USize(a), Value::USize(b)) => Ok(Value::USize(a * b)),
            (Value::F32(a), Value::F32(b)) => Ok(Value::F32(a * b)),
            (Value::F64(a), Value::F64(b)) => Ok(Value::F64(a * b)),
            _ => anyhow::bail!("Multiplication is not supported for given value types"),
        }
    }

    pub fn div(&self, other: &Value) -> anyhow::Result<Value> {
        match (self, other) {
            (Value::I8(a), Value::I8(b)) => Ok(Value::I8(a / b)),
            (Value::I16(a), Value::I16(b)) => Ok(Value::I16(a / b)),
            (Value::I32(a), Value::I32(b)) => Ok(Value::I32(a / b)),
            (Value::I64(a), Value::I64(b)) => Ok(Value::I64(a / b)),
            (Value::ISize(a), Value::ISize(b)) => Ok(Value::ISize(a / b)),
            (Value::U8(a), Value::U8(b)) => Ok(Value::U8(a / b)),
            (Value::U16(a), Value::U16(b)) => Ok(Value::U16(a / b)),
            (Value::U32(a), Value::U32(b)) => Ok(Value::U32(a / b)),
            (Value::U64(a), Value::U64(b)) => Ok(Value::U64(a / b)),
            (Value::USize(a), Value::USize(b)) => Ok(Value::USize(a / b)),
            (Value::F32(a), Value::F32(b)) => Ok(Value::F32(a / b)),
            (Value::F64(a), Value::F64(b)) => Ok(Value::F64(a / b)),
            _ => anyhow::bail!("Division is not supported for given value types"),
        }
    }

    pub fn rem(&self, other: &Value) -> anyhow::Result<Value> {
        match (self, other) {
            (Value::I8(a), Value::I8(b)) => Ok(Value::I8(a % b)),
            (Value::I16(a), Value::I16(b)) => Ok(Value::I16(a % b)),
            (Value::I32(a), Value::I32(b)) => Ok(Value::I32(a % b)),
            (Value::I64(a), Value::I64(b)) => Ok(Value::I64(a % b)),
            (Value::ISize(a), Value::ISize(b)) => Ok(Value::ISize(a % b)),
            (Value::U8(a), Value::U8(b)) => Ok(Value::U8(a % b)),
            (Value::U16(a), Value::U16(b)) => Ok(Value::U16(a % b)),
            (Value::U32(a), Value::U32(b)) => Ok(Value::U32(a % b)),
            (Value::U64(a), Value::U64(b)) => Ok(Value::U64(a % b)),
            (Value::USize(a), Value::USize(b)) => Ok(Value::USize(a % b)),
            _ => anyhow::bail!("Modulus is not supported for given value types"),
        }
    }

    pub fn eq(&self, other: &Value) -> anyhow::Result<Value> {
        match (self, other) {
            (Value::I8(a), Value::I8(b)) => Ok(Value::Boolean(a == b)),
            (Value::I16(a), Value::I16(b)) => Ok(Value::Boolean(a == b)),
            (Value::I32(a), Value::I32(b)) => Ok(Value::Boolean(a == b)),
            (Value::I64(a), Value::I64(b)) => Ok(Value::Boolean(a == b)),
            (Value::ISize(a), Value::ISize(b)) => Ok(Value::Boolean(a == b)),
            (Value::U8(a), Value::U8(b)) => Ok(Value::Boolean(a == b)),
            (Value::U16(a), Value::U16(b)) => Ok(Value::Boolean(a == b)),
            (Value::U32(a), Value::U32(b)) => Ok(Value::Boolean(a == b)),
            (Value::U64(a), Value::U64(b)) => Ok(Value::Boolean(a == b)),
            (Value::USize(a), Value::USize(b)) => Ok(Value::Boolean(a == b)),
            (Value::F32(a), Value::F32(b)) => Ok(Value::Boolean(a == b)),
            (Value::F64(a), Value::F64(b)) => Ok(Value::Boolean(a == b)),
            (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(a == b)),
            (Value::String(a), Value::String(b)) => Ok(Value::Boolean(a == b)),
            _ => anyhow::bail!("Equality comparison is not supported for given value types"),
        }
    }

    pub fn neq(&self, other: &Value) -> anyhow::Result<Value> {
        match (self, other) {
            (Value::I8(a), Value::I8(b)) => Ok(Value::Boolean(a != b)),
            (Value::I16(a), Value::I16(b)) => Ok(Value::Boolean(a != b)),
            (Value::I32(a), Value::I32(b)) => Ok(Value::Boolean(a != b)),
            (Value::I64(a), Value::I64(b)) => Ok(Value::Boolean(a != b)),
            (Value::ISize(a), Value::ISize(b)) => Ok(Value::Boolean(a != b)),
            (Value::U8(a), Value::U8(b)) => Ok(Value::Boolean(a != b)),
            (Value::U16(a), Value::U16(b)) => Ok(Value::Boolean(a != b)),
            (Value::U32(a), Value::U32(b)) => Ok(Value::Boolean(a != b)),
            (Value::U64(a), Value::U64(b)) => Ok(Value::Boolean(a != b)),
            (Value::USize(a), Value::USize(b)) => Ok(Value::Boolean(a != b)),
            (Value::F32(a), Value::F32(b)) => Ok(Value::Boolean(a != b)),
            (Value::F64(a), Value::F64(b)) => Ok(Value::Boolean(a != b)),
            (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(a != b)),
            (Value::String(a), Value::String(b)) => Ok(Value::Boolean(a != b)),
            _ => anyhow::bail!("Inequality comparison is not supported for given value types"),
        }
    }

    pub fn gt(&self, other: &Value) -> anyhow::Result<Value> {
        match (self, other) {
            (Value::I8(a), Value::I8(b)) => Ok(Value::Boolean(a > b)),
            (Value::I16(a), Value::I16(b)) => Ok(Value::Boolean(a > b)),
            (Value::I32(a), Value::I32(b)) => Ok(Value::Boolean(a > b)),
            (Value::I64(a), Value::I64(b)) => Ok(Value::Boolean(a > b)),
            (Value::ISize(a), Value::ISize(b)) => Ok(Value::Boolean(a > b)),
            (Value::U8(a), Value::U8(b)) => Ok(Value::Boolean(a > b)),
            (Value::U16(a), Value::U16(b)) => Ok(Value::Boolean(a > b)),
            (Value::U32(a), Value::U32(b)) => Ok(Value::Boolean(a > b)),
            (Value::U64(a), Value::U64(b)) => Ok(Value::Boolean(a > b)),
            (Value::USize(a), Value::USize(b)) => Ok(Value::Boolean(a > b)),
            (Value::F32(a), Value::F32(b)) => Ok(Value::Boolean(a > b)),
            (Value::F64(a), Value::F64(b)) => Ok(Value::Boolean(a > b)),
            _ => anyhow::bail!("Greater-than comparison is not supported for given value types"),
        }
    }

    pub fn lt(&self, other: &Value) -> anyhow::Result<Value> {
        match (self, other) {
            (Value::I8(a), Value::I8(b)) => Ok(Value::Boolean(a < b)),
            (Value::I16(a), Value::I16(b)) => Ok(Value::Boolean(a < b)),
            (Value::I32(a), Value::I32(b)) => Ok(Value::Boolean(a < b)),
            (Value::I64(a), Value::I64(b)) => Ok(Value::Boolean(a < b)),
            (Value::ISize(a), Value::ISize(b)) => Ok(Value::Boolean(a < b)),
            (Value::U8(a), Value::U8(b)) => Ok(Value::Boolean(a < b)),
            (Value::U16(a), Value::U16(b)) => Ok(Value::Boolean(a < b)),
            (Value::U32(a), Value::U32(b)) => Ok(Value::Boolean(a < b)),
            (Value::U64(a), Value::U64(b)) => Ok(Value::Boolean(a < b)),
            (Value::USize(a), Value::USize(b)) => Ok(Value::Boolean(a < b)),
            (Value::F32(a), Value::F32(b)) => Ok(Value::Boolean(a < b)),
            (Value::F64(a), Value::F64(b)) => Ok(Value::Boolean(a < b)),
            _ => anyhow::bail!("Less-than comparison is not supported for given value types"),
        }
    }

    pub fn gte(&self, other: &Value) -> anyhow::Result<Value> {
        match (self, other) {
            (Value::I8(a), Value::I8(b)) => Ok(Value::Boolean(a >= b)),
            (Value::I16(a), Value::I16(b)) => Ok(Value::Boolean(a >= b)),
            (Value::I32(a), Value::I32(b)) => Ok(Value::Boolean(a >= b)),
            (Value::I64(a), Value::I64(b)) => Ok(Value::Boolean(a >= b)),
            (Value::ISize(a), Value::ISize(b)) => Ok(Value::Boolean(a >= b)),
            (Value::U8(a), Value::U8(b)) => Ok(Value::Boolean(a >= b)),
            (Value::U16(a), Value::U16(b)) => Ok(Value::Boolean(a >= b)),
            (Value::U32(a), Value::U32(b)) => Ok(Value::Boolean(a >= b)),
            (Value::U64(a), Value::U64(b)) => Ok(Value::Boolean(a >= b)),
            (Value::USize(a), Value::USize(b)) => Ok(Value::Boolean(a >= b)),
            (Value::F32(a), Value::F32(b)) => Ok(Value::Boolean(a >= b)),
            (Value::F64(a), Value::F64(b)) => Ok(Value::Boolean(a >= b)),
            _ => anyhow::bail!(
                "Greater-than-or-equal comparison is not supported for given value types"
            ),
        }
    }

    pub fn lte(&self, other: &Value) -> anyhow::Result<Value> {
        match (self, other) {
            (Value::I8(a), Value::I8(b)) => Ok(Value::Boolean(a <= b)),
            (Value::I16(a), Value::I16(b)) => Ok(Value::Boolean(a <= b)),
            (Value::I32(a), Value::I32(b)) => Ok(Value::Boolean(a <= b)),
            (Value::I64(a), Value::I64(b)) => Ok(Value::Boolean(a <= b)),
            (Value::ISize(a), Value::ISize(b)) => Ok(Value::Boolean(a <= b)),
            (Value::U8(a), Value::U8(b)) => Ok(Value::Boolean(a <= b)),
            (Value::U16(a), Value::U16(b)) => Ok(Value::Boolean(a <= b)),
            (Value::U32(a), Value::U32(b)) => Ok(Value::Boolean(a <= b)),
            (Value::U64(a), Value::U64(b)) => Ok(Value::Boolean(a <= b)),
            (Value::USize(a), Value::USize(b)) => Ok(Value::Boolean(a <= b)),
            (Value::F32(a), Value::F32(b)) => Ok(Value::Boolean(a <= b)),
            (Value::F64(a), Value::F64(b)) => Ok(Value::Boolean(a <= b)),
            _ => anyhow::bail!(
                "Less-than-or-equal comparison is not supported for given value types"
            ),
        }
    }

    pub fn and(&self, other: &Value) -> anyhow::Result<Value> {
        match (self, other) {
            (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(*a && *b)),
            _ => anyhow::bail!("Logical AND is not supported for given value types"),
        }
    }

    pub fn or(&self, other: &Value) -> anyhow::Result<Value> {
        match (self, other) {
            (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(*a || *b)),
            _ => anyhow::bail!("Logical OR is not supported for given value types"),
        }
    }

    pub fn not(&self) -> anyhow::Result<Value> {
        match self {
            Value::Boolean(a) => Ok(Value::Boolean(!a)),
            _ => anyhow::bail!("Logical NOT is not supported for given value type"),
        }
    }

    pub fn bitand(&self, other: &Value) -> anyhow::Result<Value> {
        match (self, other) {
            (Value::I8(a), Value::I8(b)) => Ok(Value::I8(a & b)),
            (Value::I16(a), Value::I16(b)) => Ok(Value::I16(a & b)),
            (Value::I32(a), Value::I32(b)) => Ok(Value::I32(a & b)),
            (Value::I64(a), Value::I64(b)) => Ok(Value::I64(a & b)),
            (Value::ISize(a), Value::ISize(b)) => Ok(Value::ISize(a & b)),
            (Value::U8(a), Value::U8(b)) => Ok(Value::U8(a & b)),
            (Value::U16(a), Value::U16(b)) => Ok(Value::U16(a & b)),
            (Value::U32(a), Value::U32(b)) => Ok(Value::U32(a & b)),
            (Value::U64(a), Value::U64(b)) => Ok(Value::U64(a & b)),
            (Value::USize(a), Value::USize(b)) => Ok(Value::USize(a & b)),
            _ => anyhow::bail!("Bitwise AND is not supported for given value types"),
        }
    }

    pub fn bitor(&self, other: &Value) -> anyhow::Result<Value> {
        match (self, other) {
            (Value::I8(a), Value::I8(b)) => Ok(Value::I8(a | b)),
            (Value::I16(a), Value::I16(b)) => Ok(Value::I16(a | b)),
            (Value::I32(a), Value::I32(b)) => Ok(Value::I32(a | b)),
            (Value::I64(a), Value::I64(b)) => Ok(Value::I64(a | b)),
            (Value::ISize(a), Value::ISize(b)) => Ok(Value::ISize(a | b)),
            (Value::U8(a), Value::U8(b)) => Ok(Value::U8(a | b)),
            (Value::U16(a), Value::U16(b)) => Ok(Value::U16(a | b)),
            (Value::U32(a), Value::U32(b)) => Ok(Value::U32(a | b)),
            (Value::U64(a), Value::U64(b)) => Ok(Value::U64(a | b)),
            (Value::USize(a), Value::USize(b)) => Ok(Value::USize(a | b)),
            _ => anyhow::bail!("Bitwise OR is not supported for given value types"),
        }
    }

    pub fn bitnot(&self) -> anyhow::Result<Value> {
        match self {
            Value::I8(a) => Ok(Value::I8(!a)),
            Value::I16(a) => Ok(Value::I16(!a)),
            Value::I32(a) => Ok(Value::I32(!a)),
            Value::I64(a) => Ok(Value::I64(!a)),
            Value::ISize(a) => Ok(Value::ISize(!a)),
            Value::U8(a) => Ok(Value::U8(!a)),
            Value::U16(a) => Ok(Value::U16(!a)),
            Value::U32(a) => Ok(Value::U32(!a)),
            Value::U64(a) => Ok(Value::U64(!a)),
            Value::USize(a) => Ok(Value::USize(!a)),
            _ => anyhow::bail!("Bitwise NOT is not supported for given value type"),
        }
    }

    pub fn neg(&self) -> anyhow::Result<Value> {
        match self {
            Value::I8(a) => Ok(Value::I8(-a)),
            Value::I16(a) => Ok(Value::I16(-a)),
            Value::I32(a) => Ok(Value::I32(-a)),
            Value::I64(a) => Ok(Value::I64(-a)),
            Value::ISize(a) => Ok(Value::ISize(-a)),
            Value::F32(a) => Ok(Value::F32(-a)),
            Value::F64(a) => Ok(Value::F64(-a)),
            _ => anyhow::bail!("Negation is not supported for given value type"),
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            Value::I8(v) => v.to_string(),
            Value::I16(v) => v.to_string(),
            Value::I32(v) => v.to_string(),
            Value::I64(v) => v.to_string(),
            Value::ISize(v) => v.to_string(),
            Value::U8(v) => v.to_string(),
            Value::U16(v) => v.to_string(),
            Value::U32(v) => v.to_string(),
            Value::U64(v) => v.to_string(),
            Value::USize(v) => v.to_string(),
            Value::F32(v) => v.to_string(),
            Value::F64(v) => v.to_string(),
            Value::Boolean(v) => v.to_string(),
            Value::String(v) => v.clone(),
            Value::Void => "void".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    params: Vec<Param>,
    body: Box<Statement>,
}

#[derive(Debug)]
pub struct Environment {
    scopes: Vec<HashMap<String, Value>>,
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

    fn set(&mut self, name: String, value: Value) -> anyhow::Result<()> {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, value);
            Ok(())
        } else {
            anyhow::bail!("No scope available to set variable");
        }
    }

    fn get(&self, name: &str) -> anyhow::Result<Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Ok(value.clone());
            }
        }
        anyhow::bail!("Variable '{}' not found", name);
    }

    fn get_mut(&mut self, name: &str) -> anyhow::Result<&mut Value> {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(value) = scope.get_mut(name) {
                return Ok(value);
            }
        }
        anyhow::bail!("Variable '{}' not found", name);
    }

    fn update(&mut self, name: String, value: Value) -> anyhow::Result<()> {
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
        self.functions.insert(name, Function { params, body });
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
    Return(Value),
    Break,
    Continue,
}

#[derive(Debug)]
pub struct Interpreter {
    env: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            env: Environment::new(),
        }
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
                    if matches!(cond, Value::Boolean(false)) {
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
                    Value::Void
                };
                Ok(ControlFlow::Return(val))
            }
            Stmt::Break => Ok(ControlFlow::Break),
            Stmt::Continue => Ok(ControlFlow::Continue),
        }
    }

    fn eval_expr(&mut self, expr: &Expr) -> anyhow::Result<(Value, ControlFlow)> {
        match expr {
            Expr::Literal(lit) => Ok((Value::from_literal(lit.clone()), ControlFlow::None)),
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
                if matches!(cond, Value::Boolean(true)) {
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
                    Ok((Value::Void, result))
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
                    Ok((Value::Void, result))
                } else {
                    Ok((Value::Void, ControlFlow::None))
                }
            }
            Expr::FunctionCall { name, arguments } => {
                match name.as_str() {
                    "print" => {
                        if let Some(expr) = arguments.first() {
                            let (fmt, _) = self.eval_expr(expr)?;
                            match fmt {
                                Value::String(fmt) => {
                                    let mut output = fmt.clone();
                                    for arg in &arguments[1..] {
                                        let (val, _) = self.eval_expr(arg)?;
                                        output = output.replacen("{}", &val.as_string(), 1);
                                    }
                                    println!("{}", output);
                                }
                                _ => {
                                    println!("{}", fmt.as_string());
                                }
                            }
                            return Ok((Value::Void, ControlFlow::None));
                        }
                    }
                    "exit" => {
                        if let Some(expr) = arguments.first() {
                            let (val, _) = self.eval_expr(expr)?;
                            let code = match val {
                                Value::I8(v) => v as i32,
                                Value::I16(v) => v as i32,
                                Value::I32(v) => v,
                                Value::I64(v) => v as i32,
                                Value::ISize(v) => v as i32,
                                Value::U8(v) => v as i32,
                                Value::U16(v) => v as i32,
                                Value::U32(v) => v as i32,
                                Value::U64(v) => v as i32,
                                Value::USize(v) => v as i32,
                                _ => 0,
                            };
                            std::process::exit(code);
                        } else {
                            std::process::exit(0);
                        }
                    }
                    _ => {}
                }

                let function = self.env.get_function(name)?.clone();
                if arguments.len() != function.params.len() {
                    anyhow::bail!(
                        "Function '{}' expected {} arguments but got {}",
                        name,
                        function.params.len(),
                        arguments.len()
                    );
                }
                self.env.push_scope();
                for (param, arg_expr) in function.params.iter().zip(arguments.iter()) {
                    let (arg_value, _) = self.eval_expr(arg_expr)?;
                    self.env.set(param.name.clone(), arg_value)?;
                }

                let control_flow = match &function.body.stmt {
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
                    _ => self.exec_stmt(&function.body)?,
                };

                self.env.pop_scope();
                match control_flow {
                    ControlFlow::Return(val) => Ok((val, ControlFlow::None)),
                    _ => Ok((Value::Void, ControlFlow::None)),
                }
            }
        }
    }

    pub fn interpret(&mut self, statements: &[Statement]) -> anyhow::Result<()> {
        for stmt in statements {
            self.exec_stmt(stmt)?;
        }
        Ok(())
    }
}
