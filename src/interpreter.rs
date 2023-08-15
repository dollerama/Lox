use std::collections::HashMap;
use std::time::Instant;

use crate::tokens::*;
use crate::types::*;
use crate::environment::*;
use crate::statements::*;
use crate::expressions::*;
use crate::std_lib::*;

pub struct Interpreter {
   pub environment : Environment,
   pub time : Instant
}

impl Interpreter {
    pub fn new() -> Self {
        let mut intial_environment = Environment::new();
        intial_environment.define("clock".to_string(), Some(Literal::Function(Box::new(ClockFunction::new()))));
        intial_environment.define("len".to_string(), Some(Literal::Function(Box::new(LenFunction::new()))));
        intial_environment.define("debug".to_string(), Some(Literal::Function(Box::new(DebugFunction::new()))));
        Self {
            environment : intial_environment,
            time : Instant::now()
        }
    }
    
    pub fn collect_from_block(block : &Box<dyn Stmt>, collection : &mut Vec<Box<dyn Stmt>>) {
        if let Some(v) = block.as_any().downcast_ref::<Block>() {
            for statement in &v.statements {
                Self::collect_from_block(statement, collection)
            }
        }
        else {
            collection.push(block.clone());
        }
    }
    
    pub fn print_helper(value : Option<Literal>, new_line : bool) {
        match value {
            Some(Literal::String(s)) => {
                match new_line {
                    false => print!("{}", s),
                    true => println!("{}", s)
                }
            },
            Some(Literal::Boolean(b)) => {
                match new_line {
                    false => print!("{}", b),
                    true => println!("{}", b)
                }
            },
            Some(Literal::Number(n)) => {
                match new_line {
                    false => print!("{}", n),
                    true => println!("{}", n)
                }
            },
            Some(Literal::StrongString(s)) => {
                match new_line {
                    false => print!("{}", s),
                    true => println!("{}", s)
                }
            },
            Some(Literal::StrongBoolean(b)) => {
                match new_line {
                    false => print!("{}", b),
                    true => println!("{}", b)
                }
            },
            Some(Literal::StrongNumber(n)) => {
                match new_line {
                    false => print!("{}", n),
                    true => println!("{}", n)
                }
            },
            Some(Literal::Function(f)) => {
                match new_line {
                    false => print!("fn => {:#?}", f),
                    true => println!("fn => {:#?}", f)
                }
            },
            Some(Literal::Instance(i)) => {
                match new_line {
                    false => {
                        print!("instance of {} {{", i.class.name.clone());
                        let mut count = 0;
                        for field in &i.fields {
                            print!("{} = ", field.0);
                            Self::print_helper(field.1.clone(), false);
                            if count != i.fields.len()-1 {
                                print!(", ");
                            }
                            count += 1;
                        }
                        print!("}}");
                    }
                    true => {
                        println!("{} {{", i.class.name.clone());
                        for field in &i.fields {
                            print!("  {} = ", field.0);
                            Self::print_helper(field.1.clone(), true);
                        }
                        println!("\n}}");
                    }
                }
            },
            Some(Literal::Collection(n)) => {
                match new_line {
                    false => {
                        print!("[");
                        for i in 0..n.len() {
                            Self::print_helper(*n[i].clone(), false);
                            
                            if i != n.len()-1 {
                                print!(", ");
                            }
                        }
                        print!("]");
                    },
                    true => {
                        print!("[");
                        for i in 0..n.len() {
                            Self::print_helper(*n[i].clone(), false);
                            
                            if i != n.len()-1 {
                                print!(", ");
                            }
                        }
                        print!("]\n");
                    }
                }
            },
            Some(n) => { 
                match new_line {
                    false => print!("{:#?}", n),
                    true => println!("{:#?}", n)
                }
            },
            None => {
                match new_line {
                    false => print!("nil"),
                    true => println!("nil")
                }
            },
        }    
    }

    pub fn interpret(&mut self, statements : Vec<Box<dyn Stmt>>) -> RuntimeError<Option<Literal>> {
        for statement in statements {
            self.execute(&statement)?;
        }
        
        Ok(None)
    }
    
    pub fn execute(&mut self, stmt : &Box<dyn Stmt>) -> RuntimeError<Option<Literal>> {
        stmt.accept(&mut Box::new(self as &mut dyn StmtVisitor))
    }

    pub fn evaluate(&mut self, expr : &Box<dyn Expr>) -> RuntimeError<Option<Literal>> {
        expr.accept(&mut Box::new(self as &mut dyn ExprVisitor))
    }
    
    pub fn is_equal(&self, a : Option<Literal>, b : Option<Literal>) -> Option<Literal> {
        match (a, b) {
            (None, None) => {
                Some(Literal::Boolean(true))
            },
            (Some(a_t), Some(b_t)) => {
                match (a_t, b_t) {
                    (Literal::Boolean(a_tt), Literal::Boolean(b_tt)) => {
                        Some(Literal::Boolean(a_tt == b_tt))
                    }
                    (Literal::Number(a_tt), Literal::Number(b_tt)) => {
                        Some(Literal::Boolean(a_tt == b_tt))
                    }
                    (Literal::String(a_tt), Literal::String(b_tt)) => {
                        Some(Literal::Boolean(a_tt == b_tt))
                    }
                    _ => { None }
                }
                
            },
            _ => {
                Some(Literal::Boolean(false))
            }
        }
    }
    
    pub fn is_not_equal(&self, a : Option<Literal>, b : Option<Literal>) -> Option<Literal> {
        match (a, b) {
            (None, None) => {
                Some(Literal::Boolean(false))
            },
            (Some(a_t), Some(b_t)) => {
                match (a_t, b_t) {
                    (Literal::Boolean(a_tt), Literal::Boolean(b_tt)) => {
                        Some(Literal::Boolean(a_tt != b_tt))
                    }
                    (Literal::Number(a_tt), Literal::Number(b_tt)) => {
                        Some(Literal::Boolean(a_tt != b_tt))
                    }
                    (Literal::String(a_tt), Literal::String(b_tt)) => {
                        Some(Literal::Boolean(a_tt != b_tt))
                    }
                    _ => { None }
                }
                
            },
            _ => {
                Some(Literal::Boolean(true))
            }
        }
    }
    
    pub fn is_truthy(&self, object : Option<Literal>) -> Option<Literal> {
        match object {
            Some(b) => {
                if let Literal::Boolean(x) = b {
                    Some(Literal::Boolean(x))
                } 
                else {
                    Some(Literal::Boolean(true))
                }
            }
            None => {
                Some(Literal::Boolean(false))
            }
        }
    }
    
    fn is_truthy_flip(&self, object : Option<Literal>) -> Option<Literal> {
        match object {
            Some(b) => {
                match b {
                    Literal::Boolean(x) => Some(Literal::Boolean(!x)),
                    Literal::Collection(x) => Some(Literal::Collection(x.into_iter().rev().collect())),
                    _ => Some(Literal::Boolean(false))
                }
            }
            None => {
                Some(Literal::Boolean(true))
            }
        }
    }
    
