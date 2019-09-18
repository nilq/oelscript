use super::*;
use super::super::error::Response::Wrong;

pub struct Compiler<'c> {
    pub source: &'c Source,
}

impl<'c> Compiler<'c> {
    pub fn compile(&mut self, ast: &'c Vec<Statement>) -> String {
        let mut result = String::new();

        for statement in ast.iter() {
            result.push_str(&self.generate_statement(&statement));
            result.push_str(";\n");
        }

        result
    }

    fn generate_statement(&mut self, statement: &Statement) -> String {
        use self::StatementNode::*;

        match statement.node {
            Expression(ref expression) => self.generate_expression(expression),
            Variable(ref left, ref right, is_const) => if is_const {
                format!("const {} = {}", left, self.generate_expression(right))
            } else {
                format!("var {} = {}", left, self.generate_expression(right))
            },
            Assignment(ref left, ref right) => self.generate_assignment(left, right),
            Function(ref name, ref params, ref body) => {
                let mut result = format!("let {} = function(", self.generate_expression(name));

                for (i, param) in params.iter().enumerate() {
                    result.push_str(&param);

                    if i < params.len() - 1 {
                        result.push_str(", ")
                    }
                }

                result.push_str(") {\n");

                for e in body.iter() {
                    result.push_str(&self.generate_statement(e));
                    result.push_str(";\n");
                }

                result.push_str("}");

                result
            },

            If(ref condition, ref body, ref else_branch) => {
                let mut result = format!("if ({}) {{", self.generate_expression(condition));

                for statement in body {
                    result.push_str(&format!("\t{};\n", self.generate_statement(statement)))
                }

                result.push('}');

                if let Some(body) = else_branch {
                    result.push_str("else {");

                    for statement in body.0.iter() {
                        result.push_str(&format!("\t{};\n", self.generate_statement(statement)))
                    }

                    result.push_str("}\n");
                } else {
                    result.push('\n');
                }

                result
            },

            Return(ref expr) => {
                format!("return {}", self.generate_expression(&expr.clone().unwrap()))
            },

            _ => unreachable!()
        }
    }

    fn generate_expression(&mut self, expression: &Expression) -> String {
        use self::ExpressionNode::*;
        use std::string;

        match expression.node {
            Binary(ref left, ref op, ref right) => {
                let mut result = string::String::new();

                result.push_str(
                    &format!(
                        "({} {} {})",
                        self.generate_expression(&left),
                        self.generate_operator(&op),
                        self.generate_expression(&right),
                    )
                );

                result
            },

            Table(ref table) => {
                let mut result = "{ ".to_string();

                for (key, value) in table.iter() {
                    result.push_str(&format!("{}: {}, ", key, self.generate_expression(&value)))
                }

                result.push('}');

                result
            },

            Call(ref called, ref args) => {
                let mut caller = self.generate_expression(called);
                let mut result = format!("{}(", caller);

                if let Index(ref left, ..) = called.node {
                    caller = self.generate_expression(left)
                }

                for (i, arg) in args.iter().enumerate() {
                    result.push_str(&self.generate_expression(arg));

                    if i < args.len() - 1 {
                        result.push_str(", ")
                    }
                }

                result.push(')');

                result
            },

            Array(ref content) => {
                let mut result = "[\n".to_string();

                for (i, arg) in content.iter().enumerate() {
                    let value    = self.generate_expression(arg);
                    let mut line = format!("{}", value);

                    if i < content.len() - 1 {
                        line.push(',')
                    }

                    result.push_str(&self.make_line(&line));
                }

                result.push_str("]");

                result
            },

            Index(ref source, ref index, _) => {
                let source = self.generate_expression(source);

                if let Identifier(ref name) = index.node {
                    format!("{}.{}", source, name)
                } else {
                    let right = self.generate_expression(index);

                    format!("{}[{}]", source, right)
                }
            },

            Int(ref n)        => format!("{}", n),
            Float(ref n)      => format!("{}", n),
            Bool(ref n)       => format!("{}", n),
            Str(ref n)        => format!("\"{}\"", n),
            Identifier(ref n) => format!("{}", n),

            Neg(ref n)                  => format!("-{}", self.generate_expression(n)),
            Not(ref n)                  => format!("not {}", self.generate_expression(n)),

            Empty => String::from("null"),
            _     => String::new()
        }
    }

    fn generate_assignment<'b>(&mut self, left: &'b Expression, right: &'b Expression) -> String {
        let left_string  = self.generate_expression(left);
        let right_string = self.generate_expression(right);

        let result = format!("{} = {}", left_string, right_string);

        result
    }

    fn generate_operator<'b>(&mut self, op: &'b Operator) -> String {
        use self::Operator::*;

        match *op {
            Concat => "..".to_string(),
            NEq    => "~=".to_string(),
            _ => format!("{}", op)
        }
    }

    fn make_line(&mut self, value: &str) -> String {
        let mut output = String::new();

        for line in value.lines() {
            output.push_str("  ");

            output.push_str(&line);
            output.push('\n')
        }

        output
    }

    fn push_line(&mut self, target: &mut String, value: &str) {
        target.push_str(&self.make_line(&value))
    }
}