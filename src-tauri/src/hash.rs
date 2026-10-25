use ffmpeg_next as ffmpeg;
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

// Video hashing functions — added by Plan 03-02

/// Hash triplet for a single frame: [aHash, dHash, pHash].
pub type FrameHash = [[u8; 8]; 3];

/// Compute hashes for a file on disk, dispatching to image or video hashing.
///
/// Uses the file extension to determine the correct hashing path:
/// - Images: calls compute_hashes() directly with raw bytes
/// - Videos: extracts frames via FFmpeg, uses median frame's hash as canonical
///
/// Requires: ffmpeg-next installed (Plan 03-03).
pub fn compute_hashes_for_file(
    path: &str,
    file_type: crate::scan::FileType,
) -> Result<crate::scan::FileHash, String> {
    let bytes = std::fs::read(path).map_err(|e| {
        format!("Failed to read file {}: {}", path, e)
    })?;
    let size = bytes.len() as u64;

    match file_type {
        crate::scan::FileType::Video => {
            let frames = compute_hashes_for_video(path)?;
            let canonical = video_canonical_hash(&frames);
            Ok(crate::scan::FileHash {
                path: path.to_string(),
                ahash: canonical[0],
                dhash: canonical[1],
                phash: canonical[2],
                size,
            })
        }
        crate::scan::FileType::Image => {
            let [ahash, dhash, phash] = compute_hashes(&bytes)?;
            Ok(crate::scan::FileHash {
                path: path.to_string(),
                ahash,
                dhash,
                phash,
                size,
            })
        }
    }
}

/// Maximum number of frames to extract from a video.
const MAX_VIDEO_FRAMES: usize = 20;

/// Scale a single FFmpeg video frame to 32x32 grayscale.
fn scale_frame_to_32x32(frame: &ffmpeg::frame::Video) -> Result<DynamicImage, String> {
    let (src_w, src_h) = (frame.width(), frame.height());
    let src_format = frame.format();

    let mut scaler = ffmpeg::software::scaler(
        src_format,
        ffmpeg::software::scaling::Flags::BILINEAR,
        (src_w, src_h),
        (32, 32),
    ).map_err(|e| format!("Failed to create scaler: {}", e))?;

    let mut scaled = ffmpeg::frame::Video::new(
        ffmpeg::format::Pixel::GRAY8,
        32,
        32,
    );
    scaler.run(frame, &mut scaled).map_err(|e| format!("Failed to scale frame: {}", e))?;

    let bytes: Vec<u8> = scaled.data(0).to_vec();
    let buf = image::ImageBuffer::from_raw(32, 32, bytes).ok_or(
        "Invalid frame dimensions".to_string(),
    )?;
    Ok(DynamicImage::ImageLuma8(buf))
}