    pub fn execute_block(&mut self, statements : &Vec<Box<dyn Stmt>>, environment : Environment) -> RuntimeError<Option<Literal>> {
        let mut return_val = None;
        
        for statement in statements {
            match self.execute(&statement) {
                Ok(v) => {
                    if let Some(Literal::Keyword(v2)) = v {
                        return_val = Some(Literal::Keyword(v2));
                    }
                    else if let Some(Literal::Return(v2)) = v {
                        return_val = Some(*v2);
                    }
                    else {
                        continue;
                    }
                },
                Err((e, v)) => {
                    return Err((e, v))
                }
            }
        }
        
        Ok(return_val)
    }
}

impl StmtVisitor for Interpreter {
    fn visit_expression_stmt(&mut self, stmt : &StmtExpr) -> RuntimeError<Option<Literal>> {
        self.evaluate(&stmt.expression)
    }
    
    fn visit_var_stmt(&mut self, stmt : &Var) -> RuntimeError<Option<Literal>> {
        if let Some(init) = &stmt.initializer {
            let value : Option<Literal>;
        
            match self.evaluate(init) {
                Ok(v) => value = v,
                Err(e) => return Err(e)
            }
    
            self.environment.define(stmt.name.lexeme.clone(), value);
        }
        else {
            self.environment.define(stmt.name.lexeme.clone(), None);
        }
        
        Ok(None)
    }
    
    fn visit_print_stmt(&mut self, stmt :&Print) -> RuntimeError<Option<Literal>> {
        let value = self.evaluate(&stmt.expression)?;
        
        Self::print_helper(value, true);
        
        Ok(None)
    }
    
    fn visit_block_stmt(&mut self, stmt : &Block) -> RuntimeError<Option<Literal>> {
        self.environment = Environment::new_with_enclosing(self.environment.clone());
        let res = self.execute_block(&stmt.statements, Environment::new_with_enclosing(self.environment.clone()));
        let prev = *self.environment.clone().enclosing.unwrap();
        self.environment = prev;
        res
    }
    
    fn visit_class_stmt(&mut self, stmt : &Class) -> RuntimeError<Option<Literal>> {
        let mut super_class = None;
    
        if let Some(v) = stmt.super_class.clone() {
            let sc = self.evaluate(&v)?;

            match sc {
                Some(Literal::Class(v)) => { 
                    if let Some(v2) = v.as_any().downcast_ref::<LoxClass>() {
                        super_class = Some(Box::new(v2.clone()));
                        self.environment.define(format!("{}-super", stmt.name.lexeme.clone()), Some(Literal::Class(Box::new(v2.clone()))));
                    }
                },
                _ => return Err((stmt.name.clone(), "Super-class must be a class.".to_string()))
            }
        }
        
        self.environment.define(stmt.name.lexeme.clone(), None);
        
        let mut methods = HashMap::new();
        for method in &stmt.methods {
            if let Some(v) = method.as_any().downcast_ref::<Function>() {
                let is_init = v.name.lexeme.clone() == stmt.name.lexeme.clone();
                let function = LoxFunction::new(v.clone(), self.environment.clone(), FunctionType::Method, is_init);
                methods.insert(v.name.lexeme.clone(), function);
            }
        }
        
        let class = Box::new(LoxClass::new(stmt.name.lexeme.clone(), methods, super_class));
        
        self.environment.assign(stmt.name.clone(), Some(Literal::Class(class)))?;
        Ok(None)
    }
    
    fn visit_if_stmt(&mut self, stmt : &If) -> RuntimeError<Option<Literal>> {
        let eval = self.evaluate(&stmt.condition)?;
        
        if let Some(Literal::Boolean(b)) = self.is_truthy(eval) {
            if b {
                return self.execute(&stmt.then_branch);
            }
        }
        
        if let Some(else_branch) = &stmt.else_branch {
            return self.execute(&else_branch);
        }
        
        Ok(None)
    }
    
    fn visit_while_stmt(&mut self, stmt : &While) -> RuntimeError<Option<Literal>> {
        'main : loop {
            let eval = self.evaluate(&stmt.condition)?;

            if let Some(Literal::Boolean(b)) = self.is_truthy(eval) {
                if b {
                    if let Some(_) = stmt.body.as_any().downcast_ref::<Block>() {
                        let mut statements = Vec::new(); 
                        Self::collect_from_block(&stmt.body, &mut statements);
                        let mut cont_trigger = false;
                        
                        for statement in &statements {
                            match self.execute(statement) {
                                Ok(Some(Literal::Keyword(s))) => {
                                    match s.as_str() {
                                        "Break" => break 'main,
                                        "Continue" => {
                                            cont_trigger = true;
                                            break;
                                        }
                                        _ => { }
                                    }
                                },
                                Err(e) => {
                                    return Err(e);
                                },
                                _ => { continue; },
                            }
                        }
                        
                        if cont_trigger {
                            match stmt.loop_type {
                                LoopType::For => {
                                    self.execute(&statements[statements.len()-1])?;
                                },
                                LoopType::ForEach => {
                                    self.execute(&statements[statements.len()-2])?;
                                    self.execute(&statements[statements.len()-1])?;
                                },
                                _ => { }
                            }
                        }
                    }
                    else {
                        match self.execute(&stmt.body) {
                            Ok(Some(Literal::Keyword(s))) => {
                                match s.as_str() {
                                    "Break" => break 'main,
                                    "Continue" => break,
                                    _ => { }
                                }
                            },
                            Err(e) => {
                                return Err(e);
                            },
                            _ => { continue; },
                        }
                    }
                }
                else {
                    break;
                }
            }
        }
        Ok(None)
    }
    
    fn visit_function_stmt(&mut self, stmt : &Function) -> RuntimeError<Option<Literal>> {
        let function = Some(Literal::Function(Box::new(LoxFunction::new(stmt.clone(), self.environment.clone(), FunctionType::Normal, false))));
        self.environment.define(stmt.name.lexeme.clone(), function);
        Ok(None)
    }
    
    fn visit_return_stmt(&mut self, stmt : &Return) -> RuntimeError<Option<Literal>> {
        let mut value : Option<Literal> = None;
        
        if let Some(v) = stmt.value.clone() {
            match self.evaluate(&v) {
                Ok(v) => {
                    if let Some(i) = v {
                        value = Some(Literal::Return(Box::new(i)));
                    }
                },
                Err((e, v)) => return Err((e, v))
            }
        }
        
        Ok(value)
    }
    
    fn visit_break_stmt(&mut self, _stmt : &Break) -> RuntimeError<Option<Literal>> {
        Ok(Some(Literal::Keyword(String::from("Break"))))
    }
    
    fn visit_continue_stmt(&mut self, _stmt : &Continue) -> RuntimeError<Option<Literal>> {
        Ok(Some(Literal::Keyword(String::from("Continue"))))
    }
}

