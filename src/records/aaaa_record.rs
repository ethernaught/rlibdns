use std::any::Any;
use std::collections::HashMap;
use std::net::Ipv6Addr;
use crate::messages::inter::dns_classes::DnsClasses;
use crate::messages::inter::record_types::RecordTypes;
use crate::records::inter::record_base::RecordBase;

#[derive(Clone)]
pub struct AAAARecord {
    dns_class: Option<DnsClasses>,
    cache_flush: bool,
    ttl: u32,
    address: Option<Ipv6Addr>
}

impl Default for AAAARecord {

    fn default() -> Self {
        Self {
            dns_class: None,
            cache_flush: false,
            ttl: 0,
            address: None
        }
    }
}

impl RecordBase for AAAARecord {

    fn from_bytes(buf: &[u8], off: usize) -> Self {
        let dns_class = u16::from_be_bytes([buf[off], buf[off+1]]);
        let cache_flush = (dns_class & 0x8000) != 0;
        let dns_class = Some(DnsClasses::from_code(dns_class & 0x7FFF).unwrap());
        let ttl = u32::from_be_bytes([buf[off+2], buf[off+3], buf[off+4], buf[off+5]]);

        let length = u16::from_be_bytes([buf[off+6], buf[off+7]]) as usize;
        let record = &buf[off + 8..off + 8 + length];

        let address = match record.len() {
            16 => Ipv6Addr::from(<[u8; 16]>::try_from(record).expect("Invalid IPv6 address")),
            _ => panic!("Invalid Inet Address")
        };

        Self {
            dns_class,
            cache_flush,
            ttl,
            address: Some(address)
        }
    }

    fn to_bytes(&self, label_map: &mut HashMap<String, usize>, off: usize) -> Result<Vec<u8>, String> {
        let mut buf = vec![0u8; 10];

        buf.splice(0..2, self.get_type().get_code().to_be_bytes());

        let mut dns_class = self.dns_class.unwrap().get_code();
        if self.cache_flush {
            dns_class = dns_class | 0x8000;
        }

        buf.splice(2..4, dns_class.to_be_bytes());
        buf.splice(4..8, self.ttl.to_be_bytes());

        buf.extend_from_slice(&self.address.unwrap().octets().to_vec());

        buf.splice(8..10, ((buf.len()-10) as u16).to_be_bytes());

        Ok(buf)
    }

    fn get_type(&self) -> RecordTypes {
        RecordTypes::Aaaa
    }

    fn upcast(self) -> Box<dyn RecordBase> {
        Box::new(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn to_string(&self) -> String {
        format!("[RECORD] type {:?}, class {:?}, addr: {}", self.get_type(), self.dns_class.unwrap(), self.address.unwrap().to_string())
    }
}

impl AAAARecord {

    pub fn new(dns_classes: DnsClasses, cache_flush: bool, ttl: u32, address: Ipv6Addr) -> Self {
        Self {
            dns_class: Some(dns_classes),
            cache_flush,
            ttl,
            address: Some(address)
        }
    }

    pub fn set_dns_class(&mut self, dns_class: DnsClasses) {
        self.dns_class = Some(dns_class);
    }

    pub fn get_dns_class(&self) -> Result<DnsClasses, String> {
        match self.dns_class {
            Some(ref dns_class) => Ok(dns_class.clone()),
            None => Err("No dns class returned".to_string())
        }
    }

    pub fn set_ttl(&mut self, ttl: u32) {
        self.ttl = ttl;
    }

    pub fn get_ttl(&self) -> u32 {
        self.ttl
    }

    pub fn set_address(&mut self, address: Ipv6Addr) {
        self.address = Some(address);
    }

    pub fn get_address(&self) -> Option<Ipv6Addr> {
        self.address
    }
}
