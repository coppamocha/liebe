use std::any::TypeId;

pub struct ExportedFn {
    module: &'static str,
    name: &'static str,
    function: *const (),
    typeid: TypeId,
}
unsafe impl Sync for ExportedFn {}
inventory::collect!(ExportedFn);