impl ExprVisitor for Interpreter {
    fn visit_binary_expr(&mut self, expr : &Binary) -> RuntimeError<Option<Literal>> {
        let right = self.evaluate(&expr.right)?;
        let left = self.evaluate(&expr.left)?;
        
        let a = left;
        let b = right;

        match expr.operator.type_ {
            TokenType::Minus => {
                match (a, b) {
                    (Some(Literal::Number(x)), Some(Literal::Number(y))) => {
                        Ok(Some(Literal::Number(x - y)))
                    },
                    _ => {
                        Err((expr.operator.clone(), "Operands must be Numbers.".to_string()))
                    }
                }
            },
            TokenType::Plus => {
                match (a, b) {
                    (Some(Literal::Number(x)), Some(Literal::Number(y))) => {
                        Ok(Some(Literal::Number(x + y)))
                    },
                    (Some(Literal::String(x)), Some(Literal::String(y))) => {
                        Ok(Some(Literal::String(format!("{}{}", x, y))))
                    },
                    (Some(Literal::String(x)), Some(Literal::Number(y))) => {
                        Ok(Some(Literal::String(format!("{}{}", x, y))))
                    },
                    (Some(Literal::Number(x)), Some(Literal::String(y))) => {
                        Ok(Some(Literal::String(format!("{}{}", x, y))))
                    },
                    _ => {
                        Err((expr.operator.clone(), "Operands must be Numbers Or Strings.".to_string()))
                    }
                }
            }
            TokenType::Slash => {
                match (a, b) {
                    (Some(Literal::Number(x)), Some(Literal::Number(y))) => {
                        Ok(Some(Literal::Number(x / y)))
                    },
                    _ => {
                        Err((expr.operator.clone(), "Operands must be Numbers.".to_string()))
                    }
                }
            }
            TokenType::Star => {
                match (a, b) {
                    (Some(Literal::Number(x)), Some(Literal::Number(y))) => {
                        Ok(Some(Literal::Number(x * y)))
                    },
                    _ => {
                        Err((expr.operator.clone(), "Operands must be Numbers.".to_string()))
                    }
                }
            },
            TokenType::Mod => {
                match (a, b) {
                    (Some(Literal::Number(x)), Some(Literal::Number(y))) => {
                        Ok(Some(Literal::Number(x % y)))
                    },
                    _ => {
                        Err((expr.operator.clone(), "Operands must be Numbers.".to_string()))
                    }
                }
            }
            TokenType::Greater => {
                match (a, b) {
                    (Some(Literal::Number(x)), Some(Literal::Number(y))) => {
                        Ok(Some(Literal::Boolean(x > y)))
                    },
                    _ => {
                        Err((expr.operator.clone(), "Operands must be Numbers.".to_string()))
                    }
                }
            },
            TokenType::GreaterEqual => {
                match (a, b) {
                    (Some(Literal::Number(x)), Some(Literal::Number(y))) => {
                        Ok(Some(Literal::Boolean(x >= y)))
                    },
                    _ => {
                        Err((expr.operator.clone(), "Operands must be Numbers.".to_string()))
                    }
                }
            },
            TokenType::Less => {
                match (a, b) {
                    (Some(Literal::Number(x)), Some(Literal::Number(y))) => {
                        Ok(Some(Literal::Boolean(x < y)))
                    },
                    _ => {
                        Err((expr.operator.clone(), "Operands must be Numbers.".to_string()))
                    }
                }
            },
            TokenType::LessEqual => {
                match (a, b) {
                    (Some(Literal::Number(x)), Some(Literal::Number(y))) => {
                        Ok(Some(Literal::Boolean(x <= y)))
                    },
                    _ => {
                        Err((expr.operator.clone(), "Operands must be Numbers.".to_string()))
                    }
                }
            }
            TokenType::BangEqual => {
                Ok(self.is_not_equal(a, b))
            }
            TokenType::EqualEqual => {
                Ok(self.is_equal(a, b))
            }
            _ => Err((expr.operator.clone(), "Invalid Operands".to_string()))
        }
    }

    fn visit_ternary_expr(&mut self, expr : &Ternary) -> RuntimeError<Option<Literal>> {
        let condition = self.evaluate(&expr.condition)?;
        let right = self.evaluate(&expr.right)?;
        let left = self.evaluate(&expr.left)?;
        
        if let Some(Literal::Boolean(b)) = condition {
            match b {
                true => Ok(left),
                false => Ok(right)
            }
        }
        else {
            Err((expr.operator_a.clone(), "Invalid condition for Ternary".to_string()))
        }
    }

