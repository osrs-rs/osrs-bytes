//! A Rust library for working with the Oldschool Runescape data types.
//!
//! Data types in Oldschool Runescape are slightly different compared to normal types. Example of these types are the smart type, middle endian, and occassional switching to little endian. Therefore it has been seen as necessary to have a buffer that can work with these data types.
//!
//! This crate provides Read and Write extensions for working with the data types on any data structure implementing `&[u8]` such as Vec, Cursor etc.

use std::io::{self, Error, ErrorKind, Read, Result, Write};

pub trait ReadExt: Read {
    /// Reads an unsigned byte
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::io::Cursor;
    /// use osrs_buffer::ReadExt;
    ///
    /// let mut rdr = Cursor::new(vec![2, 5]);
    /// assert_eq!(rdr.read_u8().unwrap(), 2);
    /// assert_eq!(rdr.read_u8().unwrap(), 5);
    /// ```
    #[inline]
    fn read_u8(&mut self) -> Result<u8> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    /// Reads a signed byte
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::io::Cursor;
    /// use osrs_buffer::ReadExt;
    ///
    /// let mut rdr = Cursor::new(vec![248, 6]);
    /// assert_eq!(rdr.read_i8().unwrap(), -8);
    /// assert_eq!(rdr.read_i8().unwrap(), 6);
    /// ```
    #[inline]
    fn read_i8(&mut self) -> Result<i8> {
        Ok(self.read_u8()? as i8)
    }

