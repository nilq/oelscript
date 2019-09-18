#![feature(non_ascii_idents)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate rocket_cors;

use rocket::http::Method;
use rocket::response::{Responder, Response};
use rocket::http::{ContentType, Status};
use std::io::Cursor;

use rocket_cors::{AllowedOrigins, AllowedHeaders};

extern crate colored;

mod øl;

mod config;
use config::*;

use self::øl::source::*;
use self::øl::lexer::*;
use self::øl::parser::*;
use self::øl::compiler::*;

#[post("/compile", data = "<code>")]
fn compile<'c>(code: String) -> Result<Response<'c>, ()> {
  let source = Source::from("<main>", code.lines().map(|x| x.into()).collect::<Vec<String>>());
  let lexer  = Lexer::default(code.chars().collect(), &source);

  let mut body = String::new();

  let mut tokens = Vec::new();

  for token_result in lexer {
    if let Ok(token) = token_result {
      tokens.push(token)
    } else {
      return Response::build()
          .status(Status::Ok)
          .sized_body(Cursor::new("".to_string()))
          .header(ContentType::Plain)
          .raw_header("Access-Control-Allow-Origin", "*")
          .ok()
    }
  }

  let mut parser  = Parser::new(tokens, &source);

  match parser.parse() {
    Ok(ref ast) => {
      let mut compiler = Compiler { source: &source };

      let res = compiler.compile(ast);

      println!("{}", res);

      body = res;
    },
    Err(ref e) => (),
  }

  Response::build()
      .status(Status::Ok)
      .sized_body(Cursor::new(body.to_string()))
      .header(ContentType::Plain)
      .raw_header("Access-Control-Allow-Origin", "*")
      .ok()
}

fn main() {
  let (app_config, rocket_config) = config::config().map_err(|x| format!("{}", x)).unwrap();

  let options = rocket_cors::Cors {
    allowed_origins: AllowedOrigins::all(),
    allowed_methods: vec![Method::Get, Method::Post, Method::Put].into_iter().map(From::from).collect(),
    allowed_headers: AllowedHeaders::all(),
    allow_credentials: true,
    ..Default::default()
  };

  rocket_cors::catch_all_options_routes();

  rocket::ignite()
      .manage(app_config)
      .manage(options)
      .mount("/", rocket_cors::catch_all_options_routes())
      .mount("/", routes![compile]).launch()
  ;
}

fn test() {
  let test= r#"
// øltionary
iskold øl foo = {
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