    fn visit_grouping_expr(&mut self, expr : &Grouping) -> RuntimeError<Option<Literal>> {
        self.evaluate(&expr.expression)
    }
    fn visit_unary_expr(&mut self, expr : &Unary) -> RuntimeError<Option<Literal>> {
        let right = self.evaluate(&expr.right)?;
        
        match expr.operator.type_ {
            TokenType::Minus => {
                match right {
                    Some(val) => {
                        if let Literal::Number(x) = val {
                            Ok(Some(Literal::Number(-x)))
                        } 
                        else {
                            Err((expr.operator.clone(), "Operand must be a Number.".to_string()))
                        }
                    }
                    None => {
                        Err((expr.operator.clone(), "Operand must be a Number.".to_string()))
                    }
                }
            },
            TokenType::Hash => {
                match right {
                    Some(val) => {
                        if let Literal::Collection(x) = val {
                            Ok(Some(Literal::Number(x.len() as f64)))
                        } 
                        else {
                            Err((expr.operator.clone(), "Operand must be a List.".to_string()))
                        }
                    }
                    None => {
                        Err((expr.operator.clone(), "Operand must be a List.".to_string()))
                    }
                }
            }
            TokenType::Bang => {
                Ok(self.is_truthy_flip(right))
            },
            TokenType::Incr => {
                match right {
                    Some(val) => {
                        if let Literal::Number(x) = val {
                            let value = Some(Literal::Number(x+1.0));
                            
                            if let Some(i) = expr.right.as_any().downcast_ref::<VarExpr>() {
                                self.environment.assign(i.name.clone(), value.clone())?;
                            }
                            else if let Some(i) = expr.right.as_any().downcast_ref::<Get>() {
                                let object = self.evaluate(&i.object)?;
                                if let Some(Literal::Instance(mut v)) = object {
                                    v.set(i.name.clone(), value.clone());
                                    if let Some(as_var) = i.object.as_any().downcast_ref::<VarExpr>() {
                                        self.environment.assign(as_var.name.clone(), Some(Literal::Instance(v.clone())))?;
                                    }
                                    else if let Some(as_this) = i.object.as_any().downcast_ref::<This>() {
                                        self.environment.assign(as_this.keyword.clone(), Some(Literal::Instance(v.clone())))?;
                                    }
                                }
                            }
                            else if let Some(i) = expr.right.as_any().downcast_ref::<IndexGet>() {
                                let object = self.evaluate(&i.object)?;
                                if let Some(Literal::Collection(mut v)) = object {
                                    if let Some(Literal::Number(index)) = self.evaluate(&i.index)? {
                                        v[index as usize] = Box::new(value.clone());
                                    }
                                    else {
                                        return Err((expr.operator.clone(), "Index must be a number type.".to_string()))
                                    }
                                
                                    if let Some(as_var) = i.object.as_any().downcast_ref::<VarExpr>() {
                                        self.environment.assign(as_var.name.clone(), Some(Literal::Collection(v.clone())))?;
                                    }
                                }
                            }
                            Ok(Some(Literal::Number(x)))
                        } 
                        else {
                            Err((expr.operator.clone(), "OperAnd must be a Number.".to_string()))
                        }
                    }
                    None => {
                        Err((expr.operator.clone(), "OperAnd must be a Number.".to_string()))
                    }
                }
            },
            TokenType::Decr => {
                match right {
                    Some(val) => {
                        if let Literal::Number(x) = val {
                            let value = Some(Literal::Number(x-1.0));
                            if let Some(i) = expr.right.as_any().downcast_ref::<VarExpr>() {
                                self.environment.assign(i.name.clone(), value.clone())?;
                            }
                            else if let Some(i) = expr.right.as_any().downcast_ref::<Get>() {
                                let object = self.evaluate(&i.object)?;
                                if let Some(Literal::Instance(mut v)) = object {
                                    v.set(i.name.clone(), value.clone());
                                    if let Some(as_var) = i.object.as_any().downcast_ref::<VarExpr>() {
                                        self.environment.assign(as_var.name.clone(), Some(Literal::Instance(v.clone())))?;
                                    }
                                    else if let Some(as_this) = i.object.as_any().downcast_ref::<This>() {
                                        self.environment.assign(as_this.keyword.clone(), Some(Literal::Instance(v.clone())))?;
                                    }
                                }
                            }
                            else if let Some(i) = expr.right.as_any().downcast_ref::<IndexGet>() {
                                let object = self.evaluate(&i.object)?;
                                if let Some(Literal::Collection(mut v)) = object {
                                    if let Some(Literal::Number(index)) = self.evaluate(&i.index)? {
                                        let len = v.len().clone() as i32;
                                        v[((index as i32).rem_euclid(len)) as usize] = Box::new(value.clone());
                                    }
                                    else {
                                        return Err((expr.operator.clone(), "Index must be a number type.".to_string()))
                                    }
                                
                                    if let Some(as_var) = i.object.as_any().downcast_ref::<VarExpr>() {
                                        self.environment.assign(as_var.name.clone(), Some(Literal::Collection(v.clone())))?;
                                    }
                                }
                            }
                            Ok(Some(Literal::Number(x)))
                        } 
                        else {
                            Err((expr.operator.clone(), "OperAnd must be a Number.".to_string()))
                        }
                    }
                    None => {
                        Err((expr.operator.clone(), "OperAnd must be a Number.".to_string()))
                    }
                }
            },
            _ => {
                Err((expr.operator.clone(), "OperAnd type not found.".to_string()))
            }
        }
    }
    
    fn visit_literal_expr(&mut self, expr : &LiteralExp) -> RuntimeError<Option<Literal>> {
        if let Some(Literal::Function(f)) = expr.value.clone() {
            if let Some(lf) = f.as_any().downcast_ref::<LoxFunction>() {
                if let FunctionType::Anon = lf.f_type {
                        return Ok(Some(Literal::Function(Box::new(
                                LoxFunction::new(
                                    lf.declaration.clone(),
                                    Environment::new_with_enclosing(self.environment.clone()), 
                                    FunctionType::Anon,
                                    false
                                )
                        ))));
                }
            }
        }
        
        Ok(expr.value.clone())
    }
    
    fn visit_var_expr(&mut self, expr : &VarExpr) -> RuntimeError<Option<Literal>> {
        match self.environment.get(expr.name.clone())? {
            Some(v) => Ok(Some(v)),
            None => Ok(None)
        }
    }
    
    fn visit_this_expr(&mut self, expr : &This) -> RuntimeError<Option<Literal>> {
        self.environment.get(Token::new(TokenType::Identifier, "this", None, expr.keyword.line))
    }
    
    fn visit_super_expr(&mut self, expr : &Super) -> RuntimeError<Option<Literal>> {
        let current_this = 
        match self.environment
        .get(Token::new(TokenType::Identifier, "this", None, expr.keyword.line))? {
            Some(Literal::Instance(v)) => v,
            _ => {
                return Err((expr.keyword.clone(), "Could not find current this.".to_string()))
            }
        };
        
        match current_this.class.super_class {
            Some(v) => {
                if let Some(mut f) = v.find_method(expr.method.lexeme.clone()) {
                    let mut new_inst = LoxInstance::new(v);
                    new_inst.fields = current_this.fields.clone();
            
                    Ok(Some(
                        Literal::Function(Box::new(
                            f.bind(self, &new_inst)
                        ))
                    ))
                }
                else {
                    Err((expr.keyword.clone(), "Could not find current this.".to_string()))
                }
            },
            _ => { 
                Err((expr.keyword.clone(), "Could not find current this.".to_string()))
            }
        }
    }
    
