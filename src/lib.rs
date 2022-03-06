//! A Rust library for working with the Oldschool Runescape data types.
//!
//! Data types in Oldschool Runescape are slightly different compared to normal types. Example of these types are the smart type, middle endian, and occassional switching to little endian. Therefore it has been seen as necessary to have a buffer that can work with these data types.
//!
//! The buffer in this implementation is backed by a `Vec`, and a reading and writing position is used for knowing where the buffer can read and write from in the buffer.

use byteorder::{BigEndian, ByteOrder, LittleEndian};
use std::io::{Error, ErrorKind, Result};

/// The ByteBuffer for reading and writing binary data.
pub struct ByteBuffer {
    /// The bytes of the buffer, stored as a vector of unsigned bytes.
    pub data: Vec<u8>,
    /// The current position for reading data.
    pub read_pos: usize,
    /// The current position for writing data.
    pub write_pos: usize,
}

/// Implementations for the ByteBuffer
impl ByteBuffer {
    /// Initializes a new buffer with a given size.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let buf = ByteBuffer::new(5);
    ///
    /// assert_eq!(buf.data.len(), 5);
    ///
    /// ```
    pub fn new(size: usize) -> ByteBuffer {
        ByteBuffer {
            data: vec![0; size],
            read_pos: 0,
            write_pos: 0,
        }
    }

    /// Initializes a buffer given an existing buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let other_buf = ByteBuffer::new(9);
    /// let buf = ByteBuffer::from_buf(other_buf.data);
    ///
    /// assert_eq!(buf.data.len(), 9);
    ///
    /// ```
    pub fn from_buf(vec: Vec<u8>) -> ByteBuffer {
        ByteBuffer {
            data: vec,
            read_pos: 0,
            write_pos: 0,
        }
    }

    /// Clears the reading and writing position, resetting them to 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(1);
    /// buf.clear();
    ///
    /// assert_eq!(buf.read_pos, 0);
    /// assert_eq!(buf.write_pos, 0);
    ///
    /// ```
    pub fn clear(&mut self) {
        self.read_pos = 0;
        self.write_pos = 0;
    }

    /// Skips over data, increasing the reading and writing position of the buffer by the specified amount.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(5);
    /// buf.skip(3);
    ///
    /// assert_eq!(buf.read_pos, 3);
    /// assert_eq!(buf.write_pos, 3);
    ///
    /// ```
    pub fn skip(&mut self, skip: usize) {
        self.read_pos += skip;
        self.write_pos += skip;
    }

    /// Writes a given amount of bytes to the buffer, increasing the writing position as a result.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(5);
    /// buf.write_bytes(&[8,9,2]);
    ///
    /// assert_eq!(buf.data[0], 8);
    /// assert_eq!(buf.data[1], 9);
    /// assert_eq!(buf.data[2], 2);
    /// assert_eq!(buf.write_pos, 3);
    ///
    /// ```
    pub fn write_bytes(&mut self, bytes: &[u8]) {
        //self.flush_bit();

        let size = bytes.len() + self.write_pos;

        if size > self.data.len() {
            panic!("write size greater than bytes length")
        }

        for v in bytes {
            self.data[self.write_pos] = *v;
            self.write_pos += 1;
        }
    }

    /// Writes a bool to the buffer, increasing the writing position by 1.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(1);
    /// buf.write_bool(true);
    ///
    /// assert_eq!(buf.data[0], 1);
    /// assert_eq!(buf.write_pos, 1);
    ///
    /// ```
    pub fn write_bool(&mut self, val: bool) {
        self.write_bytes(&[val as u8]);
    }

    /// Writes an unsigned byte to the buffer, increasing the writing position by 1.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(1);
    /// buf.write_u8(42);
    ///
    /// assert_eq!(buf.data[0], 42);
    /// assert_eq!(buf.write_pos, 1);
    ///
    /// ```
    pub fn write_u8(&mut self, val: u8) {
        self.write_bytes(&[val]);
    }

