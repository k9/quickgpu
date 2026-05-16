pub fn graph(texels: &mut [u8], t: f32, f: fn(f32) -> f32, size: usize) {
    for x_base in 0..size {
        let fsize = size as f32;

        let offsets = 8;
        for offset in -offsets..=offsets {
            let offset = (offset as f32) / ((offsets * 2) as f32);
            let x = x_base as f32 + offset;
            let texture_t = (x as f32) / (fsize - 1.0);
            let y = ((f(texture_t) * fsize) as usize).min(size - 1);

            texels[y * size + x_base] = if (t - texture_t).abs() < 0.005 {
                255
            } else {
                64
            };
        }
    }
}

pub fn linear(t: f32) -> f32 {
    let value = t * 2.0;
    if value < 1.0 { value } else { 2.0 - value }
}

pub fn cubic(t: f32) -> f32 {
    let t = linear(t);
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
}