    fn visit_assign_expr(&mut self, expr : &Assign) -> RuntimeError<Option<Literal>> {
        let value = self.evaluate(&expr.value)?.clone();
        
        let current = self.environment.get(expr.name.clone())?;
        let new_value = match expr.assign_type {
            Some(v) => {
                match v {
                    TokenType::Plus => {
                        match (current, value) {
                            (Some(Literal::Number(x)), Some(Literal::Number(y))) => {
                                Some(Literal::Number(x + y))
                            },
                            (Some(Literal::StrongNumber(x)), Some(Literal::Number(y))) => {
                                 Some(Literal::StrongNumber(x + y))
                            }
                            (Some(Literal::StrongNumber(x)), Some(Literal::StrongNumber(y))) => {
                                 Some(Literal::StrongNumber(x + y))
                            }
                            (Some(Literal::String(x)), Some(Literal::String(y))) => {
                                Some(Literal::String(format!("{}{}", x, y)))
                            },
                            (Some(Literal::String(x)), Some(Literal::Number(y))) => {
                                Some(Literal::String(format!("{}{}", x, y)))
                            },
                            (Some(Literal::Number(x)), Some(Literal::String(y))) => {
                                Some(Literal::String(format!("{}{}", x, y)))
                            },
                            (Some(Literal::Collection(mut x)), Some(y)) => {
                                x.push(Box::new(Some(y)));
                                Some(Literal::Collection(x))
                            },
                            (None, Some(Literal::Number(y))) => {
                                return Err((expr.name.clone(), "Cannot add Number from Nil.".to_string()));
                            }
                            _ => {
                                return Err((expr.name.clone(), "Operands must be Numbers Or Strings.".to_string()));
                            }
                        }
                    },
                    TokenType::Minus => {
                        match (current, value) {
                            (Some(Literal::Number(x)), Some(Literal::Number(y))) => {
                                Some(Literal::Number(x - y))
                            },
                            (Some(Literal::StrongNumber(x)), Some(Literal::Number(y))) => {
                                 Some(Literal::StrongNumber(x - y))
                            }
                            (Some(Literal::StrongNumber(x)), Some(Literal::StrongNumber(y))) => {
                                 Some(Literal::StrongNumber(x - y))
                            }
                            (Some(Literal::Collection(mut x)), Some(Literal::Number(y))) => {
                                let len = x.len() as i32;
                                x.remove((y as i32).rem_euclid(len) as usize);
                                if x.len() > 0 {
                                    Some(Literal::Collection(x))
                                }
                                else {
                                    None
                                }
                            },
                            (None, Some(Literal::Number(y))) => {
                                return Err((expr.name.clone(), "Cannot subtract Number from Nil.".to_string()));
                            }
                            _ => {
                                return Err((expr.name.clone(), "Operands must be Numbers.".to_string()));
                            }
                        }
                    },
                    TokenType::Slash => {
                        match (current, value) {
                            (Some(Literal::Number(x)), Some(Literal::Number(y))) => {
                                Some(Literal::Number(x / y))
                            },
                            (Some(Literal::StrongNumber(x)), Some(Literal::Number(y))) => {
                                 Some(Literal::StrongNumber(x / y))
                            }
                            (Some(Literal::StrongNumber(x)), Some(Literal::StrongNumber(y))) => {
                                 Some(Literal::StrongNumber(x / y))
                            }
                            _ => {
                                return Err((expr.name.clone(), "Operands must be Numbers.".to_string()));
                            }
                        }
                    },
                    TokenType::Star => {
                        match (current, value) {
                            (Some(Literal::Number(x)), Some(Literal::Number(y))) => {
                                Some(Literal::Number(x * y))
                            },
                            (Some(Literal::StrongNumber(x)), Some(Literal::Number(y))) => {
                                 Some(Literal::StrongNumber(x * y))
                            }
                            (Some(Literal::StrongNumber(x)), Some(Literal::StrongNumber(y))) => {
                                 Some(Literal::StrongNumber(x * y))
                            }
                            _ => {
                                return Err((expr.name.clone(), "Operands must be Numbers.".to_string()));
                            }
                        }
                    },
                    TokenType::Mod => {
                        match (current, value) {
                            (Some(Literal::Number(x)), Some(Literal::Number(y))) => {
                                Some(Literal::Number(x.rem_euclid(y)))
                            },
                            (Some(Literal::StrongNumber(x)), Some(Literal::Number(y))) => {
                                 Some(Literal::StrongNumber(x.rem_euclid(y)))
                            }
                            (Some(Literal::StrongNumber(x)), Some(Literal::StrongNumber(y))) => {
                                 Some(Literal::StrongNumber(x.rem_euclid(y)))
                            }
                            _ => {
                                return Err((expr.name.clone(), "Operands must be Numbers.".to_string()));
                            }
                        }
                    },
                    _ => { return Err((expr.name.clone(), "Invalid assign".to_string())); }
                }
            }
            None => {
                match (current, value.clone()) {
                    (Some(Literal::StrongNumber(x)), Some(Literal::Number(y))) => {
                         Some(Literal::StrongNumber(y))
                    }
                    (Some(Literal::StrongNumber(x)), Some(Literal::StrongNumber(y))) => {
                         Some(Literal::StrongNumber(y))
                    }
                    (Some(Literal::StrongNumber(x)), Some(y)) => {
                        return Err((expr.name.clone(), "Invalid assign".to_string()));    
                    }
                    
                    (Some(Literal::StrongBoolean(x)), Some(Literal::Boolean(y))) => {
                         Some(Literal::StrongBoolean(y))
                    }
                    (Some(Literal::StrongBoolean(x)), Some(Literal::StrongBoolean(y))) => {
                         Some(Literal::StrongBoolean(y))
                    }
                    (Some(Literal::StrongBoolean(x)), Some(y)) => {
                        return Err((expr.name.clone(), "Invalid assign".to_string()));    
                    }
                    
                    (Some(Literal::StrongString(x)), Some(Literal::String(y))) => {
                         Some(Literal::StrongString(y))
                    }
                    (Some(Literal::StrongString(x)), Some(Literal::String(y))) => {
                         Some(Literal::StrongString(y))
                    }
                    (Some(Literal::StrongString(x)), Some(y)) => {
                        return Err((expr.name.clone(), "Invalid assign".to_string()));    
                    }
                    
                    _ => {
                        value.clone()
                    }
                }
            }
        };
        
        self.environment.assign(expr.name.clone(), new_value.clone())?;
        Ok(new_value)
    }
    
    fn visit_logical_expr(&mut self, expr : &Logical) -> RuntimeError<Option<Literal>> {
        let left = self.evaluate(&expr.left)?;
        
        match expr.operator.type_ {
            TokenType::Or => {
                if let Some(Literal::Boolean(b)) = self.is_truthy(left.clone()) {
                    if b {
                        return Ok(left);
                    }
                }
            },
            _ => {
                if let Some(Literal::Boolean(b)) = self.is_truthy(left.clone()) {
                    if !b {
                        return Ok(left);
                    }
                }
            }
        }
        
        self.evaluate(&expr.right)
    }
    