    /// Writes a signed byte to the buffer, increasing the writing position by 1.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(1);
    /// buf.write_i8(-67);
    ///
    /// assert_eq!(buf.data[0] as i8, -67);
    /// assert_eq!(buf.write_pos, 1);
    ///
    /// ```
    pub fn write_i8(&mut self, val: i8) {
        self.write_u8(val as u8);
    }

    /// Writes the number 128, subtracted by the signed byte to the buffer, increasing the writing position by 1.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(1);
    /// buf.write_i8_sub(99);
    ///
    /// assert_eq!(buf.data[0], 29);
    /// assert_eq!(buf.write_pos, 1);
    ///
    /// ```
    pub fn write_i8_sub(&mut self, val: i8) {
        self.write_u8(128 - val as u8);
    }

    /// Writes the byte and adds 128 to it, increasing the writing position by 1.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(1);
    /// buf.write_i8_add(42);
    ///
    /// assert_eq!(buf.data[0], 170);
    /// assert_eq!(buf.write_pos, 1);
    ///
    /// ```
    pub fn write_i8_add(&mut self, val: i8) {
        self.write_u8(val as u8 + 128);
    }

    /// Writes a negated byte to the buffer, increasing the writing position by 1.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(1);
    /// buf.write_i8_neg(55);
    ///
    /// assert_eq!(buf.data[0], 201);
    /// assert_eq!(buf.write_pos, 1);
    ///
    /// ```
    pub fn write_i8_neg(&mut self, val: i8) {
        self.write_u8(-val as u8);
    }

    /// Writes an unsigned short to the buffer, increasing the writing position by 2.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(2);
    /// buf.write_u16(20065);
    ///
    /// assert_eq!(buf.data[0], 78);
    /// assert_eq!(buf.data[1], 97);
    /// assert_eq!(buf.write_pos, 2);
    ///
    /// ```
    pub fn write_u16(&mut self, val: u16) {
        let mut buf = [0; 2];

        BigEndian::write_u16(&mut buf, val);

        self.write_bytes(&buf);
    }

    /// Writes an unsigned short smart to the buffer, increasing the writing position by 2.
    ///
    /// # Examples
    ///
    /// Writing a value lesser than or equal to 127 makes it write out a single unsigned byte.
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(2);
    /// buf.write_u16_smart(65);
    ///
    /// assert_eq!(buf.data[0], 65);
    /// assert_eq!(buf.data[1], 0);
    /// assert_eq!(buf.write_pos, 1);
    ///
    /// ```
    ///
    /// Writing a value greater than 127 will make it write out two unsigned bytes.
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(2);
    /// buf.write_u16_smart(986);
    ///
    /// assert_eq!(buf.data[0], 131);
    /// assert_eq!(buf.data[1], 218);
    /// assert_eq!(buf.write_pos, 2);
    ///
    /// ```
    ///
    pub fn write_u16_smart(&mut self, val: u16) {
        match val {
            0..=127 => self.write_u8(val as u8),
            128..=32767 => self.write_u16(val + 32768),
            _ => panic!("Value {} is too big for write_i16_smart", val),
        }
    }

    /// Writes a signed short to the buffer, increasing the writing position by 2.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(2);
    /// buf.write_i16(-14632);
    ///
    /// assert_eq!(buf.data[0], 198);
    /// assert_eq!(buf.data[1], 216);
    /// assert_eq!(buf.write_pos, 2);
    ///
    /// ```
    ///
    pub fn write_i16(&mut self, val: i16) {
        self.write_u16(val as u16);
    }

    /// Writes a signed short add to the buffer, increasing the writing position by 2.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(2);
    /// buf.write_i16_add(-9867);
    ///
    /// assert_eq!(buf.data[0], 217);
    /// assert_eq!(buf.data[1], 245);
    /// assert_eq!(buf.write_pos, 2);
    ///
    /// ```
    ///
    pub fn write_i16_add(&mut self, val: i16) {
        self.write_i8((val >> 8) as i8);
        self.write_i8((val + 128) as i8);
    }

