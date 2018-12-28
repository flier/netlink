use std::mem::size_of;

use utils::{parse_string, parse_u8};
use {DecodeError, DefaultNla, Emitable, NativeNla, Nla, NlaBuffer, NlasIterator, Parseable};

use constants::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TcNla {
    /// Unspecified
    Unspec(Vec<u8>),
    /// Name of queueing discipline
    Kind(String),
    /// Qdisc-specific options follow
    Options(Vec<u8>),
    /// Qdisc statistics
    Stats(TcStats),
    /// Module-specific statistics
    XStats(Vec<u8>),
    /// Rate limit
    Rate(Vec<u8>),
    Fcnt(Vec<u8>),
    Stats2(Vec<TcStats2Nla>),
    Stab(Vec<u8>),
    HwOffload(u8),
    Other(DefaultNla),
}

impl Nla for TcNla {
    #[rustfmt::skip]
    fn value_len(&self) -> usize {
        use self::TcNla::*;
        match *self {
            // Vec<u8>
            Unspec(ref bytes)
                | Options(ref bytes)
                | XStats(ref bytes)
                | Rate(ref bytes)
                | Fcnt(ref bytes)
                | Stab(ref bytes) => bytes.len(),
            HwOffload(_) => size_of::<u8>(),
            Stats2(ref thing) => thing.as_slice().buffer_len(),
            Stats(_) => size_of::<TcStats>(),
            Kind(ref string) => string.as_bytes().len() + 1,

            // Defaults
            Other(ref attr)  => attr.value_len(),
        }
    }

    #[cfg_attr(nightly, rustfmt::skip)]
    fn emit_value(&self, buffer: &mut [u8]) {
        use self::TcNla::*;
        match *self {
            // Vec<u8>
            Unspec(ref bytes)
                | Options(ref bytes)
                | XStats(ref bytes)
                | Rate(ref bytes)
                | Fcnt(ref bytes)
                | Stab(ref bytes) => buffer.copy_from_slice(bytes.as_slice()),

            HwOffload(ref val) => buffer[0] = *val,
            Stats2(ref stats) => stats.as_slice().emit(buffer),
            Stats(ref stats) => stats.to_bytes(buffer),

            Kind(ref string) => {
                buffer.copy_from_slice(string.as_bytes());
                buffer[string.as_bytes().len()] = 0;
            }

            // Default
            Other(ref attr) => attr.emit_value(buffer),
        }
    }

    fn kind(&self) -> u16 {
        use self::TcNla::*;
        match *self {
            Unspec(_) => TCA_UNSPEC,
            Kind(_) => TCA_KIND,
            Options(_) => TCA_OPTIONS,
            Stats(_) => TCA_STATS,
            XStats(_) => TCA_XSTATS,
            Rate(_) => TCA_RATE,
            Fcnt(_) => TCA_FCNT,
            Stats2(_) => TCA_STATS2,
            Stab(_) => TCA_STAB,
            HwOffload(_) => TCA_HW_OFFLOAD,
            Other(ref nla) => nla.kind(),
        }
    }
}

impl<'buffer, T: AsRef<[u8]> + ?Sized> Parseable<TcNla> for NlaBuffer<&'buffer T> {
    fn parse(&self) -> Result<TcNla, DecodeError> {
        use self::TcNla::*;
        let payload = self.value();
        Ok(match self.kind() {
            TCA_UNSPEC => Unspec(payload.to_vec()),
            TCA_KIND => Kind(parse_string(payload)?),
            TCA_OPTIONS => Options(payload.to_vec()),
            TCA_STATS => Stats(TcStats::from_bytes(payload)?),
            TCA_XSTATS => XStats(payload.to_vec()),
            TCA_RATE => Rate(payload.to_vec()),
            TCA_FCNT => Fcnt(payload.to_vec()),
            TCA_STATS2 => {
                let mut nlas = vec![];
                for nla in NlasIterator::new(payload) {
                    nlas.push(<Parseable<TcStats2Nla>>::parse(&(nla?))?);
                }
                Stats2(nlas)
            }
            TCA_STAB => Stab(payload.to_vec()),
            TCA_HW_OFFLOAD => HwOffload(parse_u8(payload)?),
            _ => Other(<Self as Parseable<DefaultNla>>::parse(self)?),
        })
    }
}

/// Generic queue statistics
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TcStats {
    /// Number of enqueued bytes
    pub bytes: u64,
    /// Number of enqueued packets
    pub packets: u32,
    /// Packets dropped because of lack of resources
    pub drops: u32,
    /// Number of throttle events when this flow goes out of allocated bandwidth
    pub overlimits: u32,
    /// Current flow byte rate
    pub bps: u32,
    /// Current flow packet rate
    pub pps: u32,
    pub qlen: u32,
    pub backlog: u32,
}

/// Byte/Packet throughput statistics
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TcStatsBasic {
    /// number of seen bytes
    pub bytes: u64,
    /// number of seen packets
    pub packets: u32,
}

/// Queuing statistics
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TcStatsQueue {
    /// queue length
    pub qlen: u32,
    /// backlog size of queue
    pub backlog: u32,
    /// number of dropped packets
    pub drops: u32,
    /// number of requeues
    pub requeues: u32,
    /// number of enqueues over the limit
    pub overlimits: u32,
}

impl NativeNla for TcStatsQueue {}
impl NativeNla for TcStatsBasic {}
impl NativeNla for TcStats {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TcStats2Nla {
    StatsApp(Vec<u8>),
    StatsBasic(TcStatsBasic),
    StatsQueue(TcStatsQueue),
    Other(DefaultNla),
}

impl Nla for TcStats2Nla {
    fn value_len(&self) -> usize {
        use self::TcStats2Nla::*;
        match *self {
            StatsApp(ref bytes) => bytes.len(),
            StatsBasic(_) => size_of::<TcStatsBasic>(),
            StatsQueue(_) => size_of::<TcStatsQueue>(),
            Other(ref nla) => nla.value_len(),
        }
    }

    fn emit_value(&self, buffer: &mut [u8]) {
        use self::TcStats2Nla::*;
        match *self {
            StatsApp(ref bytes) => buffer.copy_from_slice(bytes.as_slice()),
            StatsBasic(ref nla) => nla.to_bytes(buffer),
            StatsQueue(ref nla) => nla.to_bytes(buffer),
            Other(ref nla) => nla.emit_value(buffer),
        }
    }

    fn kind(&self) -> u16 {
        use self::TcStats2Nla::*;
        match *self {
            StatsApp(_) => TCA_STATS_APP,
            StatsBasic(_) => TCA_STATS_BASIC,
            StatsQueue(_) => TCA_STATS_QUEUE,
            Other(ref nla) => nla.kind(),
        }
    }
}

impl<'buffer, T: AsRef<[u8]> + ?Sized> Parseable<TcStats2Nla> for NlaBuffer<&'buffer T> {
    fn parse(&self) -> Result<TcStats2Nla, DecodeError> {
        use self::TcStats2Nla::*;
        let payload = self.value();
        Ok(match self.kind() {
            TCA_STATS_APP => StatsApp(payload.to_vec()),
            TCA_STATS_BASIC => StatsBasic(TcStatsBasic::from_bytes(payload)?),
            TCA_STATS_QUEUE => StatsQueue(TcStatsQueue::from_bytes(payload)?),
            _ => Other(<Self as Parseable<DefaultNla>>::parse(self)?),
        })
    }
}
