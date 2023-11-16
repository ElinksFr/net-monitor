use libbpf_rs::{Error, Link};

use crate::packet_size;

/// Probs are unloaded when the structs goes out of scope
#[allow(dead_code)]
pub struct LoadedProb {
    tcp_received: Link,
    tcp_send: Link,
    udp_received: Link,
    udp_send: Link,
}

impl LoadedProb {
    pub fn load_ebpf_monitoring_probs(
        skel: &mut packet_size::PacketSizeSkel<'_>,
    ) -> Result<LoadedProb, Error> {
        let tcp_received = skel.progs_mut().tcp_received_packet_size().attach()?;
        let tcp_send = skel.progs_mut().tcp_send_packet_size().attach()?;
        let udp_received = skel.progs_mut().udp_received_packet_size().attach()?;
        let udp_send = skel.progs_mut().udp_send_packet_size().attach()?;
        Ok(LoadedProb {
            tcp_received,
            tcp_send,
            udp_received,
            udp_send,
        })
    }
}
