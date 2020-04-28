
struct TcpHeader {
    srcPort: u16,
    dstPort: u16,
    seqNum: u32,
    ackNum: u32,
    headerLen: u8,
    reserved: u8,
    urg: bool,
    agk: bool,
    psh: bool,
    pst: bool,
    syn: bool,
    fin: bool,
    window: u16,
    checksum: u16,
    urgentPointer: u16,
    options: Vec<u8>
}


impl TcpHeader {
    fn serialize(&self) -> Result<Vec<u8>, &'static str> {
        
    }
}