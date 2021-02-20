#[link(name = "c")]
extern "C" {
    fn geteuid() -> u32;
    fn setuid(_: u32) -> u32;
}

pub fn get_euid() -> u32 {
    let euid = unsafe { geteuid() };
    euid
}

pub fn set_uid(uid: u32) -> u32 {
    let error_code = unsafe {
        setuid(uid)
    };
    error_code
}
