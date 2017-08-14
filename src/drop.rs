use libc::{c_void, dlclose};

pub struct DlDrop {
    handle: * mut c_void
}

impl DlDrop {
    pub fn new (handle: *mut c_void) -> DlDrop {
        DlDrop {
            handle: handle
        }
    }
}

unsafe impl Send for DlDrop {}
unsafe impl Sync for DlDrop {}

impl Drop for DlDrop {
    fn drop(&mut self) {
        assert_eq!(unsafe {dlclose(self.handle)}, 0);
    }
}