    /// Writes a signed short add as a little endian to the buffer, increasing the writing position by 2.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(2);
    /// buf.write_i16_le_add(-12632);
    ///
    /// assert_eq!(buf.data[0], 40);
    /// assert_eq!(buf.data[1], 206);
    /// assert_eq!(buf.write_pos, 2);
    ///
    /// ```
    ///
    pub fn write_i16_le_add(&mut self, val: i16) {
        self.write_i8((val + 128) as i8);
        self.write_i8((val >> 8) as i8);
    }

    /// Writes an unsigned short as a little endian to the buffer, increasing the writing position by 2.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(2);
    /// buf.write_u16_le(29543);
    ///
    /// assert_eq!(buf.data[0], 103);
    /// assert_eq!(buf.data[1], 115);
    /// assert_eq!(buf.write_pos, 2);
    ///
    /// ```
    ///
    pub fn write_u16_le(&mut self, val: u16) {
        let mut buf = [0; 2];

        LittleEndian::write_u16(&mut buf, val);

        self.write_bytes(&buf);
    }

    /// Writes a signed short as a little endian to the buffer, increasing the writing position by 2.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(2);
    /// buf.write_i16_le(-7654);
    ///
    /// assert_eq!(buf.data[0], 26);
    /// assert_eq!(buf.data[1], 226);
    /// assert_eq!(buf.write_pos, 2);
    ///
    /// ```
    ///
    pub fn write_i16_le(&mut self, val: i16) {
        self.write_u16_le(val as u16);
    }

    /// Writes an unsigned dword to the buffer, increasing the writing position by 4.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(4);
    /// buf.write_u32(98571);
    ///
    /// assert_eq!(buf.data[0], 0);
    /// assert_eq!(buf.data[1], 1);
    /// assert_eq!(buf.data[2], 129);
    /// assert_eq!(buf.data[3], 11);
    /// assert_eq!(buf.write_pos, 4);
    ///
    /// ```
    ///
    pub fn write_u32(&mut self, val: u32) {
        let mut buf = [0; 4];

        BigEndian::write_u32(&mut buf, val);

        self.write_bytes(&buf);
    }

    /// Writes a signed dword to the buffer, increasing the writing position by 4.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(4);
    /// buf.write_i32(-131045);
    ///
    /// assert_eq!(buf.data[0], 255);
    /// assert_eq!(buf.data[1], 254);
    /// assert_eq!(buf.data[2], 0);
    /// assert_eq!(buf.data[3], 27);
    /// assert_eq!(buf.write_pos, 4);
    ///
    /// ```
    ///
    pub fn write_i32(&mut self, val: i32) {
        self.write_u32(val as u32);
    }

    /// Writes a signed dword as a middle endian to the buffer, increasing the writing position by 4.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(4);
    /// buf.write_i32_me(-98231);
    ///
    /// assert_eq!(buf.data[0], 254);
    /// assert_eq!(buf.data[1], 255);
    /// assert_eq!(buf.data[2], 73);
    /// assert_eq!(buf.data[3], 128);
    /// assert_eq!(buf.write_pos, 4);
    ///
    /// ```
    ///
    pub fn write_i32_me(&mut self, val: i32) {
        self.write_i16_le((val >> 16) as i16);
        self.write_i16_le(val as i16);
    }

    /// Writes a signed dword as an inversed middle endian to the buffer, increasing the writing position by 4.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(4);
    /// buf.write_i32_ime(-98231);
    ///
    /// assert_eq!(buf.data[0], 128);
    /// assert_eq!(buf.data[1], 73);
    /// assert_eq!(buf.data[2], 255);
    /// assert_eq!(buf.data[3], 254);
    /// assert_eq!(buf.write_pos, 4);
    ///
    /// ```
    ///
    pub fn write_i32_ime(&mut self, val: i32) {
        self.write_i16(val as i16);
        self.write_i16((val >> 16) as i16);
    }