/// Compute perceptual hashes for each frame of a video file.
///
/// Extracts frames at 5-second intervals using ffmpeg-next, converts
/// each to grayscale, and computes triple-perceptual-hash (same as images).
///
/// Falls back to first frame only if no frames can be extracted (D-07).
///
/// Requires: ffmpeg-next installed (Plan 03-03).
pub fn compute_hashes_for_video(path: &str) -> Result<Vec<FrameHash>, String> {
    ffmpeg::init().map_err(|e| format!("FFmpeg init failed: {}", e))?;

    let mut input = ffmpeg::format::input(path).map_err(|e| {
        format!("Failed to open video {}: {}", path, e)
    })?;

    // Find the best video stream
    let stream = input.streams().best(ffmpeg::media::Type::Video).ok_or(
        "No video stream found in {}".to_string(),
    )?;

    let video_stream_id = stream.id();

    // Open decoder from stream codec parameters (FFmpeg 7+ compatibility)
    let ctx = ffmpeg::codec::Context::from_parameters(stream.parameters()).map_err(|e| {
        format!("Failed to create decoder context: {}", e)
    })?;
    let mut decoder = ctx.decoder().video().map_err(|e| {
        format!("Failed to open video decoder: {}", e)
    })?;

    // Get time base for frame interval calculation
    let time_base = stream.time_base();
    let rate = stream.rate();

    // Compute frame interval: 5 seconds in time_base units
    let interval = if time_base.denominator() != 0 && time_base.numerator() != 0 {
        5_i64 * time_base.denominator() as i64 / time_base.numerator() as i64
    } else {
        // Fallback: use 5 seconds * stream rate
        (5.0 * rate.numerator() as f64 / rate.denominator() as f64) as i64
    };

    let mut frames: Vec<FrameHash> = Vec::new();
    let mut next_frame_pts: i64 = 0;

    let mut pkt_iter = input.packets();
    while let Some((stream_ref, packet)) = pkt_iter.next() {
        if stream_ref.id() != video_stream_id {
            continue;
        }

        decoder.send_packet(&packet).map_err(|e| {
            format!("Failed to send packet to decoder: {}", e)
        })?;

        loop {
            let mut frame = ffmpeg::frame::Video::empty();
            match decoder.receive_frame(&mut frame) {
                Ok(_) => {
                    let pts = frame.pts().unwrap_or(0);
                    if pts >= next_frame_pts && frames.len() < MAX_VIDEO_FRAMES {
                        // Scale frame to 32x32 grayscale
                        if let Ok(img) = scale_frame_to_32x32(&frame) {
                            let a = compute_ahash(&img);
                            let d = compute_dhash(&img);
                            let p = compute_phash(&img);
                            frames.push([a, d, p]);
                        }
                        next_frame_pts += interval;
                    }

                    if frames.len() >= MAX_VIDEO_FRAMES {
                        return Ok(frames);
                    }
                }
                Err(ffmpeg::Error::Eof) => break,
                Err(_) => break,
            }
        }
    }

    // Fallback (D-07): if no frames extracted, try first frame only
    if frames.is_empty() {
        log::warn!("No frames extracted from {}, using first frame fallback", path);
        if let Ok(first_frame) = extract_first_frame(path) {
            frames.push(first_frame);
        }
    }

    Ok(frames)
}

/// Extract the first frame of a video and return its hash triplet.
fn extract_first_frame(path: &str) -> Result<FrameHash, String> {
    let input = ffmpeg::format::input(path).map_err(|e| {
        format!("Failed to open video {}: {}", path, e)
    })?;

    let stream = input.streams().best(ffmpeg::media::Type::Video).ok_or(
        "No video stream found".to_string(),
    )?;

    // Open decoder from stream codec parameters (FFmpeg 7+ compatibility)
    let ctx = ffmpeg::codec::Context::from_parameters(stream.parameters()).map_err(|e| {
        format!("Failed to create decoder context: {}", e)
    })?;
    let mut decoder = ctx.decoder().video().map_err(|e| {
        format!("Failed to open video decoder: {}", e)
    })?;

    // Flush decoder
    decoder.send_eof().map_err(|e| {
        format!("Failed to flush decoder: {}", e)
    })?;

    let mut frame = ffmpeg::frame::Video::empty();
    decoder.receive_frame(&mut frame).map_err(|e| {
        format!("Failed to receive first frame: {}", e)
    })?;

    let img = scale_frame_to_32x32(&frame)?;

    Ok([compute_ahash(&img), compute_dhash(&img), compute_phash(&img)])
}

/// Compare two videos by their frame hash sets.
///
/// Per D-07: N >= 3 matching frames AND >= 50% of analyzed frames match.
///
/// A frame "matches" if triple_match returns true for the frame triplets.
pub fn videos_match(frames1: &Vec<FrameHash>, frames2: &Vec<FrameHash>) -> bool {
    if frames1.is_empty() || frames2.is_empty() {
        return false;
    }

    let min_len = frames1.len().min(frames2.len());
    let mut matches: usize = 0;

    for i in 0..min_len {
        if triple_match(
            &frames1[i][0], &frames2[i][0],
            &frames1[i][1], &frames2[i][1],
            &frames1[i][2], &frames2[i][2],
        ) {
            matches += 1;
        }
    }

    // N >= 3 matching frames AND >= 50% of analyzed frames match
    matches >= 3 && (matches as f64) >= (min_len as f64) * 0.5
}

/// Compute a canonical hash for a video file (median frame hash).
///
/// Videos are grouped by their median frame's perceptual hash,
/// then refined by frame-set comparison (videos_match).
fn video_canonical_hash(frames: &Vec<FrameHash>) -> FrameHash {
    if frames.is_empty() {
        return [[0u8; 8]; 3];
    }
    frames[frames.len() / 2]
}

