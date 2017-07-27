`cups-sys` [![Build Status](https://travis-ci.org/LegNeato/cups-sys.svg?branch=master)](https://travis-ci.org/LegNeato/cups-sys)
-----------------------------------------
Rust FFI bindings to [CUPS](https://www.cups.org/).

Background
-----------------------------------------

CUPS is the standards-based, open source printing system developed by Apple Inc. for macOS and other UNIX-like operating systems. CUPS uses the Internet Printing Protocol (IPP) to support printing to local and network printers.

This library ([`cups-sys`](https://github.com/LegNeato/cups-sys)) provides a low-level interface to the CUPS library installed on your system. The binding is generated at build time via the  [`bindgen`](https://github.com/servo/rust-bindgen) project.

I just want to print from Rust
-----------------------------------------

```rust
use std::mem;
use std::ptr;
use cups-sys::*;

unsafe {
  let mut dests: *mut cups_dest_t = mem::zeroed();
  let num_dests = cupsGetDests(&mut dests as *mut _);
  // Get the default printer.
  let destination: cups_dest_t = cupsGetDest(ptr::null(), ptr::null(), num_dests, dests);
  // Print a real page.
  let job_id: i32 = cupsPrintFile(
      (*destination).name,
      // File to print.
      CString::new("/path/to/file")
          .unwrap()
          .as_ptr(),
      // Name of the print job.
      CString::new("Test print job")
          .unwrap()
          .as_ptr(),
      (*destination).num_options,
      (*destination).options
  );
  println!("{}", job_id);
  cupsFreeDests(num_dests, dests);
}
```

For a pure-Rust IPP implementation, check out [`ipp.rs`](https://github.com/dremon/ipp.rs).

Documentation
-----------------------------------------

The auto-generated FFI reference docs can be found at https://docs.rs/cups-sys/.

The original CUPS API documentation (with examples) can be found at https://www.cups.org/doc/api-cups.html.

 Example usage
-----------------------------------------
```rust
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
            destination.options
        );
        let make_and_model = CStr::from_ptr(c_make_and_model).to_string_lossy();
        println!("{} ({})", printer_name, make_and_model);
    }

    cupsFreeDests(num_dests, dests);
}
```

License
-----------------------------------------
`cups-sys` is licensed under either of the following, at your option:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