    /// Reads a bool
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::io::Cursor;
    /// use osrs_buffer::ReadExt;
    ///
    /// let mut rdr = Cursor::new(vec![0, 1, 2]);
    /// assert_eq!(rdr.read_bool().unwrap(), false);
    /// assert_eq!(rdr.read_bool().unwrap(), true);
    /// assert_eq!(rdr.read_bool().unwrap(), true);
    /// ```
    #[inline]
    fn read_bool(&mut self) -> Result<bool> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0] != 0)
    }

    /// Reads an unsigned short as big endian
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::io::Cursor;
    /// use osrs_buffer::ReadExt;
    ///
    /// let mut rdr = Cursor::new(vec![66, 89]);
    /// assert_eq!(rdr.read_u16().unwrap(), 16985);
    /// ```
    #[inline]
    fn read_u16(&mut self) -> Result<u16> {
        let mut buf = [0; 2];
        self.read_exact(&mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }

    /// Reads an unsigned short as little endian
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::io::Cursor;
    /// use osrs_buffer::ReadExt;
    ///
    /// let mut rdr = Cursor::new(vec![89, 66]);
    /// assert_eq!(rdr.read_u16_le().unwrap(), 16985);
    /// ```
    #[inline]
    fn read_u16_le(&mut self) -> Result<u16> {
        let mut buf = [0; 2];
        self.read_exact(&mut buf)?;
        Ok(u16::from_le_bytes(buf))
    }

    /// Reads an unsigned short as big endian
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::io::Cursor;
    /// use osrs_buffer::ReadExt;
    ///
    /// let mut rdr = Cursor::new(vec![99, 130]);
    /// assert_eq!(rdr.read_u16_add().unwrap(), 25346);
    /// ```
    #[inline]
    fn read_u16_add(&mut self) -> Result<u16> {
        Ok(((self.read_u8()? as u16) << 8) | ((self.read_u8()?.wrapping_sub(128)) as u16))
    }

    /// Reads an unsigned short add as little endian
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::io::Cursor;
    /// use osrs_buffer::ReadExt;
    ///
    /// let mut rdr = Cursor::new(vec![89, 66]);
    /// assert_eq!(rdr.read_u16_add_le().unwrap(), 17113);
    /// ```
    #[inline]
    fn read_u16_add_le(&mut self) -> Result<u16> {
        Ok(((self.read_u8()?.wrapping_sub(128)) as u16) | ((self.read_u8()? as u16) << 8))
    }

    /// Reads a signed short as big endian
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::io::Cursor;
    /// use osrs_buffer::ReadExt;
    ///
    /// let mut rdr = Cursor::new(vec![255, 98]);
    /// assert_eq!(rdr.read_i16().unwrap(), -158);
    /// ```
    #[inline]
    fn read_i16(&mut self) -> Result<i16> {
        Ok(self.read_u16()? as i16)
    }

    /// Reads a signed short as little endian
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::io::Cursor;
    /// use osrs_buffer::ReadExt;
    ///
    /// let mut rdr = Cursor::new(vec![98, 255]);
    /// assert_eq!(rdr.read_i16_le().unwrap(), -158);
    /// ```
    #[inline]
    fn read_i16_le(&mut self) -> Result<i16> {
        Ok(self.read_u16_le()? as i16)
    }

    /// Reads a signed short add
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::io::Cursor;
    /// use osrs_buffer::ReadExt;
    ///
    /// let mut rdr = Cursor::new(vec![253, 177]);
    /// assert_eq!(rdr.read_i16_add().unwrap(), -719);
    /// ```
    #[inline]
    fn read_i16_add(&mut self) -> Result<i16> {
        Ok(self.read_u16_add()? as i16)
    }

    /// Reads an unsigned short add as little endian
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::io::Cursor;
    /// use osrs_buffer::ReadExt;
    ///
    /// let mut rdr = Cursor::new(vec![98, 255]);
    /// assert_eq!(rdr.read_i16_add_le().unwrap(), -30);
    /// ```
    #[inline]
    fn read_i16_add_le(&mut self) -> Result<i16> {
        Ok(self.read_u16_add_le()? as i16)
    }

    /// Reads an unsigned dword as big endian
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::io::Cursor;
    /// use osrs_buffer::ReadExt;
    ///
    /// let mut rdr = Cursor::new(vec![42, 87, 33, 16]);
    /// assert_eq!(rdr.read_u32().unwrap(), 710353168);
    /// ```
    #[inline]
    fn read_u32(&mut self) -> Result<u32> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        Ok(u32::from_be_bytes(buf))
    }

    /// Reads an unsigned dword as little endian
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::io::Cursor;
    /// use osrs_buffer::ReadExt;
    ///
    /// let mut rdr = Cursor::new(vec![16, 33, 87, 42]);
    /// assert_eq!(rdr.read_u32_le().unwrap(), 710353168);
    /// ```
    #[inline]
    fn read_u32_le(&mut self) -> Result<u32> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }

    /// Reads an unsigned dword as middle endian
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::io::Cursor;
    /// use osrs_buffer::ReadExt;
    ///
    /// let mut rdr = Cursor::new(vec![1, 5, 9, 49]);
    /// assert_eq!(rdr.read_u32_me().unwrap(), 83964169);
    ///
    /// ```
    #[inline]
    fn read_u32_me(&mut self) -> Result<u32> {
        Ok((self.read_u16_le()? as u32) << 16 | (self.read_u16_le()? as u32))
    }

    /// Reads an unsigned dword as inversed middle endian
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::io::Cursor;
    /// use osrs_buffer::ReadExt;
    ///
    /// let mut rdr = Cursor::new(vec![0, 0, 0, 149]);
    /// assert_eq!(rdr.read_u32_ime().unwrap(), 9764864);
    ///
    /// ```
    #[inline]
    fn read_u32_ime(&mut self) -> Result<u32> {
        Ok((self.read_u16()? as u32) | ((self.read_u16()? as u32) << 16))
    }

    /// Reads a signed dword as big endian
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::io::Cursor;
    /// use osrs_buffer::ReadExt;
    ///
    /// let mut rdr = Cursor::new(vec![255, 87, 33, 16]);
    /// assert_eq!(rdr.read_i32().unwrap(), -11067120);
    /// ```
    #[inline]
    fn read_i32(&mut self) -> Result<i32> {
        Ok(self.read_u32()? as i32)
    }

    /// Reads an signed dword as little endian
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::io::Cursor;
    /// use osrs_buffer::ReadExt;
    ///
    /// let mut rdr = Cursor::new(vec![16, 33, 87, 250]);
    /// assert_eq!(rdr.read_i32_le().unwrap(), -94953200);
    /// ```
    #[inline]
    fn read_i32_le(&mut self) -> Result<i32> {
        Ok(self.read_u32_le()? as i32)
    }

    /// Reads an signed dword as middle endian
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::io::Cursor;
    /// use osrs_buffer::ReadExt;
    ///
    /// let mut rdr = Cursor::new(vec![0, 149, 0, 0]);
    /// assert_eq!(rdr.read_i32_me().unwrap(), -1795162112);
    ///
    /// ```
    #[inline]
    fn read_i32_me(&mut self) -> Result<i32> {
        Ok(self.read_u32_me()? as i32)
    }

    /// Reads an unsigned dword as inversed middle endian
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::io::Cursor;
    /// use osrs_buffer::ReadExt;
    ///
    /// let mut rdr = Cursor::new(vec![118, 195, 254, 193]);
    /// assert_eq!(rdr.read_i32_ime().unwrap(), -20875581);
    ///
    /// ```
    #[inline]
    fn read_i32_ime(&mut self) -> Result<i32> {
        Ok(self.read_u32_ime()? as i32)
    }

    /// Reads an unsigned dword as big endian
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::io::Cursor;
    /// use osrs_buffer::ReadExt;
    ///
    /// let mut rdr = Cursor::new(vec![31, 84, 11, 99, 45, 12, 94, 36]);
    /// assert_eq!(rdr.read_u64().unwrap(), 2257441833804914212);
    /// ```
    #[inline]
    fn read_u64(&mut self) -> Result<u64> {
        let mut buf = [0; 8];
        self.read_exact(&mut buf)?;
        Ok(u64::from_be_bytes(buf))
    }

    /// Reads an signed dword as big endian
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::io::Cursor;
    /// use osrs_buffer::ReadExt;
    ///
    /// let mut rdr = Cursor::new(vec![255, 84, 11, 99, 45, 12, 94, 36]);
    /// assert_eq!(rdr.read_i64().unwrap(), -48401175408779740);
    /// ```
    #[inline]
    fn read_i64(&mut self) -> Result<i64> {
        Ok(self.read_u64()? as i64)
    }

    /// Reads a CP1252 string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::io::Cursor;
    /// use osrs_buffer::ReadExt;
    ///
    /// let mut rdr = Cursor::new(vec![109, 121, 32, 116, 101, 115, 116, 0]);
    /// assert_eq!(rdr.read_string_cp1252().unwrap(), "my test");
    /// ```
    #[inline]
    fn read_string_cp1252(&mut self) -> Result<String> {
        let mut str = Vec::new();

        while let Ok(x) = self.read_u8() {
            if x != 0 {
                str.push(x);
            } else {
                break;
            }
        }

        let s = match std::str::from_utf8(&str) {
            Ok(v) => v,
            Err(e) => {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("Invalid UTF-8 sequence: {}", e),
                ))
            }
        };

        Ok(s.to_owned())
    }
}

