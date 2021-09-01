use rsip::{services::dns::*, Domain};

use std::{
    net::{Ipv4Addr, Ipv6Addr},
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone, Default)]
pub struct SpyDnsClient {
    pub inner: Arc<Mutex<InnerDnsClient>>,
}
#[derive(Debug, Clone, Default)]
pub struct InnerDnsClient {
    pub naptr_entries: Vec<NaptrEntry>,
    pub naptr_entries_should_panic: bool,
    pub naptr_entries_should_be_called: bool,
    pub naptr_entries_called: bool,
    pub srv_entries: Vec<SrvEntry>,
    pub srv_entries_should_panic: bool,
    pub srv_entries_should_be_called: bool,
    pub srv_entries_called: bool,
    pub a_entries: Vec<Ipv4Addr>,
    pub a_entries_should_be_called: bool,
    pub a_entries_called: bool,
    pub aaaa_entries_called: bool,
}

impl SpyDnsClient {
    pub fn new_with(inner: InnerDnsClient) -> Self {
        Self {
            inner: Arc::new(Mutex::new(inner)),
        }
    }

    pub fn a_entries(&self) -> Vec<Ipv4Addr> {
        let inner = self.inner.lock().unwrap();
        inner.a_entries.clone()
    }

    pub fn verify_expected_calls(&self) {
        let inner = self.inner.lock().unwrap();
        if inner.naptr_entries_should_be_called && !inner.naptr_entries_called {
            panic!("naptr_entries_for should have been callled, yet it didn't")
        }
        if inner.srv_entries_should_be_called && !inner.srv_entries_called {
            panic!("srv_entries_for should have been callled, yet it didn't")
        }
        if inner.a_entries_should_be_called && !inner.a_entries_called {
            panic!("a_entries_for should have been callled, yet it didn't")
        }
    }
}

#[async_trait::async_trait]
impl DnsClient for SpyDnsClient {
    async fn naptr_entries_for(&self, _domain: Domain) -> NaptrEntries {
        let mut inner = self.inner.lock().unwrap();
        match inner.naptr_entries_should_panic {
            true => panic!("should never call naptr_entries_for, yet it did!"),
            false => {
                inner.naptr_entries_called = true;
                inner.naptr_entries.clone().into()
            }
        }
    }
    async fn srv_entries_for(&self, _domain: Vec<SrvDomain>) -> SrvEntries {
        let mut inner = self.inner.lock().unwrap();
        match inner.srv_entries_should_panic {
            true => panic!("should never call srv_entries_for, yet it did!"),
            false => {
                inner.srv_entries_called = true;
                inner.srv_entries.clone().into()
            }
        }
    }
    async fn a_entries_for(&self, _domain: Vec<Domain>) -> Result<Vec<Ipv4Addr>, rsip::Error> {
        let mut inner = self.inner.lock().unwrap();
        inner.a_entries_called = true;
        Ok(inner.a_entries.clone())
    }
    async fn aaaa_entries_for(&self, _domain: Vec<Domain>) -> Result<Vec<Ipv6Addr>, rsip::Error> {
        let mut inner = self.inner.lock().unwrap();
        inner.aaaa_entries_called = true;
        Ok(vec![])
    }
}
