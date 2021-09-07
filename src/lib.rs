//! A SIP Rust library implementing [RFC 3263](https://datatracker.ietf.org/doc/html/rfc3263),
//! implemented on top of [rsip](https://docs.rs/rsip)
//!
//! This library implements all the necessary DNS procedures defined in
//! [RFC3263](https://datatracker.ietf.org/doc/html/rfc3263) that allow a client or a server to
//! resolve a SIP URI into the (ip, port, transport) tuple. `rsip-dns` uses a lazy enumerator
//! architecture, in a sense of a Stream (but does not implement the Stream trait), which means
//! any query to the DNS client is performend only when needed.
//!
//! ## Examples
//!
//! ### Create a Context
//! The first thing you need is to specify a [Context] which will act as a guide to the [Lookup].
//! The [Context], among others, expect anything that implements the [DnsClient] trait. Refer to
//! that for more information. `rsip-dns` provides a default implementation of that trait
//! that you can use under the `trust-dns` feature flag.
//!
//!```
//! # use rsip_dns::records::*;
//! use rsip_dns::*;
//! use std::net::{IpAddr, Ipv4Addr};
//! use rsip::{Transport, Port, Host};
//! #
//! # #[derive(Debug, Clone, Default)]
//! # pub struct CustomDnsClient;
//! #
//! # #[async_trait::async_trait]
//! # impl rsip_dns::DnsClient for CustomDnsClient {
//! #     async fn naptr_lookup(&self, _domain: rsip::Domain) -> Option<NaptrRecord> {
//! #         panic!("should never call naptr_entries_for, yet it did!")
//! #     }
//! #     async fn srv_lookup(&self, _domain: rsip_dns::SrvDomain) -> Option<SrvRecord> {
//! #         panic!("should never call srv_entries_for, yet it did!")
//! #     }
//! #     async fn ip_lookup(&self, _domain: rsip::Domain) -> Result<AddrRecord, rsip::Error> {
//! #         panic!("should never call a_entries_for, yet it did!")
//! #     }
//! # }
//! # let my_dns_client = CustomDnsClient;
//!
//! let context = Context {
//!     secure: true,
//!     host: Host::from(IpAddr::V4(Ipv4Addr::new(192, 168, 2, 13))),
//!     transport: Some(Transport::Udp),
//!     port: Some(Port::from(5060)),
//!     dns_client: my_dns_client,
//!     supported_transports: Default::default()
//! };
//!```
//!
//! Here we created a context rather manually, but you can create a context out of a url as well
//! using the [Context::initialize_from] method:
//!
//!
//! For example:
//!
//!```
//! # use rsip_dns::records::*;
//! #
//! # #[derive(Debug, Clone, Default)]
//! # pub struct CustomDnsClient;
//! #
//! # #[async_trait::async_trait]
//! # impl rsip_dns::DnsClient for CustomDnsClient {
//! #     async fn naptr_lookup(&self, _domain: rsip::Domain) -> Option<NaptrRecord> {
//! #         panic!("should never call naptr_entries_for, yet it did!")
//! #     }
//! #     async fn srv_lookup(&self, _domain: rsip_dns::SrvDomain) -> Option<SrvRecord> {
//! #         panic!("should never call srv_entries_for, yet it did!")
//! #     }
//! #     async fn ip_lookup(&self, _domain: rsip::Domain) -> Result<AddrRecord, rsip::Error> {
//! #         panic!("should never call a_entries_for, yet it did!")
//! #     }
//! # }
//! #
//! use rsip_dns::*;
//! use rsip::prelude::*;
//! # let dns_client = CustomDnsClient;
//!
//! let uri = rsip::Uri {
//!     scheme: Some(rsip::Scheme::Sip),
//!     host_with_port: ("example.com", 5060).into(),
//!     ..Default::default()
//! };
//!
//! let context = Context::initialize_from(
//!     uri,
//!     dns_client,
//!     SupportedTransports::any(),
//! ).expect("uri and supported transports don't overlap");
//!```
//!
//! ### Lookup
//! Once you have the [Context], then you need to create a [Lookup] out of it.
//! Basically there is only one (async) method that you are interested to use from [Lookup], the
//! [resolve_next](ResolvableExt::resolve_next) and this actually comes from [ResolvableExt] trait.
//!
//!```
//! # use rsip_dns::records::*;
//! #
//! # #[derive(Debug, Clone, Default)]
//! # pub struct CustomDnsClient;
//! #
//! # #[async_trait::async_trait]
//! # impl rsip_dns::DnsClient for CustomDnsClient {
//! #     async fn naptr_lookup(&self, _domain: rsip::Domain) -> Option<NaptrRecord> {
//! #         panic!("should never call naptr_entries_for, yet it did!")
//! #     }
//! #     async fn srv_lookup(&self, _domain: rsip_dns::SrvDomain) -> Option<SrvRecord> {
//! #         panic!("should never call srv_entries_for, yet it did!")
//! #     }
//! #     async fn ip_lookup(&self, _domain: rsip::Domain) -> Result<AddrRecord, rsip::Error> {
//! #         panic!("should never call a_entries_for, yet it did!")
//! #     }
//! # }
//! #
//! # async fn foo() -> Result<(), rsip::Error> {
//! # use rsip_dns::*;
//! # use rsip::prelude::*;
//! # let dns_client = CustomDnsClient;
//! #
//! # let uri = rsip::Uri {
//! #     scheme: Some(rsip::Scheme::Sip),
//! #     host_with_port: ("example.com", 5060).into(),
//! #     ..Default::default()
//! # };
//!
//! # let context = Context::initialize_from(
//! #     uri,
//! #     dns_client,
//! #     SupportedTransports::any(),
//! # ).expect("uri and supported transports don't overlap");
//!
//! let mut lookup = Lookup::from(context);
//!
//! while let target = lookup.resolve_next().await {
//!     match target {
//!         Some(Target {
//!             ip_addr,
//!             port,
//!             transport,
//!         }) => println!("next tuple: ({:?}, {:?}, {:?})", ip_addr, port, transport),
//!         None => break,
//!     }
//! }
//!
//! # Ok(())
//! # }
//!```
//! For each iteration, the [Lookup] makes sure it lazily uses the underlying dns client. For
//! instance, in the case of SRV records, it first resolves the first SRV record for A/AAAA records
//! and then moves to the next. Usually you will find what you want quite fast (in the first 1-2
//! iterations), but according to RFC3263, if you don't have port and transport, and NAPTR records
//! are not responding, you might need 10 or even more DNS queries to resolve the peer (ip, port, transport)
//! tuple. Probably the dns client could some kind of caching, but that's left up to you, since
//! you need to provide a dns client that implements the [DnsClient] trait.
//!
//! ## Resolving the next (ip, port, transport) tuple
//! RFC 3263 explains in detail how the process of figuring out the (ip, port, transport) tuple
//! depending whether a port and/or a transport exists, but basically there are 4 distinct cases:
//!
//! ##### 1. IP address
//! In this case an IP address is given, regardless if a port/transport are available.
//!  * if transport is given, then it should be used otherwise the default transport SIP scheme
//!  is used (if it's sip, then TLS, otherwise UDP)
//!  * if port is given, then it should be used, otherwise the default port fot the resolved
//!  transport should be used
//!  * use (given ip, given or default port, given or default transport)
//!
//! ##### 2. Domain with Port
//! In this case the target is a domain and also a port is given.
//!  * if transport is given as well, then it should be used otherwise the default transport SIP scheme
//!  is used (if it's sip, then TLS, otherwise UDP)
//!  * **perform** an A or AAAA record lookup for the domain to get the IPs
//!      * for each ip addr found use (resolved ip, given port, given or default transport)
//!
//! ##### 3. Domain with Transport
//!  * **perform** a SRV lookup for the supported transport (should take into account sips or sip
//!  scheme here as well)
//!      * for each SRV result, **perform** an A or AAAA
//!          * for each address record found, use (ip, srv port, given transport)
//!  * if no SRV records are found **perform** an A or AAAA and to get the ip addrs
//!      * use the default Port for the given transport and try each (ip, default port, given transport)
//!
//! ##### 4. Domain without Port or Transport
//!  * **perform** a NAPTR query to get all replacemenets domains
//!      * for each replacement domain, **perform** a SRV lookup
//!          * filter SRV results based on transports that are supported and then sort based on
//!          priority/weight
//!          * for each SRV result, **perform** an A or AAAA
//!              * for each address record found, use (ip, srv port, srv transport)
//!  * if no NAPTRs found, build and **perform** SRV lookup for each transport supported (with & without sips if secure is
//!  supported in context & given transport)
//!     * for each SRV result, **perform** an A or AAAA
//!         * for each address record found, use (ip, srv port, srv transport)
//!  * if no SRV records are found
//!      * use the default transport depending if it's SIP or SIPS URI
//!      * use the default Port for the given default transport
//!      * **perform** an A or AAAA record lookup to get the IPs
//!          * for each ip addr found use (ip, default port, default transport)
//!
//! ## Reusable structure using the `ResolvableExt` trait
//! If you notice on the section above, there are many reusable components. For instance, (2)
//! reuses (1), while (3) reuses (2) (which reuses (1)) and (4) reuses all the previous.
//!
//! The structure of the code follows this pattern by defining a `ResolvableExt` trait,
//! `Resolvable` type and other types that are built on top of `Resolvable` or implement
//! `ResolvableExt` trait.

mod context;
mod dns_client;
mod lookup;
mod target;

pub mod records;
pub mod resolvables;

pub use context::{Context, SupportedTransports};
pub use dns_client::DnsClient;
pub use lookup::Lookup;
pub use records::SrvDomain;
pub use resolvables::ResolvableExt;
pub use target::Target;

#[cfg(feature = "trust-dns")]
pub mod trust_dns;
