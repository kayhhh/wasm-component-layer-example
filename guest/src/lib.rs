use exports::component_test::wit_protocol::my_interface::{Guest, GuestMyRes, MyRes};

wit_bindgen::generate!({
    path: "../protocol.wit",
});

struct MyResImpl;

impl GuestMyRes for MyResImpl {
    fn new() -> Self {
        Self
    }
}

struct MyInterface;

impl Guest for MyInterface {
    type MyRes = MyResImpl;

    fn my_func() -> Option<MyRes> {
        None
    }
}

export!(MyInterface);