impl<R: io::Read + ?Sized> ReadExt for R {}

pub trait WriteExt: Write {
    /// Writes an unsigned byte to the writer.
    ///
    /// Examples
    ///
    /// ```rust
    /// use osrs_buffer::WriteExt;
    ///
    /// let mut wtr = Vec::new();
    /// wtr.write_u8(42).unwrap();
    /// assert_eq!(wtr[0], 42);
    /// ```
    #[inline]
    fn write_u8(&mut self, n: u8) -> Result<()> {
        self.write_all(&[n])
    }

    /// Writes an signed byte to the writer.
    ///
    /// Examples
    ///
    /// ```rust
    /// use osrs_buffer::WriteExt;
    ///
    /// let mut wtr = Vec::new();
    /// wtr.write_i8(-67).unwrap();
    /// assert_eq!(wtr[0] as i8, -67);
    /// ```
    #[inline]
    fn write_i8(&mut self, n: i8) -> Result<()> {
        self.write_u8(n as u8)
    }

    /// Writes the number 128, subtracted by the signed byte to the writer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use osrs_buffer::WriteExt;
    ///
    /// let mut wtr = Vec::new();
    /// wtr.write_i8_sub(99).unwrap();
    /// assert_eq!(wtr[0] as i8, 29);
    /// ```
    #[inline]
    fn write_i8_sub(&mut self, n: i8) -> Result<()> {
        self.write_u8(128 - n as u8)
    }

