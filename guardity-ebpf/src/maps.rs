use aya_bpf::{
    macros::map,
    maps::{HashMap, PerfEventArray},
};
use guardity_common::{
    AlertFileOpen, AlertSetuid, AlertSocketBind, AlertSocketConnectV4, AlertSocketConnectV6,
    Ipv4Addrs, Ipv6Addrs, Paths, Ports,
};

/// Map of allowed file open paths for each binary.
#[map]
pub(crate) static ALLOWED_FILE_OPEN: HashMap<u64, Paths> = HashMap::pinned(1024, 0);

/// Map of denied file open paths for each binary.
#[map]
pub(crate) static DENIED_FILE_OPEN: HashMap<u64, Paths> = HashMap::pinned(1024, 0);

/// Map of alerts for `file_open` LSM hook inspection.
#[map]
pub(crate) static ALERT_FILE_OPEN: PerfEventArray<AlertFileOpen> = PerfEventArray::pinned(1024, 0);

/// Map indicating which binaries are allowed to use `setuid`.
#[map]
pub(crate) static ALLOWED_SETUID: HashMap<u64, u8> = HashMap::pinned(1024, 0);

/// Map indicating which binaries are denied to use `setuid`.
#[map]
pub(crate) static DENIED_SETUID: HashMap<u64, u8> = HashMap::pinned(1024, 0);

/// Map of alerts for `setuid` LSM hook inspection.
#[map]
pub(crate) static ALERT_SETUID: PerfEventArray<AlertSetuid> = PerfEventArray::pinned(1024, 0);

/// Map of allowed socket bind ports for each binary.
#[map]
pub(crate) static ALLOWED_SOCKET_BIND: HashMap<u64, Ports> = HashMap::pinned(1024, 0);

/// Map of denied socket bind ports for each binary.
#[map]
pub(crate) static DENIED_SOCKET_BIND: HashMap<u64, Ports> = HashMap::pinned(1024, 0);

/// Map of alerts for `socket_bind` LSM hook inspection.
#[map]
pub(crate) static ALERT_SOCKET_BIND: PerfEventArray<AlertSocketBind> =
    PerfEventArray::pinned(1024, 0);

/// Map of allowed socket connect IPv4 addresses for each binary.
#[map]
pub(crate) static ALLOWED_SOCKET_CONNECT_V4: HashMap<u64, Ipv4Addrs> = HashMap::pinned(1024, 0);

/// Map of denied socket connect IPv4 addresses for each binary.
#[map]
pub(crate) static DENIED_SOCKET_CONNECT_V4: HashMap<u64, Ipv4Addrs> = HashMap::pinned(1024, 0);

/// Map of alerts for `socket_connect` LSM hook inspection.
#[map]
pub(crate) static ALERT_SOCKET_CONNECT_V4: PerfEventArray<AlertSocketConnectV4> =
    PerfEventArray::pinned(1024, 0);

/// Map of allowed socket connect IPv6 addresses for each binary.
#[map]
pub(crate) static ALLOWED_SOCKET_CONNECT_V6: HashMap<u64, Ipv6Addrs> = HashMap::pinned(1024, 0);

/// Map of denied socket connect IPv6 addresses for each binary.
#[map]
pub(crate) static DENIED_SOCKET_CONNECT_V6: HashMap<u64, Ipv6Addrs> = HashMap::pinned(1024, 0);

/// Map of alerts for `socket_connect` LSM hook inspection.
#[map]
pub(crate) static ALERT_SOCKET_CONNECT_V6: PerfEventArray<AlertSocketConnectV6> =
    PerfEventArray::pinned(1024, 0);