    /// Writes am unsigned integer as little endian to the buffer, increasing the writing position by 2.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(4);
    /// buf.write_u32_le(26904);
    ///
    /// assert_eq!(buf.data[0], 24);
    /// assert_eq!(buf.data[1], 105);
    /// assert_eq!(buf.data[2], 0);
    /// assert_eq!(buf.data[3], 0);
    /// assert_eq!(buf.write_pos, 4);
    ///
    /// ```
    ///
    pub fn write_u32_le(&mut self, val: u32) {
        let mut buf = [0; 4];

        LittleEndian::write_u32(&mut buf, val);

        self.write_bytes(&buf);
    }

    /// Writes a signed integer as little endian to the buffer, increasing the writing position by 2.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(4);
    /// buf.write_i32_le(18879);
    ///
    /// assert_eq!(buf.data[0], 191);
    /// assert_eq!(buf.data[1], 73);
    /// assert_eq!(buf.data[2], 0);
    /// assert_eq!(buf.data[3], 0);
    /// assert_eq!(buf.write_pos, 4);
    ///
    /// ```
    ///
    pub fn write_i32_le(&mut self, val: i32) {
        self.write_u32_le(val as u32);
    }

    /// Writes an unsigned qword to the buffer, increasing the writing position by 8.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(8);
    /// buf.write_u64(8589934592);
    ///
    /// assert_eq!(buf.data[0], 0);
    /// assert_eq!(buf.data[1], 0);
    /// assert_eq!(buf.data[2], 0);
    /// assert_eq!(buf.data[3], 2);
    /// assert_eq!(buf.data[4], 0);
    /// assert_eq!(buf.data[5], 0);
    /// assert_eq!(buf.data[6], 0);
    /// assert_eq!(buf.data[7], 0);
    /// assert_eq!(buf.write_pos, 8);
    ///
    /// ```
    ///
    pub fn write_u64(&mut self, val: u64) {
        let mut buf = [0; 8];

        BigEndian::write_u64(&mut buf, val);

        self.write_bytes(&buf);
    }

    /// Writes a signed qword to the buffer, increasing the writing position by 8.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(8);
    /// buf.write_i64(-8589934592);
    ///
    /// assert_eq!(buf.data[0], 255);
    /// assert_eq!(buf.data[1], 255);
    /// assert_eq!(buf.data[2], 255);
    /// assert_eq!(buf.data[3], 254);
    /// assert_eq!(buf.data[4], 0);
    /// assert_eq!(buf.data[5], 0);
    /// assert_eq!(buf.data[6], 0);
    /// assert_eq!(buf.data[7], 0);
    /// assert_eq!(buf.write_pos, 8);
    ///
    /// ```
    ///
    pub fn write_i64(&mut self, val: i64) {
        self.write_u64(val as u64);
    }

    /// Writes a null-terminated string to the buffer, increasing the writing position by the string's length, plus the null terminated byte.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(8);
    /// buf.write_string_null_terminated("hello");
    ///
    /// assert_eq!(buf.data[0], 104);
    /// assert_eq!(buf.data[1], 101);
    /// assert_eq!(buf.data[2], 108);
    /// assert_eq!(buf.data[3], 108);
    /// assert_eq!(buf.data[4], 111);
    /// assert_eq!(buf.data[5], 0);
    /// assert_eq!(buf.write_pos, 6);
    /// ```
    ///
    pub fn write_string_null_terminated(&mut self, s: &str) {
        // TODO: Check here for out of length

        for x in s.as_bytes() {
            self.data[self.write_pos] = *x;
            self.write_pos += 1;
        }
        self.write_i8(0);
    }