    /// Writes the byte and adds 128.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use osrs_buffer::WriteExt;
    ///
    /// let mut wtr = Vec::new();
    /// wtr.write_i8_add(42).unwrap();
    /// assert_eq!(wtr[0], 170);
    /// ```
    #[inline]
    fn write_i8_add(&mut self, n: i8) -> Result<()> {
        self.write_u8(n as u8 + 128)
    }

    /// Writes a negated byte to the writer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use osrs_buffer::WriteExt;
    ///
    /// let mut wtr = Vec::new();
    /// wtr.write_i8_neg(55).unwrap();
    /// assert_eq!(wtr[0], 201);
    /// ```
    #[inline]
    fn write_i8_neg(&mut self, n: i8) -> Result<()> {
        self.write_u8(-n as u8)
    }

    /// Writes a bool to the writer.
    ///
    /// Examples
    ///
    /// ```rust
    /// use osrs_buffer::WriteExt;
    ///
    /// let mut wtr = Vec::new();
    /// wtr.write_bool(true).unwrap();
    /// assert_eq!(wtr[0], 1);
    /// ```
    #[inline]
    fn write_bool(&mut self, b: bool) -> Result<()> {
        self.write_all(&[b as u8])
    }

    /// Writes an unsigned short to the writer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use osrs_buffer::WriteExt;
    ///
    /// let mut wtr = Vec::new();
    /// wtr.write_u16(20065).unwrap();
    /// assert_eq!(wtr[0], 78);
    /// assert_eq!(wtr[1], 97);
    /// ```
    #[inline]
    fn write_u16(&mut self, n: u16) -> Result<()> {
        self.write_all(&n.to_be_bytes())
    }

    /// Writes an unsigned short as a little endian to the writer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use osrs_buffer::WriteExt;
    ///
    /// let mut wtr = Vec::new();
    /// wtr.write_u16_le(29543).unwrap();
    /// assert_eq!(wtr[0], 103);
    /// assert_eq!(wtr[1], 115);
    /// ```
    ///
    #[inline]
    fn write_u16_le(&mut self, n: u16) -> Result<()> {
        self.write_all(&n.to_le_bytes())
    }

    /// Writes an unsigned short smart to the writer.
    ///
    /// # Examples
    ///
    /// Writing a value lesser than or equal to 127 makes it write out a single unsigned byte.
    ///
    /// ```rust
    /// use osrs_buffer::WriteExt;
    ///
    /// let mut wtr = Vec::new();
    /// wtr.write_u16_smart(65).unwrap();
    /// assert_eq!(wtr[0], 65);
    /// assert!(wtr.get(1).is_none());
    /// ```
    ///
    /// Writing a value greater than 127 will make it write out two unsigned bytes.
    ///
    /// ```rust
    /// use osrs_buffer::WriteExt;
    ///
    /// let mut wtr = Vec::new();
    /// wtr.write_u16_smart(986).unwrap();
    /// assert_eq!(wtr[0], 131);
    /// assert_eq!(wtr[1], 218);
    /// ```
    ///
    #[inline]
    fn write_u16_smart(&mut self, n: u16) -> Result<()> {
        match n {
            0..=127 => self.write_u8(n as u8),
            128..=32767 => self.write_u16(n + 32768),
            _ => Err(Error::new(
                ErrorKind::Other,
                format!("Failed writing smart, value is {}", n),
            )),
        }
    }

    /// Writes a signed short to the writer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use osrs_buffer::WriteExt;
    ///
    /// let mut wtr = Vec::new();
    /// wtr.write_i16(-14632).unwrap();
    /// assert_eq!(wtr[0], 198);
    /// assert_eq!(wtr[1], 216);
    /// ```
    #[inline]
    fn write_i16(&mut self, n: i16) -> Result<()> {
        self.write_u16(n as u16)
    }

