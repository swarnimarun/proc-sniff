use netstat::{self, AddressFamilyFlags, ProtocolFlags};

pub fn get_ports(pid: u32) -> Vec<String> {
    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;
    netstat::iterate_sockets_info(af_flags, proto_flags)
        .unwrap()
        .flatten()
        .filter(|p| p.associated_pids.contains(&pid))
        .map(|p| match p.protocol_socket_info {
            netstat::ProtocolSocketInfo::Tcp(t) => {
                format!("{}:{}", t.local_addr, t.local_port)
            }
            netstat::ProtocolSocketInfo::Udp(u) => {
                format!("{}:{}", u.local_addr, u.local_port)
            }
        })
        .collect()
}
