
pub unsafe fn to_byte_slice<T>(data: &T) -> &[u8] {
    std::slice::from_raw_parts(
        data as *const T as *const u8,
        std::mem::size_of::<T>()
    )
}

pub fn to_bytes<T>(data: &T) -> Vec<u8> {
    let slice = unsafe { to_byte_slice(data) };
    let mut bytes:Vec<u8> = Vec::new();
    for byte in slice {
        bytes.push(*byte);
    }
    bytes
}

trait Serializable {
    fn serialize() -> Vec<u8>;
    fn deserialize(bytes: &Vec<u8>);
}