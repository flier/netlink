use netlink_packet::constants::{NLM_F_DUMP, NLM_F_REQUEST};
use netlink_packet::{
    Emitable, LinkHeader, LinkMessage, NetlinkBuffer, NetlinkFlags, NetlinkMessage, NetlinkPayload,
    Parseable, RtnlMessage,
};
use netlink_sys::{Protocol, Socket, SocketAddr};

fn main() {
    let mut socket = Socket::new(Protocol::Route).unwrap();
    let _port_number = socket.bind_auto().unwrap().port_number();
    socket.connect(&SocketAddr::new(0, 0)).unwrap();

    let mut packet: NetlinkMessage =
        RtnlMessage::GetLink(LinkMessage::from_parts(LinkHeader::new(), vec![])).into();
    packet
        .header_mut()
        .set_flags(NetlinkFlags::from(NLM_F_DUMP | NLM_F_REQUEST))
        .set_sequence_number(1);
    packet.finalize();
    let mut buf = vec![0; packet.header().length() as usize];

    // Before calling emit, it is important to check that the buffer in which we're emitting is big
    // enough for the packet, other `emit()` panics.
    assert!(buf.len() == packet.buffer_len());
    packet.emit(&mut buf[..]);

    println!(">>> {:?}", packet);
    socket.send(&buf[..], 0).unwrap();

    let mut receive_buffer = vec![0; 4096];
    let mut offset = 0;

    // we set the NLM_F_DUMP flag so we expect a multipart rx_packet in response.
    loop {
        let size = socket.recv(&mut receive_buffer[..], 0).unwrap();

        loop {
            let bytes = &receive_buffer[offset..];
            // Note that we're parsing a NetlinkBuffer<&&[u8]>, NOT a NetlinkBuffer<&[u8]> here.
            // This is important because Parseable<NetlinkMessage> is only implemented for
            // NetlinkBuffer<&'buffer T>, where T implements AsRef<[u8] + 'buffer. This is not
            // particularly user friendly, but this is a low level library anyway.
            //
            // Note also that the same could be written more explicitely with:
            //
            // let rx_packet =
            //     <NetlinkBuffer<_> as Parseable<NetlinkMessage>>::parse(NetlinkBuffer::new(&bytes))
            //         .unwrap();
            //
            let rx_packet: NetlinkMessage = NetlinkBuffer::new(&bytes).parse().unwrap();

            println!("<<< {:?}", rx_packet);

            if *rx_packet.payload() == NetlinkPayload::Done {
                println!("Done!");
                return;
            }

            offset += rx_packet.header().length() as usize;
            if offset == size || rx_packet.header().length() == 0 {
                offset = 0;
                break;
            }
        }
    }
}