    /// Write bytes reversed with add, increasing the writing position by the length of the bytes written.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// // Create the other buffer with the data, and set its writing position manually
    /// let mut other_buf = ByteBuffer::new(3);
    /// other_buf.data[0] = 1;
    /// other_buf.data[1] = 2;
    /// other_buf.data[2] = 3;
    /// other_buf.write_pos = 3;
    ///
    /// // Create the buffer which the reversed bytes with add should be written to
    /// let mut buf = ByteBuffer::new(3);
    /// buf.write_bytes_reversed_add(&other_buf);
    ///
    /// assert_eq!(buf.data[0], 131);
    /// assert_eq!(buf.data[1], 130);
    /// assert_eq!(buf.data[2], 129);
    /// assert_eq!(buf.write_pos, 3);
    /// ```
    ///
    pub fn write_bytes_reversed_add(&mut self, buf: &ByteBuffer) {
        for i in (buf.read_pos..buf.write_pos).rev() {
            self.write_i8(buf.data.get(i).unwrap().wrapping_add(128) as i8);
        }
    }

    /// Read bytes from the current buffer into another buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// // Create the other buffer with the data
    /// let mut other_buf = ByteBuffer::new(3);
    /// other_buf.data[0] = 99;
    /// other_buf.data[1] = 54;
    /// other_buf.data[2] = 31;
    ///
    /// // Create the buffer which should read the bytes from the other buffer,
    /// // and pass it to read_bytes(...) along with the length
    /// let mut buf = ByteBuffer::new(3);
    /// other_buf.read_bytes(&mut buf, 3);
    ///
    /// assert_eq!(buf.data[0], 99);
    /// assert_eq!(buf.data[1], 54);
    /// assert_eq!(buf.data[2], 31);
    /// assert_eq!(buf.write_pos, 3);
    /// ```
    ///
    pub fn read_bytes(&mut self, buf: &mut ByteBuffer, size: usize) -> Result<()> {
        if self.read_pos + size > self.data.len() {
            return Err(Error::new(
                ErrorKind::UnexpectedEof,
                "could not read enough bytes from buffer",
            ));
        }
        let range = self.read_pos..self.read_pos + size;
        buf.write_bytes(&self.data[range]);
        self.read_pos += size;
        Ok(())
    }

    /// Reads a bool from the buffer, increasing the reading position by 1.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(1);
    /// buf.data[0] = 1;
    ///
    /// assert_eq!(buf.read_bool(), true);
    /// assert_eq!(buf.read_pos, 1);
    ///
    /// ```
    pub fn read_bool(&mut self) -> bool {
        let value = self
            .data
            .get(self.read_pos)
            .expect("failed reading 1 byte for bool");
        self.read_pos += 1;
        *value == 1
    }

    /// Reads an unsigned byte from the buffer, increasing the reading position by 1.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(1);
    /// buf.data[0] = 231;
    ///
    /// assert_eq!(buf.read_u8(), 231);
    /// assert_eq!(buf.read_pos, 1);
    /// ```
    pub fn read_u8(&mut self) -> u8 {
        let value = self
            .data
            .get(self.read_pos)
            .expect("failed reading 1 byte for u8");
        self.read_pos += 1;
        *value
    }

    /// Reads a signed byte from the buffer, increasing the reading position by 1.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(1);
    /// buf.data[0] = 6;
    ///
    /// assert_eq!(buf.read_i8(), 6);
    /// assert_eq!(buf.read_pos, 1);
    ///
    /// ```
    pub fn read_i8(&mut self) -> i8 {
        self.read_u8() as i8
    }

    /// Reads an unsigned short from the buffer, increasing the reading position by 2.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(2);
    /// buf.data[0] = 66;
    /// buf.data[1] = 89;
    ///
    /// assert_eq!(buf.read_u16(), 16985);
    /// assert_eq!(buf.read_pos, 2);
    ///
    /// ```
    pub fn read_u16(&mut self) -> u16 {
        let bytes = self
            .data
            .get(self.read_pos..self.read_pos + 2)
            .expect("failed reading 2 bytes for u16");
        self.read_pos += 2;
        BigEndian::read_u16(bytes)
    }

