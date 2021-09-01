mod addr_record;
mod naptr_record;
mod srv_record;

pub use addr_record::AddrRecord;
pub use naptr_record::{NaptrRecord, NaptrEntry, NaptrServices, NaptrFlags};
pub use srv_record::{SrvRecord, SrvEntry};
