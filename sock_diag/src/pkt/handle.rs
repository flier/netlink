use super::List;
use crate::Handle;

pub struct PacketHandle(Handle);

impl PacketHandle {
    pub fn new(handle: Handle) -> Self {
        PacketHandle(handle)
    }

    /// Retrieve the list of sockets (equivalent to `ss --packet`)
    pub fn list(&mut self) -> List {
        List::new(self.0.clone())
    }
}