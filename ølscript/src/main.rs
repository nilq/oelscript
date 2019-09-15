#![feature(non_ascii_idents)]

extern crate colored;

mod øl;

use self::øl::source::*;
use self::øl::lexer::*;
use self::øl::parser::*;
use self::øl::compiler::*;

fn main() {
  let test= r#"
// øltionary
øl foo = {
  lager: "god"
  stick_a_finger_in_the_soil: "rigtig god og fin"
}

// øltion
øl main(x) =
  print("bob: ", x)

  øl x + 100

foo = 10
  "#;

  let source = Source::from("<main>", test.lines().map(|x| x.into()).collect::<Vec<String>>());
  let lexer  = Lexer::default(test.chars().collect(), &source);

  let mut tokens = Vec::new();
  
  for token_result in lexer {
    if let Ok(token) = token_result {
      tokens.push(token)
    } else {
      return
    }
  }

  println!("{:#?}", tokens);

  let mut parser  = Parser::new(tokens, &source);

  match parser.parse() {
    Ok(ref ast) => {
      let mut compiler = Compiler { source: &source };

      println!("{}", compiler.compile(ast))
    },
    _ => (),
  }
}
