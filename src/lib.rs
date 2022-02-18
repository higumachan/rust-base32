use std::fmt::{Display, Formatter};

mod engine;

const MASK_NBIT: [u8; 8] = [0x00, 0x01, 0x03, 0x07, 0x0f, 0x1f, 0x3f, 0x7f];
const PAD_SIZE: usize = 5;
const TABLE: &'static [u8] = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567".as_bytes();

#[derive(Debug)]
pub enum Error {
    InvalidOutputLength,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait Engine {
    fn encode(input: &[u8], output: &mut [u8]) -> Result<usize, Error>;
}

pub struct NaiveEngine {}

impl Engine for NaiveEngine {
    #[inline]
    fn encode(input: &[u8], output: &mut [u8]) -> Result<usize, Error> {
        let mut output_index = 0;
        let data = input;

        let mut remain = PAD_SIZE;
        let mut current = 0;
        let mut result = String::new();
        for d in data {
            result.push(
                TABLE[(((*d & MASK_NBIT[remain]) << (PAD_SIZE - remain)) | current) as usize]
                    .into(),
            );
            output_index = write_u8(
                output,
                TABLE[(((*d & MASK_NBIT[remain]) << (PAD_SIZE - remain)) | current) as usize]
                    .into(),
                output_index,
            )?;

            let inner_remain = 8 - remain;
            current = ((*d) >> remain) & MASK_NBIT[inner_remain];
            let inner_remain = if inner_remain >= PAD_SIZE {
                output_index = write_u8(
                    output,
                    TABLE[(current & MASK_NBIT[PAD_SIZE]) as usize].into(),
                    output_index,
                )?;
                current = current >> PAD_SIZE;
                inner_remain - PAD_SIZE
            } else {
                inner_remain
            };
            remain = PAD_SIZE - inner_remain;
        }
        if remain > 0 {
            output_index = write_u8(
                output,
                TABLE[(current & MASK_NBIT[PAD_SIZE]) as usize].into(),
                output_index,
            )?;
        }
        Ok(output_index)
    }
}

#[inline]
fn write_u8(output: &mut [u8], datum: u8, index: usize) -> Result<usize, Error> {
    *(output.get_mut(index).ok_or(Error::InvalidOutputLength)?) = datum;
    Ok(index + 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    fn string_encode<E: Engine>(data: &[u8]) -> String {
        let mut buf = [0u8; 1024];

        let size = E::encode(data, &mut buf).unwrap();

        String::from_utf8_lossy(&buf[0..size]).to_string()
    }

    #[test]
    fn it_works() {
        let data = [0xff];
        assert_eq!(string_encode::<NaiveEngine>(&data), "7H".to_string());

        let data = [0xff, 0xff];
        assert_eq!(string_encode::<NaiveEngine>(&data), "777B".to_string());

        let data = [0xff, 0xff, 0xff];
        assert_eq!(string_encode::<NaiveEngine>(&data), "7777P".to_string());
    }

    #[test]
    fn prop_test() {
        let mut rng = thread_rng();

        for i in 0..10000 {
            let data: Vec<u8> = (0..(rng.gen::<i32>() % 200))
                .into_iter()
                .map(|_| rng.gen())
                .collect();

            string_encode::<NaiveEngine>(&data);
        }
    }
}