    /// Writes a signed short as little endian to the writer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use osrs_buffer::WriteExt;
    ///
    /// let mut wtr = Vec::new();
    /// wtr.write_i16_le(-7654).unwrap();
    /// assert_eq!(wtr[0], 26);
    /// assert_eq!(wtr[1], 226);
    /// ```
    #[inline]
    fn write_i16_le(&mut self, n: i16) -> Result<()> {
        self.write_u16_le(n as u16)
    }

    /// Writes a signed short add to the writer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use osrs_buffer::WriteExt;
    ///
    /// let mut wtr = Vec::new();
    /// wtr.write_i16_add(-9867).unwrap();
    /// assert_eq!(wtr[0], 217);
    /// assert_eq!(wtr[1], 245);
    /// ```
    ///
    #[inline]
    fn write_i16_add(&mut self, n: i16) -> Result<()> {
        self.write_i8((n >> 8) as i8)?;
        self.write_i8((n + 128) as i8)
    }

    /// Writes a signed short add as a little endian to the writer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use osrs_buffer::WriteExt;
    ///
    /// let mut wtr = Vec::new();
    /// wtr.write_i16_le_add(-12632).unwrap();
    /// assert_eq!(wtr[0], 40);
    /// assert_eq!(wtr[1], 206);
    /// ```
    ///
    #[inline]
    fn write_i16_le_add(&mut self, n: i16) -> Result<()> {
        self.write_i8((n + 128) as i8)?;
        self.write_i8((n >> 8) as i8)
    }

    /// Writes an unsigned dword to the writer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use osrs_buffer::WriteExt;
    ///
    /// let mut wtr = Vec::new();
    /// wtr.write_u32(98571).unwrap();
    /// assert_eq!(wtr[0], 0);
    /// assert_eq!(wtr[1], 1);
    /// assert_eq!(wtr[2], 129);
    /// assert_eq!(wtr[3], 11);
    /// ```
    ///
    #[inline]
    fn write_u32(&mut self, n: u32) -> Result<()> {
        self.write_all(&n.to_be_bytes())
    }

    /// Writes am unsigned integer as little endian to the writer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use osrs_buffer::WriteExt;
    ///
    /// let mut wtr = Vec::new();
    /// wtr.write_u32_le(26904).unwrap();
    /// assert_eq!(wtr[0], 24);
    /// assert_eq!(wtr[1], 105);
    /// assert_eq!(wtr[2], 0);
    /// assert_eq!(wtr[3], 0);
    /// ```
    ///
    #[inline]
    fn write_u32_le(&mut self, n: u32) -> Result<()> {
        self.write_all(&n.to_le_bytes())
    }

    /// Writes a signed dword to the writer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use osrs_buffer::WriteExt;
    ///
    /// let mut wtr = Vec::new();
    /// wtr.write_i32(-131045).unwrap();
    /// assert_eq!(wtr[0], 255);
    /// assert_eq!(wtr[1], 254);
    /// assert_eq!(wtr[2], 0);
    /// assert_eq!(wtr[3], 27);
    /// ```
    ///
    #[inline]
    fn write_i32(&mut self, n: i32) -> Result<()> {
        self.write_u32(n as u32)
    }

    /// Writes a signed integer as little endian to the writer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use osrs_buffer::WriteExt;
    ///
    /// let mut wtr = Vec::new();
    /// wtr.write_i32_le(18879).unwrap();
    /// assert_eq!(wtr[0], 191);
    /// assert_eq!(wtr[1], 73);
    /// assert_eq!(wtr[2], 0);
    /// assert_eq!(wtr[3], 0);
    /// ```
    ///
    #[inline]
    fn write_i32_le(&mut self, n: i32) -> Result<()> {
        self.write_u32_le(n as u32)
    }

    /// Writes a signed dword as a middle endian to the writer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use osrs_buffer::WriteExt;
    ///
    /// let mut wtr = Vec::new();
    /// wtr.write_i32_me(-98231).unwrap();
    /// assert_eq!(wtr[0], 254);
    /// assert_eq!(wtr[1], 255);
    /// assert_eq!(wtr[2], 73);
    /// assert_eq!(wtr[3], 128);
    /// ```
    ///
    #[inline]
    fn write_i32_me(&mut self, n: i32) -> Result<()> {
        self.write_i16_le((n >> 16) as i16)?;
        self.write_i16_le(n as i16)
    }

