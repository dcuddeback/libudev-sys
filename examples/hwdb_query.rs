extern crate libudev_sys as udev;
extern crate libc;

use std::env;
use std::str;

use std::ffi::{CString,CStr};

use libc::c_char;

fn main() {
    let args = env::args().collect::<Vec<_>>();

    if args.len() != 2 {
        println!("Usage: hwdb_query <modalias>");
        return;
    }

    let query = CString::new(args[1].clone()).unwrap();

    unsafe {
        let udev = udev::udev_new();

        if !udev.is_null() {
            let hwdb = udev::udev_hwdb_new(udev);

            if !hwdb.is_null() {
                query_hwdb(hwdb, &query);
                udev::udev_hwdb_unref(hwdb);
            }

            udev::udev_unref(udev);
        }
    }
}

unsafe fn query_hwdb(hwdb: *mut udev::udev_hwdb, query: &CString) {
    println!("{:>30}: {:?}", "query", query);

    udev::udev_hwdb_ref(hwdb);
    print_results(udev::udev_hwdb_get_properties_list_entry(hwdb, query.as_ptr(), 0));
    udev::udev_hwdb_unref(hwdb);
}

unsafe fn print_results(list_entry: *mut udev::udev_list_entry) {
    if list_entry.is_null() {
        return;
    }

    let key = unwrap_str(udev::udev_list_entry_get_name(list_entry));
    let val = unwrap_str(udev::udev_list_entry_get_value(list_entry));

    println!("{:>30}: {}", key, val);

    print_results(udev::udev_list_entry_get_next(list_entry));
}

unsafe fn unwrap_str<'a>(ptr: *const c_char) -> &'a str {
    str::from_utf8(CStr::from_ptr(ptr).to_bytes()).unwrap()
}
