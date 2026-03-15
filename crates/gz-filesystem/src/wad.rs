/// WAD file reader.
///
/// WAD format:
///   4-byte magic  ("IWAD" or "PWAD")
///   4-byte lump count
///   4-byte directory offset
///   … lump data …
///   directory: count × (offset u32, size u32, name [u8;8])

use crate::lump::LumpInfo;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WadError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid WAD magic: expected IWAD or PWAD")]
    BadMagic,
}

pub struct WadReader {
    pub lumps: Vec<LumpInfo>,
    pub is_iwad: bool,
}

impl WadReader {
    /// Parse the WAD directory from raw bytes.
    pub fn from_bytes(data: &[u8], archive_index: u32) -> Result<Self, WadError> {
        if data.len() < 12 {
            return Err(WadError::BadMagic);
        }
        let magic = &data[0..4];
        let is_iwad = match magic {
            b"IWAD" => true,
            b"PWAD" => false,
            _ => return Err(WadError::BadMagic),
        };

        let num_lumps = u32::from_le_bytes(data[4..8].try_into().unwrap()) as usize;
        let dir_offset = u32::from_le_bytes(data[8..12].try_into().unwrap()) as usize;

        let mut lumps = Vec::with_capacity(num_lumps);
        for i in 0..num_lumps {
            let entry = &data[dir_offset + i * 16..dir_offset + (i + 1) * 16];
            let offset = u32::from_le_bytes(entry[0..4].try_into().unwrap()) as u64;
            let size   = u32::from_le_bytes(entry[4..8].try_into().unwrap()) as u64;
            let raw_name = &entry[8..16];
            // WAD names are null-padded 8-byte ASCII; strip the null padding.
            let end = raw_name.iter().position(|&b| b == 0).unwrap_or(8);
            let name = String::from_utf8_lossy(&raw_name[..end]).to_uppercase();

            lumps.push(LumpInfo { name, archive_index, offset, size });
        }

        Ok(WadReader { lumps, is_iwad })
    }
}
