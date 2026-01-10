#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Entry {
    pub mask: u64,
    pub magic: u64,
    pub shift: u64,
    pub index: usize
}