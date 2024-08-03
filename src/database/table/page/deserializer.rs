pub trait Deserializer<T> {
    fn deserialize(data: &Vec<u8>, offset: usize, length: usize) -> T;
}

pub struct U32Deserializer;

impl Deserializer<u32> for U32Deserializer {
    fn deserialize(data: &Vec<u8>, offset: usize, length: usize) -> u32 {
        if offset + length <= data.len() {
            let bytes = &data[offset..offset + length];

            return u32::from_be_bytes(bytes.try_into().expect("Slice with incorrect length"));
        }

        panic!("The specific range is out of bounds");
    }
}

pub struct UTF8Deserializer;

impl Deserializer<String> for UTF8Deserializer {
    fn deserialize(data: &Vec<u8>, offset: usize, length: usize) -> String {
        if offset + length <= data.len() {
            let bytes = &data[offset..offset + length];

            // TODO Check if we can here avoid reallocating the data
            return String::from_utf8(bytes.to_vec()).expect("Expect valid UTF-8");
        }

        panic!("The specific range is out of bounds");
    }
}