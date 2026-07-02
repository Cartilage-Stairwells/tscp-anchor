use wasmtime::*;

fn main() -> Result<()> {
    let engine = Engine::default();

    let module = Module::from_file(
        &engine,
        "target/wasm32-unknown-unknown/release/tscp_protocol.wasm",
    )?;

    let mut store = Store::new(&engine, ());

    let instance = Instance::new(&mut store, &module, &[])?;

    let version = instance
        .get_typed_func::<(), i32>(&mut store, "tscp_version")?;

    let value = version.call(&mut store, ())?;

    println!("TSCP WASM ABI OK");
    println!("tscp_version={}", value);

    Ok(())
}