    fn visit_call_expr(&mut self, expr : &Call) -> RuntimeError<Option<Literal>> {
        let callee = self.evaluate(&expr.callee)?;
        let mut arguments = Vec::new();
        
        for arg in &expr.arguments {
            arguments.push(self.evaluate(&arg)?);
        }
        
        let function_val = match callee.clone() {
            Some(Literal::Function(v)) => {
                Some(v)
            },
            Some(Literal::Class(v)) => {
                Some(v as Box<dyn LoxCallable>)
            },
            _ => {
                if let Some(v) = expr.callee.as_any().downcast_ref::<VarExpr>() {
                    return Err((
                        v.name.clone(), 
                        format!(
                            "Expected function."
                        )
                    ));
                }
                else {
                    return Err((
                        expr.paren.clone(), 
                        format!(
                            "Expected variable expression."
                        )
                    ));
                }
            }
        };
        
        match self.environment.get(
            function_val.clone().unwrap().get_name()
            ) {
            Ok(Some(Literal::Function(function))) => {
                if arguments.len() != function.arity() {
                    Err((
                        expr.paren.clone(), 
                        format!(
                            "Expected {} arguments but got {}.", 
                            function.arity(), 
                            arguments.len()
                        )
                    ))
                }
                else {
                    Ok(function.call(self, arguments, true)?)
                }
            },
            Ok(Some(Literal::Class(function))) => {
                if arguments.len() != function.arity() {
                    Err((
                        expr.paren.clone(), 
                        format!(
                            "Expected {} arguments but got {}.", 
                            function.arity(), 
                            arguments.len()
                        )
                    ))
                }
                else {
                    Ok(function.call(self, arguments, true)?)
                }
            },
            Err(_) => {
                if let Some(v) = expr.callee.as_any().downcast_ref::<VarExpr>() {
                    let func = self.evaluate(&expr.callee)?;
                    if let Some(Literal::Function(f)) = func {
                        if arguments.len() != f.arity() {
                            Err((
                                expr.paren.clone(), 
                                format!(
                                    "Expected {} arguments but got {}.", 
                                    f.arity(), 
                                    arguments.len()
                                )
                            ))
                        }
                        else {
                            Ok(f.call(self, arguments, true)?)
                        }
                    }
                    else {
                        Err((
                            expr.paren.clone(), 
                            format!(
                                "Expected function from var."
                            )
                        ))
                    }
                }
                else if let Some(v) = expr.callee.as_any().downcast_ref::<Get>() {
                    let func = self.evaluate(&expr.callee)?;
                    
                    if let Some(Literal::Function(f)) = func {
                        if arguments.len() != f.arity() {
                            Err((
                                expr.paren.clone(), 
                                format!(
                                    "Expected {} arguments but got {}.", 
                                    f.arity(), 
                                    arguments.len()
                                )
                            ))
                        }
                        else {
                            let previous = self.environment.clone();
                            let res = f.call(self, arguments, false)?;
                            
                            if let Some(caller) = v.object.as_any().downcast_ref::<VarExpr>() {
                                if let Ok(Some(v)) = self.environment
                                .get(Token::new(TokenType::Identifier, "this", None, 0)) {
                                    if let Some(prev) = self.environment.clone().enclosing {
                                        self.environment = *prev;
                                    }
                                    self.environment.assign(caller.name.clone(), Some(v.clone()))?;
                                }
                            }
                            else if let Some(caller) = v.object.as_any().downcast_ref::<IndexGet>() {
                                if let Ok(Some(v2)) = self.environment
                                .get(Token::new(TokenType::Identifier, "this", None, 0)) {
                                    if let Some(prev) = self.environment.clone().enclosing {
                                        self.environment = *prev;
                                    }
                                    
                                    if let Some(variable) = caller.object.as_any().downcast_ref::<VarExpr>() {
                                        if let Some(Literal::Collection(mut coll)) = self.environment.get(variable.name.clone())? {
                                            if let Some(Literal::Number(index)) = self.evaluate(&caller.index)? {
                                                let len = coll.len() as i32;
                                                coll[((index as i32).rem_euclid(len)) as usize] = Box::new(Some(v2.clone()));
                                                self.environment.assign(variable.name.clone(), Some(Literal::Collection(coll.clone())))?;
                                            }
                                        }
                                    }
                                }
                            }
                            
                            Ok(res)
                        }
                    }
                    else {
                        Err((
                            expr.paren.clone(), 
                            format!(
                                "Expected function from get."
                            )
                        ))
                    }
                }
                else if let Some(v) = expr.callee.as_any().downcast_ref::<IndexGet>() {
                    let func = self.evaluate(&expr.callee)?;
                    
                    if let Some(Literal::Function(f)) = func {
                        if arguments.len() != f.arity() {
                            Err((
                                expr.paren.clone(), 
                                format!(
                                    "Expected {} arguments but got {}.", 
                                    f.arity(), 
                                    arguments.len()
                                )
                            ))
                        }
                        else {
                            let previous = self.environment.clone();
                            let res = f.call(self, arguments, true)?;
                            
                            if let Some(caller) = v.object.as_any().downcast_ref::<VarExpr>() {
                                if let Ok(Some(v)) = self.environment
                                .get(Token::new(TokenType::Identifier, "this", None, 0)) {
                                    if let Some(prev) = self.environment.clone().enclosing {
                                        self.environment = *prev;
                                    }
                                    self.environment.assign(caller.name.clone(), Some(v.clone()))?;
                                }
                            }
                            else if let Some(caller) = v.object.as_any().downcast_ref::<IndexGet>() {
                                if let Ok(Some(v2)) = self.environment
                                .get(Token::new(TokenType::Identifier, "this", None, 0)) {
                                    if let Some(prev) = self.environment.clone().enclosing {
                                        self.environment = *prev;
                                    }
                                    
                                    if let Some(variable) = caller.object.as_any().downcast_ref::<VarExpr>() {
                                        if let Some(Literal::Collection(mut coll)) = self.environment.get(variable.name.clone())? {
                                            if let Some(Literal::Number(index)) = self.evaluate(&caller.index)? {
                                                let len = coll.len() as i32;
                                                coll[((index as i32).rem_euclid(len)) as usize] = Box::new(Some(v2.clone()));
                                                self.environment.assign(variable.name.clone(), Some(Literal::Collection(coll.clone())))?;
                                            }
                                        }
                                    }
                                }
                            }
                            
                            Ok(res)
                        }
                    }
                    else {
                        Err((
                            expr.paren.clone(), 
                            format!(
                                "Expected function from get."
                            )
                        ))
                    }
                }
                else if let Some(_) = expr.callee.as_any().downcast_ref::<Super>() {
                    let funct = self.evaluate(&expr.callee.clone())?;
                    if let Some(Literal::Function(function)) = funct {
                        if arguments.len() != function.arity() {
                            Err((
                                expr.paren.clone(), 
                                format!(
                                    "Expected {} arguments but got {}.", 
                                    function.arity(), 
                                    arguments.len()
                                )
                            ))
                        }
                        else {
                            let previous = self.environment.clone();
                            let res = function.call(self, arguments, false)?;
                            
                            if let Ok(Some(Literal::Instance(inst_old))) = self.environment
                            .get(Token::new(TokenType::Identifier, "this", None, 0)) {
                                if let Some(prev) = self.environment.clone().enclosing {
                                        self.environment = *prev;
                                    }
                                    
                                if let Ok(Some(Literal::Instance(inst))) = self.environment
                                .get(Token::new(TokenType::Identifier, "this", None, 0)) {
                                    let mut new_inst = inst;
                                    new_inst.fields = inst_old.fields.clone();

                                    self.environment.assign(
                                        Token::new(TokenType::Identifier, "this", None, 0),
                                        Some(Literal::Instance(new_inst))
                                    )?;
                                }
                            }
                            
                            Ok(res)
                        }
                    }
                    else {
                        Err((
                            expr.paren.clone(), 
                            format!(
                                "Expected function from super."
                            )
                        ))
                    }
                }
                else {
                    Err((
                        expr.paren.clone(), 
                        format!(
                            "Expected function from unknown."
                        )
                    ))
                }
            },
            _ => {
                Err((
                    expr.paren.clone(), 
                    format!(
                        "Expected function nil."
                    )
                ))
            }
        }
    }
    
