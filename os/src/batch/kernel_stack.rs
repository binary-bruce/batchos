use crate::trap::context::TrapContext;

use super::KERNEL_STACK_SIZE;

#[repr(align(4096))]
pub(crate) struct KernelStack {
    pub(crate) data: [u8; KERNEL_STACK_SIZE],
}

impl KernelStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }

    pub fn push_context(&self, cx: TrapContext) -> &'static mut TrapContext {
        let cx_ptr = (self.get_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
        unsafe {
            *cx_ptr = cx;
        }

        unsafe { cx_ptr.as_mut().unwrap() }
    }
}
