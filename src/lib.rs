#![no_std]
#![feature(alloc_error_handler)]
#![allow(clippy::missing_safety_doc)]
#![allow(incomplete_features)]
#![feature(core_intrinsics)]
#![feature(const_generics)]

extern crate alloc;

use core::intrinsics::abort;

use cstr_core::{CStr, CString};
use log::*;

use crate::include::{PDRIVER_OBJECT, PUNICODE_STRING};
use crate::kernel::{find_kernel_module, get_kernel_module_export, get_kernel_modules, get_process_list, KernelError, Process};
use crate::util::{KernelAlloc};
use crate::util::log::KernelLogger;
use alloc::string::ToString;

pub mod include;
pub mod kernel;
#[macro_use]
pub mod util;
pub mod dispatch;
pub mod hooks;

/// When using the alloc crate it seems like it does some unwinding. Adding this
/// export satisfies the compiler but may introduce undefined behaviour when a
/// panic occurs.
#[no_mangle]
pub extern "system" fn __CxxFrameHandler3(_: *mut u8, _: *mut u8, _: *mut u8, _: *mut u8) -> i32 { unimplemented!() }

#[global_allocator]
static GLOBAL: KernelAlloc = KernelAlloc;

/// Explanation can be found here: https://github.com/Trantect/win_driver_example/issues/4
#[export_name = "_fltused"]
static _FLTUSED: i32 = 0;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    error!("panic: {:?}", info);
    #[allow(unused_unsafe)]
        unsafe { abort() }
}

static LOG_LEVEL: LevelFilter = LevelFilter::Trace;

unsafe fn main() -> Result<u32, KernelError> {
    info!("kernel-rs loaded");

    let modules = get_kernel_modules()?;

    let dxgkrnl = find_kernel_module(&modules, "dxgkrnl.sys").ok_or("could not find dxgkrnl")?;
    let address = get_kernel_module_export(dxgkrnl, "NtQueryCompositionSurfaceStatistics")
        .ok_or("could not find NtQueryCompositionSurfaceStatistics")?;

    kernel::hook_function(address, dispatch::hook);

    hooks::init_hooks()?;

    Ok(0)
}

#[no_mangle]
pub extern "system" fn driver_entry(driver_object: PDRIVER_OBJECT, _registry_path: PUNICODE_STRING) -> u32 {
    if let Err(e) = KernelLogger::init(LOG_LEVEL) {
        error!("Error setting logger: {:?}", e);
    }

    unsafe { (*driver_object).DriverUnload = Some(driver_unload) };

    match unsafe { main() } {
        Ok(code) => code,
        Err(err) => {
            error!("{:?}", err);
            1
        }
    }
}

pub unsafe extern "C" fn driver_unload(driver_object: PDRIVER_OBJECT) {
    info!("kernel-rs unloaded");
}

