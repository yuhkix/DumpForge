use std::io::Cursor;
use lzma_rs::decompress::raw::{LzmaDecoder, LzmaParams, LzmaProperties};

const LZMA_PROP_MAX: u32 = 9 * 5 * 5;
const LZMA_DIC_MIN: u32 = 1 << 12;

const PROPS_XK: u8 = 0x9d ^ 0xf3;
const PROPS_AK: u8 = 0x65 ^ 0xa2;
const USIZE_XK: u32 = 0x218d5a0a ^ 0xab8832cb;
const USIZE_AK: u32 = 0x5e98c630 ^ 0x66e44294;
const CSIZE_XK: u32 = 0x6dfc36ff ^ 0xfc4c53bf;
const CSIZE_AK: u32 = 0xc68dd929 ^ 0x78bdb6e9;

const GOOD_PROPS: u32 = 0xcd386d0f ^ 0x996f3a58;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Lzma Error: {0}")]
    Lzma(#[from] lzma_rs::error::Error),
    #[error("TryFromSlice Error: {0}")]
    TryFromSlice(#[from] std::array::TryFromSliceError),
    #[error("LZMA header invalid properties: {0} must be < 225")]
    PropsTooBig(u32),
}

#[repr(packed)]
#[derive(Debug)]
pub struct TlzmaHeader {
    pub props: [u8; 5],
    pub _1: [u8; 3],
    pub uncompressed_size: u32,
    pub compressed_size: u32,
}

impl TlzmaHeader {
    pub fn tlzma_test_header(data: &[u8]) -> Result<bool, Error> {
        let tlzma: TlzmaHeader = unsafe { std::ptr::read(data.as_ptr() as *const _) };
        let good_props = u32::from_le_bytes(tlzma.props[1..].try_into()?);
        Ok(GOOD_PROPS == good_props)
    }

    pub fn parse_header(data: &[u8]) -> Self {
        let mut tlzma: TlzmaHeader = unsafe { std::ptr::read(data.as_ptr() as *const _) };
        for i in 0..tlzma.props.len() {
            tlzma.props[i] = (tlzma.props[i] ^ PROPS_XK).wrapping_add(PROPS_AK);
        }
        tlzma.uncompressed_size = (tlzma.uncompressed_size ^ USIZE_XK).wrapping_add(USIZE_AK);
        tlzma.compressed_size = (tlzma.compressed_size ^ CSIZE_XK).wrapping_add(CSIZE_AK);
        tlzma
    }

    pub fn get_next_chunk_offset(&self, offset: usize, data: &[u8]) -> Result<usize, Error> {
        let tmp = offset + size_of::<Self>() + self.compressed_size as usize;
        let input = &data[tmp..tmp + 16];
        match Self::tlzma_test_header(input)? {
            true => Ok(tmp),
            false => Ok(offset + self.uncompressed_size as usize)
        }
    }

    pub fn lzma_decode(&self, input: &[u8], offset: usize) -> Result<(Vec<u8>, u32), Error> {
        let params = self.get_params_from_props()?;
        let mut decoder = LzmaDecoder::new(params, None)?;
        let start = offset + size_of::<Self>();
        let end = start + self.compressed_size as usize;
        let mut data = Cursor::new(&input[start..end]);
        let mut output = Vec::with_capacity(self.uncompressed_size as usize);
        decoder.decompress(&mut data, &mut output)?;

        Ok((output, self.uncompressed_size))
    }

    fn get_params_from_props(&self) -> Result<LzmaParams, Error> {
        let mut pb = self.props[0] as u32;
        if pb >= LZMA_PROP_MAX {
            return Err(Error::PropsTooBig(pb));
        }

        let mut dic = u32::from_le_bytes(self.props[1..].try_into()?);
        if dic < LZMA_DIC_MIN {
            dic = LZMA_DIC_MIN;
        }

        let lc: u32 = pb % 9;
        pb /= 9;
        let lp: u32 = pb % 5;
        pb /= 5;

        Ok(
            LzmaParams::new(
                LzmaProperties { lc, lp, pb },
                dic,
                Some(self.uncompressed_size as u64),
            )
        )
    }
}