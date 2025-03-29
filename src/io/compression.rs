use crate::*;
use std::io::{Read, Write};

use anyhow::Result;

// -----------------------------------------------------------------------------

#[inline(always)]
pub fn rle_decode<R: Read>(reader: &mut R, len: usize) -> Result<Vec<u8>> {
    let mut data = vec![];
    let mut i = 0;
    while i < len {
        let header = reader.read_i8()?;
        if header >= 0 {
            data.extend(reader.read_vec(header as usize + 1)?);
            i += header as usize + 2;
        } else if header == -128 {
            i += 1;
            continue;
        } else {
            let byte = reader.read_u8()?;
            data.extend(std::iter::repeat(byte).take(-header as usize + 1));
            i += 2;
        }
    }

    Ok(data)
}

#[inline(always)]
pub fn rle_encode<W: Write>(_writer: &mut W, _data: &[u8]) -> Result<()> {
    todo!()
}

// #[test]
// fn test_rle() {
//     use rand::Rng;
//     let mut rng = rand::thread_rng();
//     for _ in 0..100 {
//         let mut data = [0; 1000];
//         let mut i = 0;
//         while i < data.len() {
//             let n = rng.gen_range(1..=10);
//             let p = rng.gen_range(0..=255);
//             let j = std::cmp::min(i + n, data.len());
//             data[i..j].iter_mut().for_each(|x| *x = p);
//             i += n;
//         }

//         let mut encoded = vec![];
//         rle_encode(&mut encoded, &data).unwrap();

//         let decoded = encoded.clone();
//         let decoded = rle_decode(&mut decoded.as_slice(), data.len()).unwrap();

//         // assert_eq!(data.to_vec(), decoded);

//         assert_eq!(&data[..10], &decoded[..10]);
//         // println!("{:?}", (&data[..10], &decoded[..10]));
//     }
// }
