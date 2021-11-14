use std::env::var;

use tokio_util::codec::Decoder;
use bytes::{BytesMut, Buf};
use crate::util::{VarIntError, get_var_int};

pub struct MyPacketDecoder;

const MAX: usize = 8 * 1024 * 1024;

impl Decoder for MyPacketDecoder {
    type Item = BytesMut;
    type Error = std::io::Error;

    fn decode(
        &mut self,
        src: &mut BytesMut
    ) -> Result<Option<Self::Item>, Self::Error> {
        // Check that the length is not too large to avoid a denial of
        // service attack where the server runs out of memory.
        // if length > MAX {
        //     return Err(std::io::Error::new(
        //         std::io::ErrorKind::InvalidData,
        //         format!("Frame of length {} is too large.", length)
        //     ));
        // }

        let mut my_iter = src.iter();
        let (var_int_length, length_of_length) = match get_var_int::<i32, _>(&mut my_iter) {
            Ok((v, var_int_length)) => {
                (v, var_int_length)
            }, Err(e) => {
                match e {
                    VarIntError::MissingExpectedByte => {return Ok(None);},
                    VarIntError::TooManyBytes{length: var_int_length} =>
                    {return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("VarInt of length {} is too large.", var_int_length.max_length())
            ));},
            }
        }
    };
    todo!("decoded length of message: {}, with the length of the VarInt being {} byte(s) long.", var_int_length, length_of_length);
}
}