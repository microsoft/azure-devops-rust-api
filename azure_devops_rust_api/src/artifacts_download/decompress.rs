// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! Decompression for Azure DevOps blob store chunks.
//!
//! Azure DevOps blob store uses an LZ77 variant for content chunks.
//! This module implements the decompressor.

use azure_core::error::{Error, ErrorKind, Result};

/// Decompress a chunk compressed with the LZ77 encoding used by
/// Azure DevOps blob store.
///
/// The format uses:
/// - 32-bit flag indicators with a sentinel bit for tracking consumption
/// - 2-bit literal length encoding (1/2/3/4 byte batches)
/// - 16-bit match references: 13-bit offset + 3-bit length
/// - Nibble-based extended length encoding
pub fn decompress_chunk(compressed: &[u8]) -> Result<Vec<u8>> {
    if compressed.len() < 5 {
        return Err(Error::with_message(
            ErrorKind::DataConversion,
            format!("Compressed data too small: {} bytes", compressed.len()),
        ));
    }

    let mut output = Vec::with_capacity(compressed.len() * 4);
    let mut ci = 0usize;
    let mut indicator: i32;
    let mut nibble_pos: Option<(usize, bool)> = None;

    // When true, the current indicator MSB is a literal-encoding bit (not a
    // decision bit). This happens after reading a fresh indicator whose raw
    // MSB was 0 (literal): that bit is consumed by the explicit check, and
    // after *2+1 the MSB is the first encoding bit.
    let mut fresh_literal;

    let raw = i32::from_le_bytes(compressed[ci..ci + 4].try_into().unwrap());
    ci += 4;
    indicator = raw.wrapping_mul(2).wrapping_add(1);
    fresh_literal = raw >= 0;

    // If the first decision is match, process it before the main loop.
    if raw < 0 {
        if ci + 1 >= compressed.len() {
            return Ok(output);
        }
        process_match(compressed, &mut ci, &mut output, &mut nibble_pos)?;
    }

    loop {
        if ci >= compressed.len() {
            break;
        }

        if fresh_literal {
            fresh_literal = false;
        } else if indicator >= 0 {
            indicator = indicator.wrapping_mul(2);
        } else {
            indicator = indicator.wrapping_mul(2);

            if indicator == 0 {
                if ci + 3 >= compressed.len() {
                    break;
                }
                let raw = i32::from_le_bytes(compressed[ci..ci + 4].try_into().unwrap());
                ci += 4;
                indicator = raw.wrapping_mul(2).wrapping_add(1);
                if raw >= 0 {
                    fresh_literal = true;
                    continue;
                }
            }

            if ci + 1 >= compressed.len() {
                break;
            }
            process_match(compressed, &mut ci, &mut output, &mut nibble_pos)?;
            continue;
        }

        // Literal encoding: bits encode length as 1/2/3/4 bytes per batch
        loop {
            if indicator < 0 {
                if ci >= compressed.len() {
                    return Ok(output);
                }
                output.push(compressed[ci]);
                ci += 1;
                break;
            }
            indicator = indicator.wrapping_mul(2);
            if indicator < 0 {
                if ci + 1 >= compressed.len() {
                    return Ok(output);
                }
                output.extend_from_slice(&compressed[ci..ci + 2]);
                ci += 2;
                break;
            }
            indicator = indicator.wrapping_mul(2);
            if indicator < 0 {
                if ci + 2 >= compressed.len() {
                    return Ok(output);
                }
                output.extend_from_slice(&compressed[ci..ci + 3]);
                ci += 3;
                break;
            }
            indicator = indicator.wrapping_mul(2);
            if ci + 3 >= compressed.len() {
                return Ok(output);
            }
            output.extend_from_slice(&compressed[ci..ci + 4]);
            ci += 4;
            if indicator < 0 {
                break;
            }
            indicator = indicator.wrapping_mul(2);
        }

        // Post-literal shift: consume the "1" bit that ended the literal group
        indicator = indicator.wrapping_mul(2);

        if indicator == 0 {
            if ci + 3 >= compressed.len() {
                break;
            }
            let raw = i32::from_le_bytes(compressed[ci..ci + 4].try_into().unwrap());
            ci += 4;
            indicator = raw.wrapping_mul(2).wrapping_add(1);
            if raw >= 0 {
                fresh_literal = true;
                continue;
            }
        }

        // Match always follows a literal group
        if ci + 1 >= compressed.len() {
            break;
        }
        process_match(compressed, &mut ci, &mut output, &mut nibble_pos)?;
    }

    Ok(output)
}

