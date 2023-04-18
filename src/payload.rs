use crate::constants::IMAGE_WIDTH;
use serde::ser::SerializeTuple;
use serde::Serialize;
use std::convert::TryInto;

const ARRAY_SIZE: usize = 3 * IMAGE_WIDTH as usize;

#[derive(Debug, Serialize)]
pub struct Payload {
    pub row: usize,
    pub pixels: BigBoy,
}
#[derive(Debug)]
pub struct BigBoy([u8; ARRAY_SIZE as usize]);

impl Serialize for BigBoy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_tuple(ARRAY_SIZE as usize)?;
        for elem in &self.0[..] {
            seq.serialize_element(elem)?;
        }
        seq.end()
    }
}

impl TryInto<BigBoy> for &[u8] {
    type Error = ();
    fn try_into(self) -> Result<BigBoy, Self::Error> {
        if self.len() <= ARRAY_SIZE as usize {
            let mut result = BigBoy([0; ARRAY_SIZE as usize]);
            for (i, val) in self.iter().enumerate() {
                result.0[i] = *val;
            }
            Ok(result)
        } else {
            Err(())
        }
    }
}