    /// Reads a signed short from the buffer, increasing the reading position by 2.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(2);
    /// buf.data[0] = 255;
    /// buf.data[1] = 98;
    ///
    /// assert_eq!(buf.read_i16(), -158);
    /// assert_eq!(buf.read_pos, 2);
    ///
    /// ```
    pub fn read_i16(&mut self) -> i16 {
        self.read_u16() as i16
    }

    /// Reads an unsigned short as little endian from the buffer, increasing the reading position by 2.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(2);
    /// buf.data[0] = 89;
    /// buf.data[1] = 66;
    ///
    /// assert_eq!(buf.read_u16_le(), 16985);
    /// assert_eq!(buf.read_pos, 2);
    ///
    /// ```
    pub fn read_u16_le(&mut self) -> u16 {
        let bytes = self
            .data
            .get(self.read_pos..self.read_pos + 2)
            .expect("failed reading 2 bytes for u16");
        self.read_pos += 2;
        LittleEndian::read_u16(bytes)
    }

    /// Reads a signed short as little endian from the buffer, increasing the reading position by 2.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(2);
    /// buf.data[0] = 98;
    /// buf.data[1] = 255;
    ///
    /// assert_eq!(buf.read_i16_le(), -158);
    /// assert_eq!(buf.read_pos, 2);
    ///
    /// ```
    pub fn read_i16_le(&mut self) -> i16 {
        self.read_u16_le() as i16
    }

    /// Reads an unsigned short add from the buffer, increasing the reading position by 2.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(2);
    /// buf.data[0] = 99;
    /// buf.data[1] = 130;
    ///
    /// assert_eq!(buf.read_u16_add(), 25346);
    /// assert_eq!(buf.read_pos, 2);
    ///
    /// ```
    pub fn read_u16_add(&mut self) -> u16 {
        ((self.read_u8() as u16) << 8) | ((self.read_u8().wrapping_sub(128)) as u16)
    }

    /// Reads a signed short add from the buffer, increasing the reading position by 2.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(2);
    /// buf.data[0] = 253;
    /// buf.data[1] = 177;
    ///
    /// assert_eq!(buf.read_i16_add(), -719);
    /// assert_eq!(buf.read_pos, 2);
    ///
    /// ```
    pub fn read_i16_add(&mut self) -> i16 {
        self.read_u16_add() as i16
    }

    /// Reads an unsigned short add as little endian from the buffer, increasing the reading position by 2.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(2);
    /// buf.data[0] = 89;
    /// buf.data[1] = 66;
    ///
    /// assert_eq!(buf.read_u16_add_le(), 17113);
    /// assert_eq!(buf.read_pos, 2);
    ///
    /// ```
    pub fn read_u16_add_le(&mut self) -> u16 {
        ((self.read_u8().wrapping_sub(128)) as u16) | ((self.read_u8() as u16) << 8)
    }

    /// Reads a signed short add as little endia from the buffer, increasing the reading position by 2.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(2);
    /// buf.data[0] = 98;
    /// buf.data[1] = 255;
    ///
    /// assert_eq!(buf.read_i16_add_le(), -30);
    /// assert_eq!(buf.read_pos, 2);
    ///
    /// ```
    pub fn read_i16_add_le(&mut self) -> i16 {
        self.read_u16_add_le() as i16
    }

    /// Reads an unsigned dword from the buffer, increasing the reading position by 4.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(4);
    /// buf.data[0] = 42;
    /// buf.data[1] = 87;
    /// buf.data[2] = 33;
    /// buf.data[3] = 16;
    ///
    /// assert_eq!(buf.read_u32(), 710353168);
    /// assert_eq!(buf.read_pos, 4);
    ///
    /// ```
    pub fn read_u32(&mut self) -> u32 {
        let bytes = self
            .data
            .get(self.read_pos..self.read_pos + 4)
            .expect("failed reading 4 bytes for u32");
        self.read_pos += 4;
        BigEndian::read_u32(bytes)
    }