    /// Writes a signed dword as an inversed middle endian to the writer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use osrs_buffer::WriteExt;
    ///
    /// let mut wtr = Vec::new();
    /// wtr.write_i32_ime(-98231).unwrap();
    /// assert_eq!(wtr[0], 128);
    /// assert_eq!(wtr[1], 73);
    /// assert_eq!(wtr[2], 255);
    /// assert_eq!(wtr[3], 254);
    /// ```
    ///
    #[inline]
    fn write_i32_ime(&mut self, n: i32) -> Result<()> {
        self.write_i16(n as i16)?;
        self.write_i16((n >> 16) as i16)
    }

    /// Writes an unsigned qword to the writer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use osrs_buffer::WriteExt;
    ///
    /// let mut wtr = Vec::new();
    /// wtr.write_u64(8589934592).unwrap();
    /// assert_eq!(wtr[0], 0);
    /// assert_eq!(wtr[1], 0);
    /// assert_eq!(wtr[2], 0);
    /// assert_eq!(wtr[3], 2);
    /// assert_eq!(wtr[4], 0);
    /// assert_eq!(wtr[5], 0);
    /// assert_eq!(wtr[6], 0);
    /// assert_eq!(wtr[7], 0);
    /// ```
    ///
    #[inline]
    fn write_u64(&mut self, n: u64) -> Result<()> {
        self.write_all(&n.to_be_bytes())
    }

    /// Writes a signed qword to the writer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use osrs_buffer::WriteExt;
    ///
    /// let mut wtr = Vec::new();
    /// wtr.write_i64(-8589934592).unwrap();
    /// assert_eq!(wtr[0], 255);
    /// assert_eq!(wtr[1], 255);
    /// assert_eq!(wtr[2], 255);
    /// assert_eq!(wtr[3], 254);
    /// assert_eq!(wtr[4], 0);
    /// assert_eq!(wtr[5], 0);
    /// assert_eq!(wtr[6], 0);
    /// assert_eq!(wtr[7], 0);
    /// ```
    ///
    #[inline]
    fn write_i64(&mut self, n: i64) -> Result<()> {
        self.write_u64(n as u64)
    }

    /// Writes a CP1252 string to the writer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use osrs_buffer::WriteExt;
    ///
    /// let mut wtr = Vec::new();
    /// wtr.write_string_cp1252("hello").unwrap();
    /// assert_eq!(wtr[0], 104);
    /// assert_eq!(wtr[1], 101);
    /// assert_eq!(wtr[2], 108);
    /// assert_eq!(wtr[3], 108);
    /// assert_eq!(wtr[4], 111);
    /// assert_eq!(wtr[5], 0);
    /// ```
    ///
    #[inline]
    fn write_string_cp1252(&mut self, s: &str) -> Result<()> {
        for b in s.as_bytes() {
            self.write_u8(*b)?;
        }
        self.write_i8(0)
    }

    /// Write bytes reversed with add to the writer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use osrs_buffer::WriteExt;
    ///
    /// let wtr1 = vec![1, 2, 3];
    ///
    /// let mut wtr2 = Vec::new();
    /// wtr2.write_bytes_reversed_add(&wtr1);
    /// assert_eq!(wtr2[0], 131);
    /// assert_eq!(wtr2[1], 130);
    /// assert_eq!(wtr2[2], 129);
    /// ```
    ///
    #[inline]
    fn write_bytes_reversed_add(&mut self, buf: &[u8]) -> Result<()> {
        for b in buf.iter().rev() {
            self.write_i8(b.wrapping_add((i8::MAX as u8) + 1) as i8)?;
        }
        Ok(())
    }
}

impl<W: io::Write + ?Sized> WriteExt for W {}

/*
#[cfg(test)]
mod tests {
    //use super::*;

    // Tests that do not fit in documentation should be placed here.


    #[test]
    fn test_write_u8() {
        ...
    }
}
*/
