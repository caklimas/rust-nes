#[derive(Copy, Clone, Debug, Default)]
pub struct DirectMemoryAccess {
    pub page: u8,
    pub address: u8,
    pub data: u8
}