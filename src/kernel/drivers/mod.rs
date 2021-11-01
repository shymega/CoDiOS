//! This is the drivers crate for the `CoDiOS` firmware.  It abstracts over the cover display's
//! hardware, and provides the functions necessary for the kernel to interface with devices/peripherals.
#![deny(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    clippy::all,
    clippy::cargo,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    unused_extern_crates,
    variant_size_differences
)]