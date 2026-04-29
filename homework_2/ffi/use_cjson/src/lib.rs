use std::ffi::CStr;

pub mod cjson {
    #![allow(non_upper_case_globals)] // 'cause cJSON uses non-upper-case globals and others below
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs")); // to include the generated bindings
}

// check ../use_cjson_rust_lib
pub fn foo() {
    unsafe {
        let obj = cjson::cJSON_CreateObject();

        add_field(obj, "key1", "value1");
        add_field(obj, "key2", "value2");
        add_field(obj, "key3", "value3");

        let json = cjson::cJSON_Print(obj);
        let rust_cstr = CStr::from_ptr(json);
        let rust_str = rust_cstr.to_str().unwrap();
        // let rust_string = String::from(rust_str);
        println!("json object: {rust_str}");

        cjson::cJSON_Delete(obj); // WARNING: cleanup is obligatory using C bindings
    }
}

unsafe fn add_field(obj: *mut cjson::cJSON, key: &str, value: &str) {
    let key = std::ffi::CString::new(key).unwrap();
    let value = std::ffi::CString::new(value).unwrap();
    unsafe { cjson::cJSON_AddStringToObject(obj, key.as_ptr(), value.as_ptr()) };
}
