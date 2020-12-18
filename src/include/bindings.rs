/* automatically generated by rust-bindgen 0.56.0 */

pub type ULONG_PTR = crate::include::raw::c_ulonglong;
pub type SIZE_T = ULONG_PTR;
pub type PSIZE_T = *mut ULONG_PTR;
pub type PVOID = *mut crate::include::raw::c_void;
pub type LONG = crate::include::raw::c_long;
pub type CCHAR = crate::include::raw::c_char;
pub type NTSTATUS = LONG;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _KPROCESS {
    _unused: [u8; 0],
}
pub type PEPROCESS = *mut _KPROCESS;
pub type KPROCESSOR_MODE = CCHAR;
extern "C" {
    pub fn MmCopyVirtualMemory(
        FromProcess: PEPROCESS, FromAddress: PVOID, ToProcess: PEPROCESS, ToAddress: PVOID, BufferSize: SIZE_T,
        PreviousMode: KPROCESSOR_MODE, NumberOfBytesCopied: PSIZE_T,
    ) -> NTSTATUS;
}
