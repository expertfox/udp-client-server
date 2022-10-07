use std::net::UdpSocket;
use std::thread;

#[derive(Clone, PartialEq, Copy, Debug)]
struct PacketA {
    x: u8,
    y: u8,
    z: u8,
}

#[derive(Clone, PartialEq, Copy, Debug)]
struct PacketC {
    x: bool,
    y: u8,
    z: u8,
}
impl PacketC {
    fn write(&self, buffer: &mut Buffer) {
        match self.x {
            true => buffer.elements.push(1),
            false => buffer.elements.push(0),
        }
        buffer.elements.push(self.y.clone());
        buffer.elements.push(self.z.clone());
    }
    fn read(&mut self, buffer: &mut [u8; 1500]) {
        match buffer[1] {
            1 => self.x = true,
            0 => self.x = false,
            _ => self.x = false, //RETHINK WHAT TO DO
        }
        self.y = buffer[2];
        self.z = buffer[3];
    }
}

#[derive(Clone, PartialEq, Copy, Debug)]
enum PacketTypeEnum {
    A(PacketA),
    C(PacketC),
}
impl PacketA {
    fn write(&self, buffer: &mut Buffer) {
        buffer.elements.push(self.x.clone());
        buffer.elements.push(self.y.clone());
        buffer.elements.push(self.z.clone());
    }
    fn read(&mut self, buffer: &mut [u8; 1500]) {
        self.x = buffer[1];
        self.y = buffer[2];
        self.z = buffer[3];
    }
}

// struct PacketB<T> {
//     maxElements: usize,
//     elements: [u8; maxElements],
// }

#[derive(Debug)]
struct Packet {
    packetType: u8,
    packet: PacketTypeEnum,
}
#[derive(Debug)]
struct Buffer {
    elements: Vec<u8>,
}

impl Packet {
    fn write_packet_info(&mut self, buffer: &mut Buffer) {
        buffer.elements.push(self.packetType);
    }
    fn read_packet_info(&mut self, buffer: &[u8; 1500]) {
        self.packetType = buffer[0];
    }
}

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:8888").expect("Could not bind socket");

    loop {
        let sock = socket.try_clone().expect("Failed to clone socket");
        let mut buffer = Buffer {
            elements: Vec::new(),
        };
        match socket.recv_from(&mut buffer.elements) {
            Ok((_, src)) => {
                thread::spawn(move || {
                    let packet_a = PacketA { x: 7, y: 8, z: 20 };
                    let mut packet1 = Packet {
                        packetType: 1,
                        packet: PacketTypeEnum::A(packet_a),
                    };
                    packet1.write_packet_info(&mut buffer);
                    packet_a.write(&mut buffer);
                    buffer.elements.push(255);

                    let packet_a = PacketA { x: 3, y: 48, z: 31 };
                    let mut packet1 = Packet {
                        packetType: 1,
                        packet: PacketTypeEnum::A(packet_a),
                    };
                    packet1.write_packet_info(&mut buffer);
                    packet_a.write(&mut buffer);
                    buffer.elements.push(255);

                    let packet_a = PacketA {
                        x: 23,
                        y: 28,
                        z: 44,
                    };
                    let mut packet1 = Packet {
                        packetType: 1,
                        packet: PacketTypeEnum::A(packet_a),
                    };
                    packet1.write_packet_info(&mut buffer);
                    packet_a.write(&mut buffer);
                    buffer.elements.push(255);

                    println!("Buffer: {:?}", buffer.elements);
                    sock.send_to(&buffer.elements, &src)
                        .expect("Failed to send a response");
                });
            }
            Err(e) => {
                eprintln!("couldn't recieve a datagram: {}", e);
            }
        }
    }
}
