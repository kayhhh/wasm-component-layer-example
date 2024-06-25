use wasm_component_layer::{
    Component, Func, FuncType, Linker, OptionType, OptionValue, ResourceType, Store, Value,
    ValueType,
};
use wasm_runtime_layer::Engine;

struct MyRes;

pub fn run_test(component_bytes: &[u8]) {
    let engine = Engine::new(wasmi_runtime_layer::Engine::default());
    let mut store = Store::new(&engine, ());
    let component = Component::new(&engine, component_bytes).unwrap();
    let mut linker = Linker::default();

    // Define imports.
    let host_interface = linker
        .define_instance("component-test:wit-protocol/host".try_into().unwrap())
        .unwrap();

    let my_res_ty = ResourceType::new::<MyRes>(None);
    let option_ty = OptionType::new(ValueType::Own(my_res_ty.clone()));

    let my_func_ty = Func::new(
        &mut store,
        FuncType::new([], [ValueType::Option(option_ty.clone())]),
        move |_ctx, _args, results| {
            let option = OptionValue::new(option_ty.clone(), None).unwrap();
            results[0] = Value::Option(option);

            Ok(())
        },
    );

    host_interface.define_resource("my-res", my_res_ty).unwrap();
    host_interface.define_func("my-func", my_func_ty).unwrap();

    // Run guest.
    let instance = linker.instantiate(&mut store, &component).unwrap();

    let guest = instance
        .exports()
        .instance(&"component-test:wit-protocol/guest".try_into().unwrap())
        .unwrap();

    let run = guest.func("run").unwrap();
    run.call(&mut store, &[], &mut []).unwrap();
}
