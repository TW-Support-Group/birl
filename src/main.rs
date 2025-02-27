#![feature(let_chains)]

mod ast;
mod evaluator;
mod lexer;
mod parser;
mod token;

use evaluator::Evaluator;
use lexer::Lexer;
use parser::Parser;

use xitca_web::{
    body::ResponseBody,
    error::ErrorStatus,
    handler::{handler_service, json::LazyJson},
    http::Response,
    middleware::Logger,
    route::post,
    App,
};

#[derive(serde::Deserialize)]
struct BirlRequest {
    code: String,
}

async fn index(
    req: LazyJson<BirlRequest>,
) -> Result<Response<ResponseBody>, xitca_web::error::Error> {
    let BirlRequest { code } = req.deserialize()?;

    let tokens = Lexer::new(&code)
        .lex()
        .map_err(|_e| ErrorStatus::bad_request())?;

    let ast = Parser::new(tokens)
        .parse()
        .map_err(|_e| ErrorStatus::bad_request())?;

    let mut out = Vec::new();
    let mut err = Vec::new();
    let mut evaluator = Evaluator::new(&mut out, &mut err);

    evaluator.eval(ast);

    // TODO: Log errors from err.
    // TODO: Return a different status code if err is not empty.

    let res = Response::builder()
        .status(200)
        .header("Content-Type", "text/plain")
        .body(str::from_utf8(&out).unwrap().to_string().into())
        .unwrap();

    Ok(res)
}

fn main() -> std::io::Result<()> {
    env_logger::init();

    // TODO: Add clap commands for running in:
    // 1. Batch mode
    // 2. REPL mode
    // 3. Server mode

    App::new()
        .at("/", post(handler_service(index)))
        .enclosed(Logger::new())
        .serve()
        .bind("0.0.0.0:8080")?
        .run()
        .wait()
}
