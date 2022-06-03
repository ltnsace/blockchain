use bincode;
use serde::{Serialize , Deserialize};
use crypto::digest::Digest;
use crypto::sha3::Sha3;


// pub fn serialize<T: ?Sized>(value: &T) -> Result<Vec<u8>> where
//     T: Serialize, 
pub fn my_serialize<T: ?Sized>(value: &T) -> Vec<u8>
    where T: Serialize,
{
    let serialize = bincode::serialize(value).unwrap();
    serialize
}

// pub fn deserialize<'a, T>(bytes: &'a [u8]) -> Result<T> where
//     T: Deserialize<'a>, 
pub fn my_deserialize<'a, T>(bytes:&'a [u8]) -> T
    where T: Deserialize<'a>,
{
    let deserialize = bincode::deserialize(bytes).unwrap();
    deserialize
}

pub fn get_hash(value: &[u8]) -> String{
    let mut hasher = Sha3::sha3_256();
    hasher.input(value);
    hasher.result_str()
}

//for test
#[derive(Serialize, Deserialize, PartialEq,Debug)]
struct Point{
    x: i32,
    y: i32,
}

#[cfg(test)]
mod tests {
    use crate::coder::Point;
    use crate::coder::{my_serialize, my_deserialize};

    #[test]
    fn  coders_work() {
        let point = Point{ x : 1, y: 1};
        let se = my_serialize(&point);
        let de:Point = my_deserialize(&se);
        assert_eq!(point, de);
    }
}