    fn visit_get_expr(&mut self, expr : &Get) -> RuntimeError<Option<Literal>> {
        let object = self.evaluate(&expr.object.clone())?;
        if let Some(Literal::Instance(v)) = object {
            Ok(v.get(expr.name.clone(), self)?)
        }
        else {
            Err((expr.name.clone(), "Only instances have properties.".to_string()))
        }
    }
    
    fn visit_set_expr(&mut self, expr : &Set) -> RuntimeError<Option<Literal>> {
        let object = self.evaluate(&expr.object)?;
        
        if let Some(Literal::Instance(mut v)) = object {
            let value = self.evaluate(&expr.value)?;

            let current_val = v.get(expr.name.clone(), self);
            
            let new_value = 
            match current_val {
                Ok(current) => {
                    match expr.assign_type {
                        Some(v) => {
                            match v {
                                TokenType::Plus => {
                                    match (current, value.clone()) {
                                        (Some(Literal::Number(x)), Some(Literal::Number(y))) => {
                                            Some(Literal::Number(x + y))
                                        },
                                        (Some(Literal::String(x)), Some(Literal::String(y))) => {
                                            Some(Literal::String(format!("{}{}", x, y)))
                                        },
                                        (Some(Literal::String(x)), Some(Literal::Number(y))) => {
                                            Some(Literal::String(format!("{}{}", x, y)))
                                        },
                                        (Some(Literal::Number(x)), Some(Literal::String(y))) => {
                                            Some(Literal::String(format!("{}{}", x, y)))
                                        },
                                        (Some(Literal::Collection(mut x)), Some(y)) => {
                                            x.push(Box::new(Some(y)));
                                            Some(Literal::Collection(x))
                                        },
                                        _ => {
                                            return Err((expr.name.clone(), "Operands must be Numbers Or Strings.".to_string()));
                                        }
                                    }
                                },
                                TokenType::Minus => {
                                    match (current, value.clone()) {
                                        (Some(Literal::Number(x)), Some(Literal::Number(y))) => {
                                            Some(Literal::Number(x - y))
                                        },
                                        (Some(Literal::Collection(mut x)), Some(Literal::Number(y))) => {
                                            let len = x.len() as i32;
                                            if len > 0 {
                                                x.remove((y as i32).rem_euclid(len) as usize);
                                            }
                                            Some(Literal::Collection(x))
                                        },
                                        _ => {
                                            return Err((expr.name.clone(), "Operands must be Numbers.".to_string()));
                                        }
                                    }
                                },
                                TokenType::Slash => {
                                    match (current, value.clone()) {
                                        (Some(Literal::Number(x)), Some(Literal::Number(y))) => {
                                            Some(Literal::Number(x / y))
                                        },
                                        _ => {
                                            return Err((expr.name.clone(), "Operands must be Numbers.".to_string()));
                                        }
                                    }
                                },
                                TokenType::Star => {
                                    match (current, value.clone()) {
                                        (Some(Literal::Number(x)), Some(Literal::Number(y))) => {
                                            Some(Literal::Number(x * y))
                                        },
                                        _ => {
                                            return Err((expr.name.clone(), "Operands must be Numbers.".to_string()));
                                        }
                                    }
                                },
                                TokenType::Mod => {
                                    match (current, value.clone()) {
                                        (Some(Literal::Number(x)), Some(Literal::Number(y))) => {
                                            Some(Literal::Number(x.rem_euclid(y)))
                                        },
                                        _ => {
                                            return Err((expr.name.clone(), "Operands must be Numbers.".to_string()));
                                        }
                                    }
                                },
                                _ => { return Err((expr.name.clone(), "Invalid assign".to_string())); }
                            }
                        }
                        None => {
                            value.clone()
                        }
                    }
                },
                Err(_) => {
                    value.clone()
                }
            };
            
            v.set(expr.name.clone(), new_value.clone());
            
            if let Some(as_this) = expr.object.as_any().downcast_ref::<This>() {
                self.environment.assign(as_this.keyword.clone(), Some(Literal::Instance(v.clone())))?;
            }
            else if let Some(as_var) = expr.object.as_any().downcast_ref::<VarExpr>() {
                self.environment.assign(as_var.name.clone(), Some(Literal::Instance(v.clone())))?;
            }

            Ok(new_value)
        }
        else {
            Err((expr.name.clone(), "Only instances have fields.".to_string()))
        }
    }
    
    fn visit_index_expr(&mut self, expr : &Index) -> RuntimeError<Option<Literal>> {
        let mut collection = Vec::new();
        for c in &expr.collection {
            collection.push(Box::new(self.evaluate(c)?));
        }
        Ok(Some(Literal::Collection(collection)))
    }
    
    fn visit_index_get_expr(&mut self, expr : &IndexGet) -> RuntimeError<Option<Literal>> {
        if let Some(v) = expr.object.as_any().downcast_ref::<IndexGet>() {
            if let Some(Literal::Collection(c)) = self.evaluate(&expr.object)? {
                if let Some(Literal::Number(index)) = self.evaluate(&expr.index)? {
                    if c.len() > 0 {
                        Ok(*c[((index as i32).rem_euclid(c.len() as i32)) as usize].clone())
                    }
                    else {
                        Ok(None)
                    }
                }
                else {
                     Err((expr.keyword.clone(), "Attempt to index with non list type.".to_string()))
                }
            }
            else {
                self.visit_index_get_expr(&v)
            }
        }
        else {
            let mut object = self.evaluate(&expr.object)?;
            if let Some(Literal::String(s)) = object {
                let mut vec = vec!();
                for c in s.chars() {
                    vec.push(Box::new(Some(Literal::String(String::from(c)))));
                }
                object = Some(Literal::Collection(vec));
            }
            
            if let Some(Literal::Collection(c)) =  object {
                if let Some(Literal::Number(index)) = self.evaluate(&expr.index)? {
                    if c.len() > 0 {
                        Ok(*c[((index as i32).rem_euclid(c.len() as i32)) as usize].clone())
                    }
                    else {
                        Ok(None)
                    }
                }
                else {
                     Err((expr.keyword.clone(), "Attempt to index with non collection type.".to_string()))
                }
            }
            else {
                Err((expr.keyword.clone(), "Attempt to index non-var.".to_string()))
            }
        }
    }
    