/// Process a single LZ match: read the 16-bit match descriptor and optional
/// extended length, then copy `match_len` bytes from the output history.
fn process_match(
    compressed: &[u8],
    ci: &mut usize,
    output: &mut Vec<u8>,
    nibble_pos: &mut Option<(usize, bool)>,
) -> Result<()> {
    let v = u16::from_le_bytes(compressed[*ci..*ci + 2].try_into().unwrap());
    *ci += 2;

    let mut match_len = (v & 7) as usize;
    let offset = ((v >> 3) as usize) + 1;

    if match_len == 7 {
        let nibble_val = if let Some((nib_idx, _)) = nibble_pos.take() {
            (compressed[nib_idx] >> 4) as usize
        } else {
            if *ci >= compressed.len() {
                return Err(Error::with_message(
                    ErrorKind::DataConversion,
                    "Unexpected end of compressed data in nibble read",
                ));
            }
            let nib_idx = *ci;
            *ci += 1;
            *nibble_pos = Some((nib_idx, true));
            (compressed[nib_idx] & 0x0F) as usize
        };

        match_len = nibble_val;
        if match_len == 15 {
            if *ci >= compressed.len() {
                return Err(Error::with_message(
                    ErrorKind::DataConversion,
                    "Unexpected end of compressed data in length extension",
                ));
            }
            match_len = compressed[*ci] as usize;
            *ci += 1;
            if match_len == 255 {
                if *ci + 1 >= compressed.len() {
                    return Err(Error::with_message(
                        ErrorKind::DataConversion,
                        "Unexpected end of compressed data in 16-bit length",
                    ));
                }
                match_len =
                    u16::from_le_bytes(compressed[*ci..*ci + 2].try_into().unwrap()) as usize;
                *ci += 2;
                if match_len == 0 {
                    if *ci + 3 >= compressed.len() {
                        return Err(Error::with_message(
                            ErrorKind::DataConversion,
                            "Unexpected end of compressed data in 32-bit length",
                        ));
                    }
                    match_len =
                        u32::from_le_bytes(compressed[*ci..*ci + 4].try_into().unwrap()) as usize;
                    *ci += 4;
                }
                if match_len < 22 {
                    return Err(Error::with_message(
                        ErrorKind::DataConversion,
                        format!("Invalid extended match length: {}", match_len),
                    ));
                }
                match_len -= 22;
            }
            match_len += 15;
        }
        match_len += 7;
    }
    match_len += 3;

    if offset > output.len() {
        return Err(Error::with_message(
            ErrorKind::DataConversion,
            format!(
                "Match offset {} exceeds output size {} at compressed pos {}",
                offset,
                output.len(),
                ci
            ),
        ));
    }
    let src_start = output.len() - offset;
    for i in 0..match_len {
        let byte = output[src_start + (i % offset)];
        output.push(byte);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_too_small_input() {
        assert!(decompress_chunk(&[]).is_err());
        assert!(decompress_chunk(&[0; 4]).is_err());
    }

    #[test]
    fn test_single_literal() {
        // Indicator 0x40000000: bit31=0 (literal), bit30=1 (1-byte literal)
        let compressed: &[u8] = &[0x00, 0x00, 0x00, 0x40, 0x48]; // literal 'H'
        let result = decompress_chunk(compressed).unwrap();
        assert_eq!(result, b"H");
    }

    #[test]
    fn test_literal_then_match() {
        // Indicator 0x40000000 + literal 'A' + match(offset=1, len=3) -> "AAAA"
        let compressed: &[u8] = &[0x00, 0x00, 0x00, 0x40, 0x41, 0x00, 0x00];
        let result = decompress_chunk(compressed).unwrap();
        assert_eq!(result, b"AAAA");
    }

    #[test]
    fn test_minimum_valid_input() {
        // 5 bytes is the minimum accepted size
        let compressed: &[u8] = &[0x00, 0x00, 0x00, 0x40, 0x58]; // literal 'X'
        let result = decompress_chunk(compressed).unwrap();
        assert_eq!(result, b"X");
    }

    #[test]
    fn test_match_offset_out_of_bounds() {
        // First decision is match (bit31=1), match with offset > 0 when output is empty
        // raw = 0x80000000, match v=0x0008 -> offset=2, but output is empty -> error
        let compressed: &[u8] = &[0x00, 0x00, 0x00, 0x80, 0x08, 0x00];
        assert!(decompress_chunk(compressed).is_err());
    }
}
