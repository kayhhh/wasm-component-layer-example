fn main() {
    let component_bytes = include_bytes!("../../target/wasm32-unknown-unknown/debug/guest.wasm");
    host::run_test(component_bytes).unwrap();
}
