
pub unsafe fn to_bytes<T>(data: &T) -> &[u8] {
    std::slice::from_raw_parts(
        data as *const T as *const u8,
        std::mem::size_of::<T>()
    )
}

trait Serializable {
    fn serialize() -> Vec<u8>;
    fn deserialize(bytes: &Vec<u8>);
}