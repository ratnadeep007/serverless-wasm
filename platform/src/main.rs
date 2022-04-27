use std::collections::HashMap;
use std::io;
use std::result;
use std::sync::{Arc, RwLock};

use wasi_common::pipe::WritePipe;
use wasmtime::{Engine, Linker, Module, Store};
use wasmtime_wasi::sync::WasiCtxBuilder;

use actix_web::web::{Path, Query};
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new().service(handler))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[get("/{module_name}")]
async fn handler(module_name: Path<String>) -> impl Responder {
    let wasm_module = format!("{}{}", module_name, ".wasm");
    let val = invoke_wasm_module(wasm_module).expect("");
    HttpResponse::Ok().body(val)
}

fn invoke_wasm_module(module_name: String) -> result::Result<String, wasmtime_wasi::Error> {
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    let stdout_buf: Vec<u8> = vec![];
    let stdout_mutex = Arc::new(RwLock::new(stdout_buf));
    let stdout = WritePipe::from_shared(stdout_mutex.clone());

    let wasi = WasiCtxBuilder::new().stdout(Box::new(stdout)).build();
    let mut store = Store::new(&engine, wasi);

    let module = Module::from_file(&engine, &module_name)?;
    linker.module(&mut store, &module_name, &module)?;

    let instance = linker.instantiate(&mut store, &module)?;
    let instance_main = instance.get_typed_func::<(), (), _>(&mut store, "_start")?;
    instance_main.call(&mut store, ())?;

    let mut buffer: Vec<u8> = Vec::new();
    stdout_mutex.read().unwrap().iter().for_each(|i| {
        buffer.push(*i);
    });
    let s = String::from_utf8(buffer)?;
    Ok(s)
}
