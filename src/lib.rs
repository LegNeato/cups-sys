#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;
    use std::ptr;
    use std::ffi::CString;
    use std::ffi::CStr;

    #[test]
    fn constants() {
        assert_eq!(
            CUPS_FORMAT_TEXT,
            CString::new("text/plain").unwrap().as_bytes_with_nul()
        );
        assert_eq!(CUPS_JOBID_CURRENT, 0);
    }

    #[test]
    fn list_printers() {
        unsafe {
            let mut dests: *mut cups_dest_t = mem::zeroed();
            let num_dests = cupsGetDests(&mut dests as *mut _);
            std::slice::from_raw_parts(dests, num_dests as usize);
            cupsFreeDests(num_dests, dests);
        }
    }

    #[test]
    fn default_printer() {
        unsafe {
            let mut dests: *mut cups_dest_t = mem::zeroed();
            let num_dests = cupsGetDests(&mut dests as *mut _);
            cupsGetDest(ptr::null(), ptr::null(), num_dests, dests);
            cupsFreeDests(num_dests, dests);
        }
    }

    #[test]
    fn printer_info() {
        unsafe {
            let mut dests: *mut cups_dest_t = mem::zeroed();
            let num_dests = cupsGetDests(&mut dests as *mut _);
            let destinations = std::slice::from_raw_parts(dests, num_dests as usize);

            for destination in destinations {
                let c_printer_name = CStr::from_ptr((*destination).name);
                let printer_name = c_printer_name.to_string_lossy();

                let c_make_and_model = cupsGetOption(
                    CString::new("printer-make-and-model").unwrap().as_ptr(),
                    destination.num_options,
                    destination.options,
                );
                let make_and_model = CStr::from_ptr(c_make_and_model).to_string_lossy();
                println!("{} ({})", printer_name, make_and_model);
                /*
                // This prints a real page.
                let job_id: i32 = cupsPrintFile(
                    destination.name,
                    CString::new("/System/Library/CoreServices/SystemVersion.plist")
                        .unwrap()
                        .as_ptr(),
                    CString::new("Test print")
                        .unwrap()
                        .as_ptr(),
                    destination.num_options,
                    destination.options
                );
                println!("{}", job_id);
                */

            }
            cupsFreeDests(num_dests, dests);
        }
    }
}
