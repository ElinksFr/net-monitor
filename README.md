# Net Monitor

Reproduces some of the functionality of [nethogs](https://github.com/raboof/nethogs) with rust and [eBPF](https://ebpf.io/).

It is really bare boned but does what **I** need from it as well as being fun/instructive to build. 

## How does it works

When net-monitor starts, it loads 5 probs into the kernels:
- 2 probs for the outgoing tcp/udp packets
- 2 probs for the incoming tcp/udp packets
- 1 prob to clean the tracking data when a process ends

The probs are all modifying the same Map `packet_stats` which olds the total number of bytes send and receive since the tracking starts. See `./src/bpf/packet_size.bfp.c`

On the user-land side we inspect the Map on a regular basis, and we keep an historic of ticks. When we have 2 ticks, we can derive a throughput. The ticks are stored in a fixed size `HistoryBuffer`, of a least one element where the new ticks erase the oldest ones (could have used a Vec but it would have been less fun).

The TUI is composed of a simple table build with [ratatui](https://ratatui.rs/).


## Building from source

You need a recent-ish kernel but no idea how recent, tested with `5.15.0`.

Some build dependecies via apt for Ubuntu `apt install clang gcc-multilib libelf1 libelf-dev zlib1g-dev`


Build the binary `cargo build --release` creates an executable in `./target/release/net-monitor`.
Or, install directly via cargo `cargo install --path .`

Running `sudo net-monitor` (needs to run with elevated privileges to loads ebfp programs).

Quit with `q` or `Ctrl+c`.

## Maybe one day

Features that may be implemented one day

 - [ ] Ability to sort the table
 - [ ] Pin the eBPF programs/map and having an agent monitoring that the cli can connect to
 - [ ] Clean all the unwrap/expect/un-necessary updates
 - [ ] Split the ipv4/ipv6 monitoring
 - [ ] Split the udp/tcp monitoring

## Run tests

The only tests are for the part that uses `unsafe`, the `HistoryBuffer` and the `Iterator` implementation, run `cargo test`. 

## vmlinux.h

To regenerate: `bpftool btf dump file /sys/kernel/btf/vmlinux format c > src/bpf/vmlinux.h`