    fn visit_index_set_expr(&mut self, expr : &IndexSet, coll : Vec<Box<Option<Literal>>>) -> RuntimeError<Option<Literal>> {
        let mut object = self.evaluate(&expr.object)?;
        let mut string_manip = false;
        if let Some(Literal::String(s)) = object {
            let mut vec = vec!();
            for c in s.chars() {
                vec.push(Box::new(Some(Literal::String(String::from(c)))));
            }
            string_manip = true;
            object = Some(Literal::Collection(vec));
        }
            
        let mut use_coll = coll.len() != 0;
        if let Some(Literal::Collection(mut v)) = object {
            let value = self.evaluate(&expr.value)?;

            let current_val = if let Some(Literal::Number(index)) = self.evaluate(&expr.index)? {
                Ok(*v[index as usize].clone())
            }
            else {
                Err((expr.name.clone(), "Attempt to index with non number type.".to_string()))
            };

            let new_value = match current_val {
                Ok(current) => {
                    match expr.assign_type {
                        Some(v) => {
                            match v {
                                TokenType::Plus => {
                                    match (current, value.clone()) {
                                        (Some(Literal::Number(x)), Some(Literal::Number(y))) => {
                                            Some(Literal::Number(x + y))
                                        },
                                        (Some(Literal::String(x)), Some(Literal::String(y))) => {
                                            Some(Literal::String(format!("{}{}", x, y)))
                                        },
                                        (Some(Literal::String(x)), Some(Literal::Number(y))) => {
                                            Some(Literal::String(format!("{}{}", x, y)))
                                        },
                                        (Some(Literal::Number(x)), Some(Literal::String(y))) => {
                                            Some(Literal::String(format!("{}{}", x, y)))
                                        },
                                        (Some(Literal::Collection(mut x)), Some(y)) => {
                                            x.push(Box::new(Some(y)));
                                            Some(Literal::Collection(x))
                                        },
                                        _ => {
                                            return Err((expr.name.clone(), "Operands must be Numbers Or Strings.".to_string()));
                                        }
                                    }
                                },
                                TokenType::Minus => {
                                    match (current, value.clone()) {
                                        (Some(Literal::Number(x)), Some(Literal::Number(y))) => {
                                            Some(Literal::Number(x - y))
                                        },
                                        (Some(Literal::Collection(mut x)), Some(Literal::Number(y))) => {
                                            let len = x.len() as i32;
                                            if len > 0 {
                                                x.remove((y as i32).rem_euclid(len) as usize);
                                            }
                                            Some(Literal::Collection(x))
                                        },
                                        _ => {
                                            return Err((expr.name.clone(), "Operands must be Numbers.".to_string()));
                                        }
                                    }
                                },
                                TokenType::Slash => {
                                    match (current, value.clone()) {
                                        (Some(Literal::Number(x)), Some(Literal::Number(y))) => {
                                            Some(Literal::Number(x / y))
                                        },
                                        _ => {
                                            return Err((expr.name.clone(), "Operands must be Numbers.".to_string()));
                                        }
                                    }
                                },
                                TokenType::Star => {
                                    match (current, value.clone()) {
                                        (Some(Literal::Number(x)), Some(Literal::Number(y))) => {
                                            Some(Literal::Number(x * y))
                                        },
                                        _ => {
                                            return Err((expr.name.clone(), "Operands must be Numbers.".to_string()));
                                        }
                                    }
                                },
                                TokenType::Mod => {
                                    match (current, value.clone()) {
                                        (Some(Literal::Number(x)), Some(Literal::Number(y))) => {
                                            Some(Literal::Number(x % y))
                                        },
                                        _ => {
                                            return Err((expr.name.clone(), "Operands must be Numbers.".to_string()));
                                        }
                                    }
                                },
                                _ => { return Err((expr.name.clone(), "Invalid assign".to_string())); }
                            }
                        }
                        None => {
                            value.clone()
                        }
                    }
                },
                Err(_) => {
                    value.clone()
                }
            };
        
            if let Some(as_var) = expr.object.as_any().downcast_ref::<VarExpr>() {
                if let Some(Literal::Number(index)) = self.evaluate(&expr.index)? {
                    if use_coll {
                        v[index as usize] = Box::new(Some(Literal::Collection(coll.clone())));
                    }
                    else {
                        v[index as usize] = Box::new(new_value.clone());
                    }
                }
                else {
                    return Err((expr.name.clone(), "Attempt to index with non number type.".to_string()));
                }
                if !string_manip {
                    self.environment.assign(as_var.name.clone(), Some(Literal::Collection(v.clone())))?;
                
                    Ok(new_value)
                }
                else {
                    let mut new_str = String::from("");
                    for c in v {
                        if let Some(Literal::String(s)) = *c {
                            new_str += s.as_str();
                        }
                    }
                
                    self.environment.assign(as_var.name.clone(), Some(Literal::String(new_str)))?;
                
                    Ok(new_value)
                }
            }
            else if let Some(as_get) = expr.object.as_any().downcast_ref::<Get>() {
                
                if let Some(Literal::Number(index)) = self.evaluate(&expr.index)? {
                    if use_coll {
                        v[index as usize] = Box::new(Some(Literal::Collection(coll.clone())));
                    }
                    else {
                        v[index as usize] = Box::new(new_value.clone());
                    }
                }
                else {
                    return Err((expr.name.clone(), "Attempt to index with non number type.".to_string()));
                }
                
                if let Some(Literal::Instance(mut inst)) = self.evaluate(&as_get.object)? {
                    inst.set(as_get.name.clone(), Some(Literal::Collection(v.clone())));

                    if let Some(as_this) = as_get.object.as_any().downcast_ref::<This>() {
                        self.environment.assign(as_this.keyword.clone(), Some(Literal::Instance(inst.clone())))?;
                    }
                    else if let Some(as_var) = as_get.object.as_any().downcast_ref::<VarExpr>() {
                        self.environment.assign(as_var.name.clone(), Some(Literal::Instance(inst.clone())))?;
                    }
                }
                
                Ok(new_value)
            }
            else if let Some(as_index_get) = expr.object.as_any().downcast_ref::<IndexGet>() {
                let mut new_set = expr.clone();
                new_set.index = as_index_get.index.clone();
                new_set.object = as_index_get.object.clone();
                
                if !use_coll {
                    if let Some(Literal::Number(index)) = self.evaluate(&expr.index)? {
                        v[index as usize] = Box::new(new_value.clone());
                    }
                    else {
                        return Err((expr.name.clone(), "Attempt to index with non number type.".to_string()));
                    }
                    
                    self.visit_index_set_expr(&new_set, v.clone())
                }
                else {
                    if let Some(Literal::Number(index)) = self.evaluate(&expr.index)? {
                        v[index as usize] = Box::new(Some(Literal::Collection(coll.clone())));
                    }
                    else {
                        return Err((expr.name.clone(), "Attempt to index with non number type.".to_string()));
                    }
                    
                    self.visit_index_set_expr(&new_set, v.clone())
                }
            }
            else {
                Err((expr.name.clone(), "Only list types can be indexed.".to_string()))
            }
        }
        else {
            Err((expr.name.clone(), "Only list types can be indexed.".to_string()))
        }
    }
}