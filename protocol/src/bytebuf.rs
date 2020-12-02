use std::io::{Read, Result, ErrorKind, Error};
use std::any::TypeId;

pub trait ByteBuf {
    fn read_u8(&mut self) -> std::io::Result<u8>;
    fn read_u16(&mut self) -> std::io::Result<u16>;
    fn read_u32(&mut self) -> std::io::Result<u32>;
    fn read_u64(&mut self) -> std::io::Result<u64>;
    fn read_u128(&mut self) -> std::io::Result<u128>;
    fn read_i8(&mut self) -> std::io::Result<i8>;
    fn read_i16(&mut self) -> std::io::Result<i16>;
    fn read_i32(&mut self) -> std::io::Result<i32>;
    fn read_i64(&mut self) -> std::io::Result<i64>;
    fn read_i128(&mut self) -> std::io::Result<i128>;
    fn read_f32(&mut self) -> std::io::Result<f32>;
    fn read_f64(&mut self) -> std::io::Result<f64>;
    fn read_varint(&mut self) -> std::io::Result<i32>;
    fn read_varlong(&mut self) -> std::io::Result<i64>;
    fn read_string(&mut self) -> std::io::Result<String>;
    fn read_t<T: 'static>(&mut self) -> std::io::Result<T>;
}

impl<U> ByteBuf for U where U: Read {
    fn read_u8(&mut self) -> std::io::Result<u8> {
        let mut buf = [0; 1];
        let read = self.read(&mut buf)?;
        if read != 1 {
            Err(Error::new(ErrorKind::InvalidData, "Cannot read, stream ended"))
        } else {
            Ok(u8::from_be_bytes(buf))
        }
    }

    fn read_u16(&mut self) -> Result<u16> {
        let mut buf = [0; 2];
        let read = self.read(&mut buf)?;
        if read != 2 {
            Err(Error::new(ErrorKind::InvalidData, "Cannot read, stream ended"))
        } else {
            Ok(u16::from_be_bytes(buf))
        }
    }

    fn read_u32(&mut self) -> Result<u32> {
        let mut buf = [0; 4];
        let read = self.read(&mut buf)?;
        if read != 4 {
            Err(Error::new(ErrorKind::InvalidData, "Cannot read, stream ended"))
        } else {
            Ok(u32::from_be_bytes(buf))
        }
    }

    fn read_u64(&mut self) -> Result<u64> {
        let mut buf = [0; 8];
        let read = self.read(&mut buf)?;
        if read != 8 {
            Err(Error::new(ErrorKind::InvalidData, "Cannot read, stream ended"))
        } else {
            Ok(u64::from_be_bytes(buf))
        }
    }

    fn read_u128(&mut self) -> Result<u128> {
        let mut buf = [0; 16];
        let read = self.read(&mut buf)?;
        if read != 16 {
            Err(Error::new(ErrorKind::InvalidData, "Cannot read, stream ended"))
        } else {
            Ok(u128::from_be_bytes(buf))
        }
    }

    fn read_i8(&mut self) -> std::io::Result<i8> {
        let mut buf = [0; 1];
        let read = self.read(&mut buf)?;
        if read != 1 {
            Err(Error::new(ErrorKind::InvalidData, "Cannot read, stream ended"))
        } else {
            Ok(i8::from_be_bytes(buf))
        }
    }

    fn read_i16(&mut self) -> Result<i16> {
        let mut buf = [0; 2];
        let read = self.read(&mut buf)?;
        if read != 2 {
            Err(Error::new(ErrorKind::InvalidData, "Cannot read, stream ended"))
        } else {
            Ok(i16::from_be_bytes(buf))
        }
    }

    fn read_i32(&mut self) -> Result<i32> {
        let mut buf = [0; 4];
        let read = self.read(&mut buf)?;
        if read != 4 {
            Err(Error::new(ErrorKind::InvalidData, "Cannot read, stream ended"))
        } else {
            Ok(i32::from_be_bytes(buf))
        }
    }

    fn read_i64(&mut self) -> Result<i64> {
        let mut buf = [0; 8];
        let read = self.read(&mut buf)?;
        if read != 8 {
            Err(Error::new(ErrorKind::InvalidData, "Cannot read, stream ended"))
        } else {
            Ok(i64::from_be_bytes(buf))
        }
    }

