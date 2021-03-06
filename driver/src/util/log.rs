use alloc::string::String;

use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};
pub use winapi::km::wdm::DbgPrint;
use winapi::km::wdm::DbgPrintEx;

/// Prints a string using DbgPrintEx. Automatically adds a null terminator
pub fn __kernel_print(mut text: String) {
    text.push('\n');
    text.push('\0');
    unsafe { DbgPrintEx(0, 0, text.as_ptr()) };
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => ({
        $crate::util::log::__kernel_print(alloc::format!($($arg)*));
    })
}

#[macro_export]
macro_rules! dbg {
    () => {
        println!("[{}:{}]", $crate::file!(), $crate::line!());
    };
    ($val:expr $(,)?) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                println!("[{}:{}] {} = {:#?}",
                    core::file!(), core::line!(), core::stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg!($val)),+,)
    };
}

pub struct KernelLogger;

static LOGGER: KernelLogger = KernelLogger;

impl KernelLogger {
    pub fn init(level: LevelFilter) -> Result<(), SetLoggerError> {
        log::set_logger(&LOGGER)
            .map(|()| log::set_max_level(level))
    }
}

impl log::Log for KernelLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let prefix = match record.level() {
            Level::Error => "[ERROR]",
            Level::Warn => "[!]",
            Level::Info => "[+]",
            Level::Debug => "[*]",
            Level::Trace => "[?]",
        };

        __kernel_print(alloc::format!("{} {}", prefix, record.args()));
    }

    fn flush(&self) {}
}
