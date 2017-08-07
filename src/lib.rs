extern crate libc;
use libc::c_int;

extern crate sharedlib;
// use sharedlib::Data;
// use sharedlib::error::*;
use sharedlib::Lib;
use sharedlib::Symbol;
use sharedlib::Func;

fn load_lib() -> sharedlib::Result<Lib> {
    let lib;
    unsafe {
        let path_to_lib = "mosquitto";
        lib = Lib::new(path_to_lib)
    }
    lib
}

pub fn mosquitto_lib_init() -> sharedlib::Result<i32> {

    let result: i32;
    unsafe {
        let lib = try!(load_lib());
        let func: Func<extern "C" fn() -> c_int> = try!(lib.find_func("mosquitto_lib_init"));
        let lib_init = func.get();
        result = lib_init();
    }
    Ok(result)
}

pub fn mosquitto_lib_cleanup() -> sharedlib::Result<i32> {

    let result: i32;
    unsafe {
        let lib = try!(load_lib());
        let func: Func<extern "C" fn() -> c_int> = try!(lib.find_func("mosquitto_lib_cleanup"));
        let lib_init = func.get();
        result = lib_init();
    }
    Ok(result)
}

pub fn mosquitto_lib_version() -> sharedlib::Result<(i32, i32, i32)> {

    let major: c_int = 0;
    let minor: c_int = 0;
    let revision: c_int = 0;
    unsafe {
        let lib = try!(load_lib());
        let func: Func<extern "C" fn(major: *const c_int,
                                     minor: *const c_int,
                                     revision: *const c_int)
                                     -> c_int> = try!(lib.find_func("mosquitto_lib_version"));
        let lib_version = func.get();
        lib_version(&major, &minor, &revision);
    }
    Ok((major, minor, revision))
}

#[cfg(test)]
mod tests {

    use super::mosquitto_lib_init;
    use super::mosquitto_lib_cleanup;
    use super::mosquitto_lib_version;

    #[test]
    fn lib_init() {
        println!("Старт теста");
        let result = mosquitto_lib_init().unwrap();
        println!("{}", result);
    }

    #[test]
    fn lib_cleanup() {
        println!("Старт теста");
        let result = mosquitto_lib_cleanup().unwrap();
        println!("{}", result);
    }

    #[test]
    fn lib_version() {
        println!("Старт теста");
        let (major, minor, revision) = mosquitto_lib_version().unwrap();
        println!("{}.{}.{}", major, minor, revision);
    }
}