    fn read_i128(&mut self) -> Result<i128> {
        let mut buf = [0; 16];
        let read = self.read(&mut buf)?;
        if read != 16 {
            Err(Error::new(ErrorKind::InvalidData, "Cannot read, stream ended"))
        } else {
            Ok(i128::from_be_bytes(buf))
        }
    }


    fn read_f32(&mut self) -> Result<f32> {
        let mut buf = [0; 4];
        let read = self.read(&mut buf)?;
        if read != 4 {
            Err(Error::new(ErrorKind::InvalidData, "Cannot read, stream ended"))
        } else {
            Ok(f32::from_be_bytes(buf))
        }
    }

    fn read_f64(&mut self) -> Result<f64> {
        let mut buf = [0; 8];
        let read = self.read(&mut buf)?;
        if read != 8 {
            Err(Error::new(ErrorKind::InvalidData, "Cannot read, stream ended"))
        } else {
            Ok(f64::from_be_bytes(buf))
        }
    }

    fn read_varint(&mut self) -> Result<i32> {
        let mut num_read = 0;
        let mut result: i32 = 0;
        let mut read = 0b10000000;
        while read & 0b10000000 != 0 {
            read = self.read_u8()?;
            let value = read & 0b01111111;
            result |= (value as i32) << (7 * num_read as i32);

            num_read += 1;
            if num_read > 5 {
                return Err(Error::new(ErrorKind::InvalidData, "VarInt too big"));
            }
        }
        Ok(result)
    }

    fn read_varlong(&mut self) -> Result<i64> {
        let mut num_read = 0;
        let mut result: i64 = 0;
        let mut read = 0b10000000;
        while read & 0b10000000 != 0 {
            read = self.read_u8()?;
            let value = read & 0b01111111;
            result |= (value as i64) << (7 * num_read as i64);

            num_read += 1;
            if num_read > 10 {
                return Err(Error::new(ErrorKind::InvalidData, "VarLong too big"));
            }
        }
        Ok(result)
    }

    fn read_string(&mut self) -> Result<String> {
        let size = self.read_varint()?;
        if size < 0 {
            return Err(Error::new(ErrorKind::InvalidData, "Negative string size"));
        }
        let mut buf = vec![0; size as usize];
        let size = self.read(&mut buf)?;
        if size != buf.len() {
            return Err(Error::new(ErrorKind::InvalidData, "Cannot read string: Stream ended"));
        }
        let result = String::from_utf8(buf);
        if let Ok(result) = result {
            Ok(result)
        } else {
            Err(Error::new(ErrorKind::InvalidData, "Invalid UTF-8: ".to_string() + &result.unwrap_err().to_string()))
        }
    }

    fn read_t<T: 'static>(&mut self) -> Result<T> {
        let u8_type_id = TypeId::of::<u8>();
        let u16_type_id = TypeId::of::<u16>();
        let u32_type_id = TypeId::of::<u32>();
        let u64_type_id = TypeId::of::<u64>();
        let u128_type_id = TypeId::of::<u128>();
        let type_id = TypeId::of::<T>();

        if type_id == u8_type_id {
            unsafe { std::mem::transmute_copy(&self.read_u8()) }
        } else if type_id == u16_type_id {
            unsafe { std::mem::transmute_copy(&self.read_u16()) }
        } else if type_id == u32_type_id {
            unsafe { std::mem::transmute_copy(&self.read_u32()) }
        } else if type_id == u64_type_id {
            unsafe { std::mem::transmute_copy(&self.read_u64()) }
        } else if type_id == u128_type_id {
            unsafe { std::mem::transmute_copy(&self.read_u128()) }
        } else {
            Err(Error::new(ErrorKind::NotFound, "Unknown type. Supported types are: [u8, u16, u32, u64, u128]"))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Cursor};
    use crate::bytebuf::ByteBuf;

    #[test]
    fn test_u8() {
        assert_eq!(create_read().read_u8().unwrap(), 1);
    }

    #[test]
    fn test_u16() {
        assert_eq!(create_read().read_u16().unwrap(), 1 * 256 + 2);
    }

    #[test]
    fn test_u32() {
        assert_eq!(create_read().read_u32().unwrap(), 1 * 256 * 256 * 256 + 2 * 256 * 256 + 3 * 256 + 4);
    }

