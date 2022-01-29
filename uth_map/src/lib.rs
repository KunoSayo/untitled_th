use std::collections::HashMap;
use std::num::NonZeroUsize;

pub mod bound;

/// The game block in the file.
/// Not the map in the gaming stage.
pub trait GameBlock {
    fn get_res(&self) -> &str;

    fn get_flags(&self) -> &[String];

    fn get_values(&self) -> &HashMap<String, f32>;
}

/// The game map in the file.
/// Not the map in the gaming stage.
pub trait GameMap<Block: GameBlock> {
    fn get_width(&self) -> Option<NonZeroUsize>;

    fn get_height(&self) -> Option<NonZeroUsize>;

    fn get_block_info(&self, x: usize, y: usize) -> Option<&Block>;
}

pub struct BlockInfo {
    res: String,
    flags: Vec<String>,
    value: HashMap<String, f32>,
}

impl GameBlock for BlockInfo {
    fn get_res(&self) -> &str {
        &self.res
    }

    fn get_flags(&self) -> &[String] {}

    fn get_values(&self) -> &HashMap<String, f32> {
        todo!()
    }
}

pub struct FixedUthMap {
    width: u32,
    height: u32,
    blocks: Vec<BlockInfo>,
    map: Vec<u32>,
}

impl BlockInfo {
    pub fn get_res(&self) -> &str {
        &self.res
    }

    pub fn get_flags(&self) -> &Vec<String> {
        &self.flags
    }

    pub fn get_values(&self) -> &HashMap<String, f32> {
        &self.value
    }
}

impl FixedUthMap {
    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn new(map: Vec<u32>, width: usize, height: usize)
}

fn read_zero_end_string(reader: &mut &[u8]) -> Result<&[u8], &'static str> {
    reader.position(|x| x == 0).ok_or("Read zero end string failed").map(|x| {
        let ret = &reader[0..x];
        *reader = &reader[x + 1..];
        ret
    })
}

impl TryFrom<Vec<u8>> for FixedUthMap {
    // todo: ???? str for err. b k s n
    type Error = &'static str;

    fn try_from(mut value: Vec<u8>) -> Result<Self, Self::Error> {
        let mut reader = &value[..];

        use byteorder::ReadBytesExt;
        use byteorder::BE;

        if reader.len() < 3 || &reader[0..3] != b"uth" {
            return Err("Not map file");
        }
        reader = &reader[3..];
        let blocks = reader.read_u32::<BE>().map_err(|_| "Read blocks failed")?;
        let width = reader.read_u32::<BE>().map_err(|_| "Read width failed")?;
        let height = reader.read_u32::<BE>().map_err(|_| "Read height failed")?;

        let mut block_info = Vec::with_capacity(blocks as _);

        for _ in 0..blocks {
            let flag_count = reader.read_u8().map_err(|_| "Read flag count failed")?;
            let key_value_count = reader.read_u8().map_err(|_| "Read flag count failed")?;
            let bounding = reader.read_u8().map_err(|_| "Read flag count failed")?;
            let res = String::from_utf8(read_zero_end_string(&mut reader)?.into()).map_err(|_| "Read utf8 string failed")?;

            let mut flags = Vec::with_capacity(flag_count as _);
            for _ in 0..flag_count {
                let flag = match String::from_utf8(read_zero_end_string(&mut reader)?.into()) {
                    Ok(s) => s,
                    Err(_e) => {
                        return Err("Invalid file, not valid utf8 string");
                    }
                };
                flags.push(flag);
            }
            let mut key_values = HashMap::with_capacity(key_value_count as _);
            for _ in 0..key_value_count {
                let key = match String::from_utf8(read_zero_end_string(&mut reader)?.into()) {
                    Ok(s) => s,
                    Err(_e) => {
                        return Err("Invalid file, not valid utf8 string");
                    }
                };
                let value = reader.read_f32::<BE>().map_err(|_| "Read f32 be value failed")?;
                if let Some(_) = key_values.insert(key, value) {
                    //todo: log dup here.
                }
            }
            block_info.push(BlockInfo {
                res,
                flags,
                value: key_values,
            })
        }
        let mut map = Vec::with_capacity(width * height as _);
        for _ in 0..height * width {
            let block_idx = reader.read_u32::<BE>().map_err(|_| "Read block idx failed")?;
            map.push(block_idx);
        }

        todo!()
    }
}

mod test {
    #[test]
    fn compile() {}
}