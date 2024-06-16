use super::USER_STACK_SIZE;

#[repr(align(4096))]
pub(crate) struct UserStack {
    pub(crate) data: [u8; USER_STACK_SIZE],
}

impl UserStack {
    pub fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}
