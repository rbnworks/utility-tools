use image::{DynamicImage, ImageBuffer, Luma, Rgba};
use image::imageops::{resize, FilterType};
use std::io::Cursor;

/// Compute three perceptual hashes for image bytes.
/// Returns [aHash, dHash, pHash] each as [u8; 8].
pub fn compute_hashes(image_bytes: &[u8]) -> Result<[[u8; 8]; 3], String> {
    let img = load_image(image_bytes)
        .map_err(|e| format!("Failed to decode image: {}", e))?;

    let a = compute_ahash(&img);
    let d = compute_dhash(&img);
    let p = compute_phash(&img);

    Ok([a, d, p])
}

/// Load an image from raw bytes, auto-detecting format.
pub fn load_image(bytes: &[u8]) -> Result<DynamicImage, String> {
    let cursor = Cursor::new(bytes);
    image::ImageReader::new(cursor)
        .with_guessed_format()
        .map_err(|e| format!("Failed to guess image format: {}", e))?
        .decode()
        .map_err(|e| format!("Failed to decode image: {}", e))
}

/// Convert an ImageBuffer<Rgba<u8>> to GrayImage (Luma<u8>).
fn rgba_to_luma(img: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let (w, h) = img.dimensions();
    let mut gray = ImageBuffer::new(w, h);
    for y in 0..h {
        for x in 0..w {
            let pixel = img.get_pixel(x, y);
            let luma = (0.299 * pixel[0] as f32
                + 0.587 * pixel[1] as f32
                + 0.114 * pixel[2] as f32) as u8;
            gray.put_pixel(x, y, Luma([luma]));
        }
    }
    gray
}

/// Compute average hash (aHash) — resize to 8x8 grayscale, compare to mean.
///
/// Standard aHash algorithm:
/// 1. Resize to 8x8, convert to grayscale
/// 2. Compute average pixel value
/// 3. Bit = 1 if pixel >= average, else 0
pub fn compute_ahash(img: &DynamicImage) -> [u8; 8] {
    // Resize to 8x8 (returns ImageBuffer<Rgba<u8>, Vec<u8>>)
    let resized = resize(img, 8, 8, FilterType::Lanczos3);
    // Convert to grayscale
    let gray = rgba_to_luma(&resized);

    // Compute average
    let mut sum: u64 = 0;
    for y in 0..8 {
        for x in 0..8 {
            sum += gray.get_pixel(x, y).0[0] as u64;
        }
    }
    let avg = sum / 64;

    // Generate hash
    let mut hash: u8 = 0;
    let mut bit: u32 = 7;
    for y in 0..8 {
        for x in 0..8 {
            let val = gray.get_pixel(x, y).0[0];
            if val >= avg as u8 {
                hash |= 1 << bit;
            }
            bit -= 1;
        }
    }

    // Pack into 8 bytes (one bit per byte for consistency with dHash/pHash layout)
    let mut result = [0u8; 8];
    for i in 0..8 {
        let bit_idx = (7 - i) as u32;
        if (hash >> bit_idx) & 1 == 1 {
            result[i] = 0xFF;
        }
    }
    result
}

/// Compute differential hash (dHash) — resize to 9x8, compare adjacent pixels.
///
/// Standard dHash algorithm:
/// 1. Resize to 9x8, convert to grayscale
/// 2. For each row, compare left pixel to right pixel
/// 3. Bit = 1 if left >= right, else 0
pub fn compute_dhash(img: &DynamicImage) -> [u8; 8] {
    // Resize to 9x8 (returns ImageBuffer<Rgba<u8>, Vec<u8>>)
    let resized = resize(img, 9, 8, FilterType::Lanczos3);
    // Convert to grayscale
    let gray = rgba_to_luma(&resized);

    let mut hash: u8 = 0;
    let mut bit: u32 = 7;
    for y in 0..8 {
        for x in 0..8 {
            let left = gray.get_pixel(x, y).0[0];
            let right = gray.get_pixel(x + 1, y).0[0];
            if left >= right {
                hash |= 1 << bit;
            }
            bit -= 1;
        }
    }

    // Pack into 8 bytes
    let mut result = [0u8; 8];
    for i in 0..8 {
        let bit_idx = (7 - i) as u32;
        if (hash >> bit_idx) & 1 == 1 {
            result[i] = 0xFF;
        }
    }
    result
}

