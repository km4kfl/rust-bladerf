extern crate libc;

use std::*;
use libc::*;

#[allow(dead_code, non_camel_case_types)]
mod bladerf;
use bladerf::*;

impl fmt::Debug for Struct_bladerf_devinfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "serial: UNIMPLEMENTED, bus: {}, address: {})", self.usb_bus, self.usb_addr)
    }
}

//#[link(name = "bladerf")]
//extern {
//	fn bladerf_get_device_list(devices: &*mut [Struct_bladerf_devinfo]) -> libc::c_int;
//	fn bladerf_free_device_list(devices: *mut [Struct_bladerf_devinfo]);
//    fn bladerf_set_usb_reset_on_open (enabled: bool);
//}

pub fn get_device_list() -> Result<isize, isize> {

	unsafe{ 
		let devices: *mut [Struct_bladerf_devinfo] = mem::uninitialized();

		let n = bladerf_get_device_list(&devices) as isize;

		for i in 0..n {
			let serial_ptr: *const i8 = &(*devices)[i as usize].serial[0] as *const i8;
			let serial_string = ffi::CStr::from_ptr(serial_ptr);
			println!("serial: {}, bus: {}, address: {}", 
				str::from_utf8(serial_string.to_bytes()).unwrap(),
				(*devices)[i as usize].usb_bus as usize,
				(*devices)[i as usize].usb_addr as usize);
		}

		bladerf_free_device_list(devices);

		if n >= 0 {
			Ok(n)
		} else {
			Err(n)
		}
	}
}

pub fn set_usb_reset_on_open(enabled: bool) {
    unsafe{ 
    	bladerf_set_usb_reset_on_open(enabled as uint8_t); 
    } 
}

#[test]
fn discovery() {
	match get_device_list() {
		Ok(devices) => {
			println!("Discovered {} devices", devices);
		},
		Err(code) => {
			println!("Error {} listing devices", code);
			assert!(false);
		}
	}
}
