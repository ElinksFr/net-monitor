use libbpf_rs::Link;

use crate::packet_size;

/// Probs are unloaded when the structs goes out of scope
#[allow(dead_code)]
pub struct LoadedProb {
    tcp_received: Link,
    tcp_send: Link,
    udp_received: Link,
    udp_send: Link,
    clean_on_exit: Link,
}

impl LoadedProb {
    pub fn load_ebpf_monitoring_probs(
        skel: &mut packet_size::PacketSizeSkel<'_>,
    ) -> Result<LoadedProb, libbpf_rs::Error> {
        let tcp_received = skel.progs.tcp_received_packet_size.attach()?;
        let tcp_send = skel.progs.tcp_send_packet_size.attach()?;
        let udp_received = skel.progs.udp_received_packet_size.attach()?;
        let udp_send = skel.progs.udp_send_packet_size.attach()?;
        let clean_on_exit = skel.progs.stop_tracking_on_process_exit.attach()?;

        Ok(LoadedProb {
            tcp_received,
            tcp_send,
            udp_received,
            udp_send,
            clean_on_exit,
        })
    }
}
