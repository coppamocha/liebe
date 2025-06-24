pub struct ExportedFn {
    pub module: &'static str,
    pub name: &'static str,
    pub function: *const (),
}
unsafe impl Sync for ExportedFn {}
inventory::collect!(ExportedFn);

pub fn init() {}
