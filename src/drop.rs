use super::dlopen::DlOpen;

pub struct DlDrop {
    lib: DlOpen
}

impl DlDrop {
    pub fn new (lib: DlOpen) -> DlDrop {
        DlDrop {
            lib: lib
        }
    }
}

unsafe impl Send for DlDrop {}
unsafe impl Sync for DlDrop {}