    /// Reads a signed dword from the buffer, increasing the reading position by 4.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(4);
    /// buf.data[0] = 255;
    /// buf.data[1] = 87;
    /// buf.data[2] = 33;
    /// buf.data[3] = 16;
    ///
    /// assert_eq!(buf.read_i32(), -11067120);
    /// assert_eq!(buf.read_pos, 4);
    ///
    /// ```
    pub fn read_i32(&mut self) -> i32 {
        self.read_u32() as i32
    }

    /// Reads an unsigned dword from the buffer, increasing the reading position by 4.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(4);
    /// buf.data[0] = 16;
    /// buf.data[1] = 33;
    /// buf.data[2] = 87;
    /// buf.data[3] = 42;
    ///
    /// assert_eq!(buf.read_u32_le(), 710353168);
    /// assert_eq!(buf.read_pos, 4);
    ///
    /// ```
    pub fn read_u32_le(&mut self) -> u32 {
        let bytes = self
            .data
            .get(self.read_pos..self.read_pos + 4)
            .expect("failed reading 4 bytes for u32");
        self.read_pos += 4;
        LittleEndian::read_u32(bytes)
    }

    /// Reads a signed dword from the buffer, increasing the reading position by 4.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(4);
    /// buf.data[0] = 16;
    /// buf.data[1] = 33;
    /// buf.data[2] = 87;
    /// buf.data[3] = 250;
    ///
    /// assert_eq!(buf.read_i32_le(), -94953200);
    /// assert_eq!(buf.read_pos, 4);
    ///
    /// ```
    pub fn read_i32_le(&mut self) -> i32 {
        self.read_u32_le() as i32
    }

    /// Reads an unsigned dword as middle endian from the buffer, increasing the reading position by 4.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(4);
    /// buf.data[0] = 1;
    /// buf.data[1] = 5;
    /// buf.data[2] = 9;
    /// buf.data[3] = 49;
    ///
    /// assert_eq!(buf.read_u32_me(), 83964169);
    /// assert_eq!(buf.read_pos, 4);
    ///
    /// ```
    pub fn read_u32_me(&mut self) -> u32 {
        (self.read_u16_le() as u32) << 16 | (self.read_u16_le() as u32)
    }

    /// Reads a signed dword as middle endian from the buffer, increasing the reading position by 4.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(4);
    /// buf.data[0] = 0;
    /// buf.data[1] = 149;
    /// buf.data[2] = 0;
    /// buf.data[3] = 0;
    ///
    /// assert_eq!(buf.read_i32_me(), -1795162112);
    /// assert_eq!(buf.read_pos, 4);
    ///
    /// ```
    pub fn read_i32_me(&mut self) -> i32 {
        self.read_u32_me() as i32
    }

    /// Reads an unsigned dword as inversed middle endian from the buffer, increasing the reading position by 4.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(4);
    /// buf.data[0] = 0;
    /// buf.data[1] = 0;
    /// buf.data[2] = 0;
    /// buf.data[3] = 149;
    ///
    /// assert_eq!(buf.read_u32_ime(), 9764864);
    /// assert_eq!(buf.read_pos, 4);
    ///
    /// ```
    pub fn read_u32_ime(&mut self) -> u32 {
        (self.read_u16() as u32) | ((self.read_u16() as u32) << 16)
    }

    /// Reads a signed dword as inversed middle endian from the buffer, increasing the reading position by 4.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(4);
    /// buf.data[0] = 118;
    /// buf.data[1] = 195;
    /// buf.data[2] = 254;
    /// buf.data[3] = 193;
    ///
    /// assert_eq!(buf.read_i32_ime(), -20875581);
    /// assert_eq!(buf.read_pos, 4);
    ///
    /// ```
    pub fn read_i32_ime(&mut self) -> i32 {
        self.read_u32_ime() as i32
    }

