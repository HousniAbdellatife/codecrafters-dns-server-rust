use std::any::TypeId;
#[allow(unused_imports)]
use std::net::UdpSocket;


const TYPE_A: u16 = 1;
const CLASS_IN: u16 = 1;

fn main() {

    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);

                let question = Question::new(
                    "codecrafters.io".to_string(),
                    TYPE_A,
                    CLASS_IN
                );

                let answer = ResourceRecord {
                    name: "codecrafters.io".to_string(),
                    record_type: 1,
                    class: 1,
                    ttl: 60,
                    length: 4,
                    data: "\x08\x08\x08\x08".as_bytes().to_vec(),
                };

                let mut dnsHeader = DnsReplyHeader::new();
                dnsHeader.qdcount = 1;
                dnsHeader.ancount = 1;

                let dnsReply = DnsReply {
                    header: dnsHeader,
                    questions: vec![question],
                    answers: vec![answer],
                };

                let response = dnsReply.to_bytes();
                udp_socket
                    .send_to(&response, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}


struct DnsReply {
    header: DnsReplyHeader,
    questions: Vec<Question>,
    answers: Vec<ResourceRecord>
}

impl DnsReply {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.header.to_bytes());
        for question in &self.questions {
            bytes.extend_from_slice(&question.to_bytes());
        }

        for answer in &self.answers {
            bytes.extend_from_slice(&answer.to_bytes());
        }

        bytes
    }
}

struct Question {
    name: String,
    record_type: u16,
    class: u16
}


struct ResourceRecord {
    name: String,
    record_type: u16,
    class: u16,
    ttl: u32,
    length: u16,
    data: Vec<u8>
}

impl ResourceRecord {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for label in self.name.split(".") {
            bytes.push(label.len() as u8);
            bytes.extend_from_slice(label.as_bytes());
        }

        bytes.push(0x0);

        bytes.extend_from_slice(self.record_type.to_le_bytes().as_slice());
        bytes.extend_from_slice(self.class.to_le_bytes().as_slice());
        bytes.extend_from_slice(self.ttl.to_le_bytes().as_slice());
        bytes.extend_from_slice(self.length.to_le_bytes().as_slice());

        bytes.extend_from_slice(self.data.as_slice());

        bytes
    }
}
impl Question {
    fn new(name: String, record_type: u16, class: u16) -> Question {
        Self { name, record_type, class }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for label in self.name.split(".") {
            bytes.push(label.len() as u8);
            bytes.extend_from_slice(label.as_bytes());
        }
        bytes.push(0x0);

        bytes.extend_from_slice(&self.record_type.to_be_bytes());

        bytes.extend_from_slice(&self.class.to_be_bytes());

        bytes
    }
}
struct DnsReplyHeader {
    // bytes 1,2
    pid: u16,

    // bytes 3,4
    qr: bool, // query/response indicator -> 1 bit
    opcode: u8, //operation code          -> 4 bits
    aa: bool, // authoritative answer     -> 1 bit
    tc: bool, // Truncation               -> 1 bit
    rd: bool, // Recursion Desired        -> 1 bit
    ra: bool, // Recursion Available      -> 1 bits
    z: u8, // Reserved                    -> 3 bits
    rcode: u8, // Response Code (RCODE)   -> 4 bits


    // bytes 5,6
    qdcount: u16,  // Question Count

    // bytes 7, 8
    ancount: u16, // Answer Record Count

    // bytes 9, 10
    nscount: u16, // Authority Record Count (NSCOUNT)

    // bytes 11, 12
    arcount: u16,  // Additional Record Count (ARCOUNT)
}


impl DnsReplyHeader {
    fn new() -> Self {
        Self {
            pid: 1234,

            qr: true,
            opcode: 0,
            aa: false,
            tc: false,
            rd: false,

            ra: false,
            z: 0,
            rcode: 0,

            qdcount: 0,
            ancount: 0,
            nscount: 0,
            arcount: 0,
        }
    }

    fn to_bytes(&self) -> [u8; 12] {
        let mut header = [0u8; 12];

        header[0..2].copy_from_slice(&self.pid.to_be_bytes());

        header[2] =
            ((self.qr as u8) << 7)
            | (self.opcode << 3)
            | ((self.aa as u8) << 2)
            | ((self.tc as u8) << 1)
            | (self.rd as u8);

        header[3] =
            ((self.ra as u8) << 7)
                | ((self.z) << 4)
                | (self.rcode);

        header[4..6].copy_from_slice(&self.qdcount.to_be_bytes());
        header[6..8].copy_from_slice(&self.ancount.to_be_bytes());
        header[8..10].copy_from_slice(&self.nscount.to_be_bytes());
        header[10..12].copy_from_slice(&self.arcount.to_be_bytes());


        header
    }
}