/// Compute perceptual hash (pHash) using 2D DCT on 32x32 grayscale image.
///
/// Follows HASH-03: uses actual 2D DCT-II on low-frequency coefficients,
/// NOT simple pixel sampling.
pub fn compute_phash(img: &DynamicImage) -> [u8; 8] {
    // Resize to 32x32 (returns ImageBuffer<Rgba<u8>, Vec<u8>>)
    let resized = resize(img, 32, 32, FilterType::Lanczos3);
    // Convert to grayscale
    let gray = rgba_to_luma(&resized);

    // Extract 32x32 pixel values normalized to [-1, 1]
    let mut pixels: [[f64; 32]; 32] = [[0.0; 32]; 32];
    for y in 0..32u32 {
        for x in 0..32u32 {
            let val = gray.get_pixel(x, y).0[0] as f64;
            let y_usize = y as usize;
            let x_usize = x as usize;
            pixels[y_usize][x_usize] = (2.0 * val / 255.0) - 1.0;
        }
    }

    // Compute 2D DCT-II
    let mut dct: [[f64; 32]; 32] = [[0.0; 32]; 32];
    for v in 0..32u32 {
        for u in 0..32u32 {
            let mut sum = 0.0;
            for m in 0..32u32 {
                for n in 0..32u32 {
                    let m_f = m as f64;
                    let n_f = n as f64;
                    let u_f = u as f64;
                    let v_f = v as f64;
                    let m_usize = m as usize;
                    let n_usize = n as usize;
                    sum += pixels[m_usize][n_usize]
                        * (std::f64::consts::PI * (2.0 * m_f + 1.0) * u_f / 64.0)
                            .cos()
                        * (std::f64::consts::PI * (2.0 * n_f + 1.0) * v_f / 64.0)
                            .cos();
                }
            }
            let cu = if u == 0 { 1.0 / (2.0_f64.sqrt()) } else { 1.0 };
            let cv = if v == 0 { 1.0 / (2.0_f64.sqrt()) } else { 1.0 };
            let v_usize = v as usize;
            let u_usize = u as usize;
            dct[v_usize][u_usize] = cu * cv * 0.25 * sum;
        }
    }

    // Extract 63 low-frequency coefficients (exclude DC at [0][0])
    let mut coeffs: Vec<f64> = Vec::new();
    for v in 0..8u32 {
        for u in 0..8u32 {
            if u == 0 && v == 0 {
                continue;
            }
            if u < 8 && v < 8 {
                let v_usize = v as usize;
                let u_usize = u as usize;
                coeffs.push(dct[v_usize][u_usize]);
            }
        }
    }

    // Compute median
    coeffs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let median = if coeffs.len() % 2 == 0 {
        (coeffs[coeffs.len() / 2 - 1] + coeffs[coeffs.len() / 2]) / 2.0
    } else {
        coeffs[coeffs.len() / 2]
    };

    // Generate 64-bit hash from 8x8 block including DC
    let mut hash: u64 = 0;

    // DC component at [0][0] — bit 64 (MSB)
    if dct[0][0] > 0.0 {
        hash |= 1u64 << 63;
    }

    // Remaining 63 bits from low-frequency coefficients (excluding DC)
    let mut bit_pos: u32 = 62;
    for v in 0..8u32 {
        for u in 0..8u32 {
            if u == 0 && v == 0 {
                continue;
            }
            let v_usize = v as usize;
            let u_usize = u as usize;
            if dct[v_usize][u_usize] > median {
                hash |= 1u64 << bit_pos;
            }
            bit_pos -= 1;
        }
    }

    // Convert u64 to [u8; 8] (big-endian)
    let mut result = [0u8; 8];
    for i in 0..8 {
        result[i] = ((hash >> (56 - 8 * i)) & 0xFF) as u8;
    }

    result
}

/// Compute Hamming distance between two [u8; 8] hashes.
pub fn hamming_distance(a: &[u8; 8], b: &[u8; 8]) -> u32 {
    let mut distance: u32 = 0;
    for i in 0..8 {
        distance += (a[i] ^ b[i]).count_ones();
    }
    distance
}

/// Check if three hashes match using triple-agreement.
/// All three algorithms (aHash, dHash, pHash) must have Hamming distance <= 5.
///
/// Implements HASH-05 / D-06: triple-agreement with <= 5 threshold.
pub fn triple_match(
    ahash1: &[u8; 8],
    ahash2: &[u8; 8],
    dhash1: &[u8; 8],
    dhash2: &[u8; 8],
    phash1: &[u8; 8],
    phash2: &[u8; 8],
) -> bool {
    hamming_distance(ahash1, ahash2) <= 5
        && hamming_distance(dhash1, dhash2) <= 5
        && hamming_distance(phash1, phash2) <= 5
}