    /// Reads an unsigned qword from the buffer, increasing the reading position by 8.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(8);
    /// buf.data[0] = 31;
    /// buf.data[1] = 84;
    /// buf.data[2] = 11;
    /// buf.data[3] = 99;
    /// buf.data[4] = 45;
    /// buf.data[5] = 12;
    /// buf.data[6] = 94;
    /// buf.data[7] = 36;
    ///
    /// assert_eq!(buf.read_u64(), 2257441833804914212);
    /// assert_eq!(buf.read_pos, 8);
    ///
    /// ```
    pub fn read_u64(&mut self) -> u64 {
        let bytes = self
            .data
            .get(self.read_pos..self.read_pos + 8)
            .expect("failed reading 8 bytes for u64");
        self.read_pos += 8;
        BigEndian::read_u64(bytes)
    }

    /// Reads a signed qword from the buffer, increasing the reading position by 8.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(8);
    /// buf.data[0] = 255;
    /// buf.data[1] = 84;
    /// buf.data[2] = 11;
    /// buf.data[3] = 99;
    /// buf.data[4] = 45;
    /// buf.data[5] = 12;
    /// buf.data[6] = 94;
    /// buf.data[7] = 36;
    ///
    /// assert_eq!(buf.read_i64(), -48401175408779740);
    /// assert_eq!(buf.read_pos, 8);
    ///
    /// ```
    pub fn read_i64(&mut self) -> i64 {
        self.read_u64() as i64
    }

    /// Reads a string from the buffer, increasing the reading position by the length of the string.
    ///
    /// # Examples
    ///
    /// ```
    /// use osrs_buffer::ByteBuffer;
    ///
    /// let mut buf = ByteBuffer::new(8);
    /// buf.data[0] = 109;
    /// buf.data[1] = 121;
    /// buf.data[2] = 32;
    /// buf.data[3] = 116;
    /// buf.data[4] = 101;
    /// buf.data[5] = 115;
    /// buf.data[6] = 116;
    /// buf.data[7] = 0;
    ///
    /// assert_eq!(buf.read_string(), "my test");
    /// assert_eq!(buf.read_pos, 7);
    ///
    /// ```
    pub fn read_string(&mut self) -> String {
        let mut str = Vec::new();

        while let Some(x) = self.data.get(self.read_pos) {
            if *x != 0 {
                str.push(*x);
                self.read_pos += 1;
            } else {
                break;
            }
        }

        let s = match std::str::from_utf8(&str) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        s.to_owned()
    }
}

#[cfg(test)]
mod tests {
    //use super::*;

    // Tests that do not fit in documentation should be placed here.

    /*
    #[test]
    fn test_write_u8() {
        let mut buf = ByteBuffer::new(1);
        buf.write_u8(155);
        assert_eq!(buf.data[0], 155);
        assert_eq!(buf.write_pos, 1);
    }

    #[test]
    fn test_read_u8() {
        let mut buf = ByteBuffer::new(1);
        buf.data[0] = 231;
        assert_eq!(buf.read_u8(), 231);
        assert_eq!(buf.read_pos, 1);
    }

    #[test]
    fn test_write_i8() {
        let mut buf = ByteBuffer::new(1);
        buf.write_i8(101);
        assert_eq!(buf.data[0], 101);
        assert_eq!(buf.write_pos, 1);
    }

    #[test]
    fn test_read_i8() {
        let mut buf = ByteBuffer::new(1);
        buf.data[0] = 35;
        assert_eq!(buf.read_i8(), 35);
        assert_eq!(buf.read_pos, 1);
    }

    #[test]
    fn test_write_u16() {
        let mut buf = ByteBuffer::new(2);
        buf.write_u16(18953);
        assert_eq!(buf.data[0], 74);
        assert_eq!(buf.data[1], 9);
        assert_eq!(buf.write_pos, 2);
    }
    */
}
