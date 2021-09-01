pub mod mocked_dns_client;
pub mod panic_dns_client;
//pub mod spy_dns_client;

pub use mocked_dns_client::MockedDnsClient;
pub use panic_dns_client::PanicDnsClient;
//pub use spy_dns_client::{InnerDnsClient, SpyDnsClient};
