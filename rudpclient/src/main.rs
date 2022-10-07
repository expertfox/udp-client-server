use std::net::UdpSocket;
use std::{io, str};

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

fn pop(barry: &[u8]) -> [u8; 300] {
    let mut array = [0u8; 300];
    for (&x, p) in barry.iter().zip(array.iter_mut()) {
        *p = x;
    }
    array
}
fn main() {
    let socket = UdpSocket::bind("127.0.0.1:8881").expect("Could not bind client socket");
    socket
        .connect("127.0.0.1:8888")
        .expect("Could not connect to server");

    loop {
        let mut input = String::new();
        let mut buffer = [0u8; 1500];
        let mut raw_packet_vec: Vec<[u8; 300]> = Vec::new();

        socket
            .send(input.as_bytes())
            .expect("Failed to write to server");

        socket
            .recv_from(&mut buffer)
            .expect("Could not read into buffer");

        let mut starting_index = 0;
        for item in buffer.into_iter().enumerate() {
            let (i, x): (usize, u8) = item;
            if x == 255 {
                raw_packet_vec.push(pop(&buffer[starting_index..i]));
                starting_index = i + 1;
            }
        }

        println!("{:?}", raw_packet_vec);

        let mut packet_a = PacketA { x: 0, y: 0, z: 0 };

        // let packet_a = PacketA{}
        let mut packet_c = PacketC {
            x: true,
            y: 0,
            z: 0,
        };

        let mut packet_ref = Packet {
            packetType: 0,
            packet: PacketTypeEnum::A(packet_a),
        };

        packet_ref.read_packet_info(&mut buffer);

        match packet_ref.packetType {
            1 => {
                packet_a.read(&mut buffer);

                packet_ref.packet = PacketTypeEnum::A(packet_a);
            }
            // 2 => {
            //     packet_a.read(&mut buffer);

            //     packet_ref.packet = PacketTypeEnum::B(packet_b);
            // }
            3 => {
                packet_c.read(&mut buffer);

                packet_ref.packet = PacketTypeEnum::C(packet_c);
            }
            _ => {}
        }

        // print!("ADSAD:{:?}", packet_ref);
    }
}