    #[test]
    fn test_u64() {
        assert_eq!(create_read().read_u64().unwrap(), 1 * 256 * 256 * 256 * 256 * 256 * 256 * 256 + 2 * 256 * 256 * 256 * 256 * 256 * 256 + 3 * 256 * 256 * 256 * 256 * 256 + 4 * 256 * 256 * 256 * 256 + 5 * 256 * 256 * 256 + 6 * 256 * 256 + 7 * 256 + 8);
    }

    #[test]
    fn test_u128() {
        // too many * 256
    }

    #[test]
    fn test_i8() {
        assert_eq!(create_read().read_i8().unwrap(), 1);
    }

    #[test]
    fn test_i16() {
        assert_eq!(create_read().read_i16().unwrap(), 1 * 256 + 2);
    }

    #[test]
    fn test_i32() {
        assert_eq!(create_read().read_i32().unwrap(), 1 * 256 * 256 * 256 + 2 * 256 * 256 + 3 * 256 + 4);
    }

    #[test]
    fn test_i64() {
        assert_eq!(create_read().read_i64().unwrap(), 1 * 256 * 256 * 256 * 256 * 256 * 256 * 256 + 2 * 256 * 256 * 256 * 256 * 256 * 256 + 3 * 256 * 256 * 256 * 256 * 256 + 4 * 256 * 256 * 256 * 256 + 5 * 256 * 256 * 256 + 6 * 256 * 256 + 7 * 256 + 8);
    }

    #[test]
    fn test_f32() {
        assert_eq!(create_read().read_f32().unwrap(), 0.000000000000000000000000000000000000023879393);
    }

    #[test]
    fn test_f64() {
        assert_eq!(create_read().read_f64().unwrap(), 0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000820788039913184);
    }

    #[test]
    fn test_varint() {
        assert_eq!(Box::new(Cursor::new(vec!(0))).read_varint().unwrap(), 0);
        assert_eq!(Box::new(Cursor::new(vec!(1))).read_varint().unwrap(), 1);
        assert_eq!(Box::new(Cursor::new(vec!(128, 1))).read_varint().unwrap(), 128);
        assert_eq!(Box::new(Cursor::new(vec!(255, 1))).read_varint().unwrap(), 255);
        assert_eq!(Box::new(Cursor::new(vec!(255, 255, 127))).read_varint().unwrap(), 2097151);
        assert_eq!(Box::new(Cursor::new(vec!(255, 255, 255, 255, 7))).read_varint().unwrap(), 2147483647);
        assert_eq!(Box::new(Cursor::new(vec!(255, 255, 255, 255, 15))).read_varint().unwrap(), -1);
        assert_eq!(Box::new(Cursor::new(vec!(128, 128, 128, 128, 8))).read_varint().unwrap(), -2147483648);
    }

    #[test]
    fn test_varlong() {
        assert_eq!(Box::new(Cursor::new(vec!(255, 255, 127))).read_varlong().unwrap(), 2097151);
        assert_eq!(Box::new(Cursor::new(vec!(255, 255, 255, 255, 7))).read_varlong().unwrap(), 2147483647);
        assert_eq!(Box::new(Cursor::new(vec!(255, 255, 255, 255, 255, 255, 255, 255, 127))).read_varlong().unwrap(), 9223372036854775807);
        assert_eq!(Box::new(Cursor::new(vec!(255, 255, 255, 255, 255, 255, 255, 255, 255, 1))).read_varlong().unwrap(), -1);
        assert_eq!(Box::new(Cursor::new(vec!(255, 255, 255, 255, 7))).read_varlong().unwrap(), 2147483647);
    }

    #[test]
    fn test_chained() {
        let mut read = create_read();
        assert_eq!(read.read_i8().unwrap(), 1);
        assert_eq!(read.read_u16().unwrap(), 2 * 256 + 3);
        assert_eq!(read.read_u8().unwrap(), 4);
    }

    #[test]
    fn test_generic() {
        assert_eq!(create_read().read_t::<u8>().unwrap(), 1);
        assert_eq!(create_read().read_t::<u16>().unwrap(), 1 * 256 + 2);
    }

    fn create_read() -> Box<dyn Read> {
        Box::new(Cursor::new(vec!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16)))
    }
}