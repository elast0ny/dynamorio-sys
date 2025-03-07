#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[link(name = "wrapper")]
extern "C" {
    pub fn dr_stdout() -> file_t;
    pub fn dr_stderr() -> file_t;
    pub fn dr_stdin() -> file_t;
}
