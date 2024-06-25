use component_test::wit_protocol::host::my_func;
use exports::component_test::wit_protocol::guest::Guest;

wit_bindgen::generate!({
    path: "../protocol.wit",
});

struct GuestInterface;

impl Guest for GuestInterface {
    fn run() {
        let _res = my_func();
    }
}

export!(GuestInterface);
