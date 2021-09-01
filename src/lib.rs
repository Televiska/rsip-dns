mod context;
mod dns_client;
mod lookup;
mod target;

pub mod records;
pub mod resolvables;

pub use context::{AvailableTransports, Context};
pub use dns_client::DnsClient;
pub use lookup::Lookup;
pub use target::Target;

#[cfg(feature = "trust-dns")]
mod trust_dns_client;

//pub use triple::{Triple, TripleList};
#[cfg(feature = "trust-dns")]
pub use trust_dns_client::TrustDnsClient;

// From RFC 3263 basically there are 4 cases:
// ### IP address
//  * if transport is defined use that, otherwise default for SIP(S) URI
//  * if port is defined use that, otherwise default for transport
//  * use (given ip, given or default port, given or default transport)
// ### Domain with Port
//  * if transport is defined use that, otherwise default for SIP(S) URI
//  *! Do an A or AAAA record lookup to get the IPs
//      * for each ip addr found use (ip, given port, given or default transport)
// ### Domain with Transport
//  *! build & run a SRV lookup for the supported transport (with sips if URI is secure/sips)
//      * for each SRV result, do an A or AAAA
//          * for each address record found, use (ip, srv port, given transport)
//  *! if no SRV records are found run an A or AAAA and to get the ip addrs
//      * use the default Port for the given transport and try each (ip, default port, given transport)
// ### Domain without Port or Transport
//  *! run a NAPTR query to get all replacemenets domains
//      *! for each replacement domain, run a SRV lookup
//          * filter SRV results based on transports we support, and then sort based on priority/weight
//          *! for each SRV result, do an A or AAAA
//              * for each address record found, use (ip, srv port, srv transport)
//  * if no NAPTRs found, build a SRV lookup for each transport supported (with & without sips if secure is
//  supported in context & given transport)
//      *! for each SRV built, run a SRV lookup
//          *! for each SRV result, do an A or AAAA
//              * for each address record found, use (ip, srv port, srv (=supported) transport)
//  * if no SRV records are found
//      * use the default transport depending if it's SIP or SIPS URI
//      * use the default Port for the given default transport
//      *! Do an A or AAAA record lookup to get the IPs
//          * for each ip addr found use (ip, default port, default transport)
