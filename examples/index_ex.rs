use std::ops::Index;

enum ConnectionType {
    WAN,
    LAN,
    VOIP,
}

struct ConnectionCount {
    w: usize,
    l: usize,
    v: usize,
}

impl Index<ConnectionType> for ConnectionCount {
    type Output = usize;

    fn index(&self, connection_type: ConnectionType) -> &Self::Output {
        match connection_type {
            ConnectionType::LAN => &self.l,
            ConnectionType::WAN => &self.w,
            ConnectionType::VOIP => &self.v,
        }
    }
}

fn main() {
    let connection_count = ConnectionCount {
        l: 10,
        w: 11,
        v: 12,
    };
    assert_eq!(connection_count[ConnectionType::WAN], 11);
    assert_eq!(connection_count[ConnectionType::LAN], 10);
    assert_eq!(connection_count[ConnectionType::VOIP], 12);
}
