use std::fmt;
use std::fmt::Formatter;

#[derive(Copy, Default, Clone, Eq, PartialEq, Hash, Debug)]
pub enum RRTypes {
    #[default]
    A,
    Aaaa,
    Ns,
    CName,
    Soa,
    Ptr,
    Mx,
    Txt,
    Srv,
    Opt,
    RRSig,
    Nsec,
    DnsKey,
    Https,
    Spf,
    Tsig,
    Any,
    Ixfr,
    Axfr,
    Caa
}

impl RRTypes {

    pub fn from_code(code: u16) -> Option<Self> {
        for c in [
            Self::A,
            Self::Aaaa,
            Self::Ns,
            Self::CName,
            Self::Soa,
            Self::Ptr,
            Self::Mx,
            Self::Txt,
            Self::Srv,
            Self::Opt,
            Self::RRSig,
            Self::Nsec,
            Self::DnsKey,
            Self::Https,
            Self::Spf,
            Self::Tsig,
            Self::Ixfr,
            Self::Axfr,
            Self::Any,
            Self::Caa
        ] {
            if c.get_code() == code {
                return Some(c);
            }
        }

        None
    }

    pub fn get_code(&self) -> u16 {
        match self {
            Self::A => 1,
            Self::Aaaa => 28,
            Self::Ns => 2,
            Self::CName => 5,
            Self::Soa => 6,
            Self::Ptr => 12,
            Self::Mx => 15,
            Self::Txt => 16,
            Self::Srv => 33,
            Self::Opt => 41,
            Self::RRSig => 46,
            Self::Nsec => 47,
            Self::DnsKey => 48,
            Self::Https => 65,
            Self::Spf => 99,
            Self::Tsig => 250,
            Self::Ixfr => 251,
            Self::Axfr => 252,
            Self::Any => 255,
            Self::Caa => 257
        }
    }

    pub fn from_str(value: &str) -> Option<Self> {
        for c in [
            Self::A,
            Self::Aaaa,
            Self::Ns,
            Self::CName,
            Self::Soa,
            Self::Ptr,
            Self::Mx,
            Self::Txt,
            Self::Srv,
            Self::Opt,
            Self::RRSig,
            Self::Nsec,
            Self::DnsKey,
            Self::Https,
            Self::Spf,
            Self::Tsig,
            Self::Ixfr,
            Self::Axfr,
            Self::Any,
            Self::Caa
        ] {
            if c.to_string() == value {
                return Some(c);
            }
        }

        None
    }
}

impl fmt::Display for RRTypes {

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::A => "A",
            Self::Aaaa => "AAAA",
            Self::Ns => "NS",
            Self::CName => "CNAME",
            Self::Soa => "SOA",
            Self::Ptr => "PTR",
            Self::Mx => "MX",
            Self::Txt => "TXT",
            Self::Srv => "SRV",
            Self::Opt => "OPT",
            Self::RRSig => "RRSIG",
            Self::Nsec => "NSEC",
            Self::DnsKey => "DNSKEY",
            Self::Https => "HTTPS",
            Self::Spf => "SPF",
            Self::Tsig => "TSIG",
            Self::Ixfr => "IXFR",
            Self::Axfr => "AXFR",
            Self::Any => "ANY",
            Self::Caa => "CAA"
        })
    }
}
