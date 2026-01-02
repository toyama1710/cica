/// Color space structures and conversions.
///
/// All structures use `#[repr(C)]` for fixed memory layout suitable for wgpu.

const COLOR_EPSILON: f32 = 1.0 / 1024.0;

///  CIE XYZ color space pixel.
///
/// - X: X value
/// - Y: Y value
/// - Z: Z value
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Xyz {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

const D50_XYZ: Xyz = Xyz {
    x: 0.964212,
    y: 1.0,
    z: 0.825188,
};
#[allow(dead_code)]
const D65_XYZ: Xyz = Xyz {
    x: 0.95047,
    y: 1.0,
    z: 1.08883,
};

// Bradford chromatic adaptation matrices
// see also: http://www.brucelindbloom.com/index.html?Eqn_ChromAdapt.html

/// Bradford matrix for D50 → D65 chromatic adaptation
#[rustfmt::skip]
const BRADFORD_D50_TO_D65: [[f32; 3]; 3] = [
    [ 0.9555766, -0.0230393,  0.0631636],
    [-0.0282895,  1.0099416,  0.0210077],
    [ 0.0122982, -0.0204830,  1.3299098],
];

/// Bradford matrix for D65 → D50 chromatic adaptation
#[rustfmt::skip]
const BRADFORD_D65_TO_D50: [[f32; 3]; 3] = [
    [ 1.0478112,  0.0228866, -0.0501270],
    [ 0.0295424,  0.9904844, -0.0170491],
    [-0.0092345,  0.0150436,  0.7521316],
];

/// CIE Lab color space pixel.
///
/// - L: Lightness [0.0, 100.0]
/// - a: Green-red axis (approximately [-128.0 ~ 127.0])
/// - b: Blue-yellow axis (approximately [-128.0, 127.0])
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Lab {
    pub l: f32,
    pub a: f32,
    pub b: f32,
}

/// sRGB color space pixel.
///
/// Each channel is a gamma-corrected value in the range 0.0 ~ 1.0.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Srgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

/// HSV color space pixel.
///
/// - h: Hue [0.0, 360.0)
/// - s: Saturation [0.0, 1.0]
/// - v: Value [0.0, 1.0]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hsv {
    pub h: f32,
    pub s: f32,
    pub v: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum ColorSpace {
    Xyz(Xyz),
    Lab(Lab),
    Srgb(Srgb),
    Hsv(Hsv),
}

impl Xyz {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl Lab {
    pub fn new(l: f32, a: f32, b: f32) -> Self {
        // clamp only l value. since a & b have no limit theoretically.
        Self {
            l: l.clamp(0.0, 100.0),
            a: a,
            b: b,
        }
    }
}

impl Srgb {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self {
            r: r.clamp(0.0, 1.0),
            g: g.clamp(0.0, 1.0),
            b: b.clamp(0.0, 1.0),
        }
    }
}

impl Hsv {
    pub fn new(h: f32, s: f32, v: f32) -> Self {
        Self {
            h: h % 360.0,
            s: s.clamp(0.0, 1.0),
            v: v.clamp(0.0, 1.0),
        }
    }
}

// Lab - XYZ conversion constants
const LAB_EPSILON: f32 = (6.0 * 6.0 * 6.0) / (29.0 * 29.0 * 29.0); // = (6/29)^3
const LAB_FACTOR: f32 = 841.0 / 108.0 * 116.0;

fn lab_f(t: f32) -> f32 {
    if t > LAB_EPSILON {
        t.powf(1.0 / 3.0)
    } else {
        LAB_FACTOR * t + 16.0
    }
}

fn lab_f_inv(t: f32) -> f32 {
    let t3 = t.powi(3);
    if t3 > LAB_EPSILON {
        t3
    } else {
        (t - 16.0) / LAB_FACTOR
    }
}

fn xyz_to_lab(xyz: Xyz) -> (f32, f32, f32) {
    let x = xyz.x / D50_XYZ.x;
    let y = xyz.y / D50_XYZ.y;
    let z = xyz.z / D50_XYZ.z;

    let fx = lab_f(x);
    let fy = lab_f(y);
    let fz = lab_f(z);

    let l = 116.0 * fy - 16.0;
    let a = 500.0 * (fx - fy);
    let b = 200.0 * (fy - fz);
    (l, a, b)
}

fn lab_to_xyz(lab: Lab) -> (f32, f32, f32) {
    let fy = (lab.l + 16.0) / 116.0;
    let fx = lab.a / 500.0 + fy;
    let fz = fy - lab.b / 200.0;

    let x = lab_f_inv(fx) * D50_XYZ.x;
    let y = lab_f_inv(fy) * D50_XYZ.y;
    let z = lab_f_inv(fz) * D50_XYZ.z;
    (x, y, z)
}

/// Apply a 3x3 matrix to XYZ values
fn apply_matrix(m: &[[f32; 3]; 3], x: f32, y: f32, z: f32) -> (f32, f32, f32) {
    let r = m[0][0] * x + m[0][1] * y + m[0][2] * z;
    let g = m[1][0] * x + m[1][1] * y + m[1][2] * z;
    let b = m[2][0] * x + m[2][1] * y + m[2][2] * z;
    (r, g, b)
}

/// Convert D50 XYZ to D65 XYZ using Bradford chromatic adaptation
fn xyz_d50_to_d65(x: f32, y: f32, z: f32) -> (f32, f32, f32) {
    apply_matrix(&BRADFORD_D50_TO_D65, x, y, z)
}

/// Convert D65 XYZ to D50 XYZ using Bradford chromatic adaptation
fn xyz_d65_to_d50(x: f32, y: f32, z: f32) -> (f32, f32, f32) {
    apply_matrix(&BRADFORD_D65_TO_D50, x, y, z)
}

// XYZ (D65) to linear sRGB matrix
// see also: http://www.brucelindbloom.com/index.html?Eqn_RGB_XYZ_Matrix.html
#[rustfmt::skip]
const XYZ_D65_TO_LINEAR_SRGB: [[f32; 3]; 3] = [
    [ 3.2404542, -1.5371385, -0.4985314],
    [-0.9692660,  1.8760108,  0.0415560],
    [ 0.0556434, -0.2040259,  1.0572252],
];

// Convert D50 XYZ to linear sRGB (via D65 chromatic adaptation)
fn xyz_to_linear_srgb(x: f32, y: f32, z: f32) -> (f32, f32, f32) {
    // First adapt from D50 to D65
    let (x65, y65, z65) = xyz_d50_to_d65(x, y, z);
    // Then convert D65 XYZ to linear sRGB
    apply_matrix(&XYZ_D65_TO_LINEAR_SRGB, x65, y65, z65)
}

fn linear_to_gamma(linear: f32) -> f32 {
    if linear <= 0.0031308 {
        linear * 12.92
    } else {
        1.055 * linear.powf(1.0 / 2.4) - 0.055
    }
}

fn linear_srgb_to_srgb(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    (linear_to_gamma(r), linear_to_gamma(g), linear_to_gamma(b))
}

fn xyz_to_srgb(x: f32, y: f32, z: f32) -> (f32, f32, f32) {
    let (r, g, b) = xyz_to_linear_srgb(x, y, z);
    let (r, g, b) = linear_srgb_to_srgb(r, g, b);
    (r, g, b)
}

fn srgb_to_hsv(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let s = if max < f32::EPSILON { 0.0 } else { delta / max };
    let v = max;
    let h = if delta < COLOR_EPSILON || s < COLOR_EPSILON {
        0.0
    } else if max == r {
        (60.0 * ((g - b) / delta) + 360.0) % 360.0
    } else if max == g {
        (60.0 * ((b - r) / delta) + 120.0 + 360.0) % 360.0
    } else {
        (60.0 * ((r - g) / delta) + 240.0 + 360.0) % 360.0
    };
    (h, s, v)
}

fn gamma_to_linear(gamma: f32) -> f32 {
    if gamma <= 0.040449936 {
        gamma / 12.92
    } else {
        ((gamma + 0.055) / 1.055).powf(2.4)
    }
}

fn srgb_to_linear_srgb(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    (gamma_to_linear(r), gamma_to_linear(g), gamma_to_linear(b))
}

// Linear sRGB to XYZ (D65) matrix
// see also: http://www.brucelindbloom.com/index.html?Eqn_RGB_XYZ_Matrix.html
#[rustfmt::skip]
const LINEAR_SRGB_TO_XYZ_D65: [[f32; 3]; 3] = [
    [0.4124564, 0.3575761, 0.1804375],
    [0.2126729, 0.7151522, 0.0721750],
    [0.0193339, 0.1191920, 0.9503041],
];

// Convert linear sRGB to D50 XYZ (via D65 chromatic adaptation)
fn linear_srgb_to_xyz(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    // First convert linear sRGB to D65 XYZ
    let (x65, y65, z65) = apply_matrix(&LINEAR_SRGB_TO_XYZ_D65, r, g, b);
    // Then adapt from D65 to D50
    xyz_d65_to_d50(x65, y65, z65)
}

fn srgb_to_xyz(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let (r, g, b) = srgb_to_linear_srgb(r, g, b);
    linear_srgb_to_xyz(r, g, b)
}

// see also: https://en.wikipedia.org/wiki/HSL_and_HSV#To_RGB
fn hsv_to_srgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let c = v * s;
    let h_prime = h / 60.0;
    let x = c * (1.0 - ((h_prime % 2.0) - 1.0).abs());
    let m = v - c;

    let (r1, g1, b1) = if h_prime < 1.0 {
        (c, x, 0.0)
    } else if h_prime < 2.0 {
        (x, c, 0.0)
    } else if h_prime < 3.0 {
        (0.0, c, x)
    } else if h_prime < 4.0 {
        (0.0, x, c)
    } else if h_prime < 5.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    (r1 + m, g1 + m, b1 + m)
}

impl From<Xyz> for ColorSpace {
    fn from(xyz: Xyz) -> Self {
        ColorSpace::Xyz(xyz)
    }
}
impl From<Lab> for ColorSpace {
    fn from(lab: Lab) -> Self {
        ColorSpace::Lab(lab)
    }
}
impl From<Srgb> for ColorSpace {
    fn from(srgb: Srgb) -> Self {
        ColorSpace::Srgb(srgb)
    }
}
impl From<Hsv> for ColorSpace {
    fn from(hsv: Hsv) -> Self {
        ColorSpace::Hsv(hsv)
    }
}

impl From<Xyz> for Srgb {
    fn from(xyz: Xyz) -> Self {
        let (r, g, b) = xyz_to_srgb(xyz.x, xyz.y, xyz.z);
        Srgb::new(r, g, b)
    }
}
impl From<Xyz> for Hsv {
    fn from(xyz: Xyz) -> Self {
        let (r, g, b) = xyz_to_srgb(xyz.x, xyz.y, xyz.z);
        let (h, s, v) = srgb_to_hsv(r, g, b);
        Hsv::new(h, s, v)
    }
}
impl From<Xyz> for Lab {
    fn from(xyz: Xyz) -> Self {
        let (l, a, b) = xyz_to_lab(xyz);
        Lab::new(l, a, b)
    }
}

impl From<Srgb> for Xyz {
    fn from(srgb: Srgb) -> Self {
        let (x, y, z) = srgb_to_xyz(srgb.r, srgb.g, srgb.b);
        Xyz::new(x, y, z)
    }
}
impl From<Hsv> for Xyz {
    fn from(hsv: Hsv) -> Self {
        let (r, g, b) = hsv_to_srgb(hsv.h, hsv.s, hsv.v);
        let (x, y, z) = srgb_to_xyz(r, g, b);
        Xyz::new(x, y, z)
    }
}
impl From<Lab> for Xyz {
    fn from(lab: Lab) -> Self {
        let (x, y, z) = lab_to_xyz(lab);
        Xyz::new(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xyz_to_srgb() {
        // D50 white point (used as PCS) should map to sRGB white
        let xyz = D50_XYZ;
        let srgb: Srgb = xyz.into();
        println!("D50 XYZ -> srgb: {:?}", srgb);
        // D50 XYZ white → sRGB white (via Bradford D50→D65 adaptation)
        assert!(srgb.r > 1.0 - COLOR_EPSILON && srgb.r <= 1.0);
        assert!(srgb.g > 1.0 - COLOR_EPSILON && srgb.g <= 1.0);
        assert!(srgb.b > 1.0 - COLOR_EPSILON && srgb.b <= 1.0);
    }

    #[test]
    fn test_srgb_to_hsv() {
        let srgb = Srgb::new(1.0, 1.0, 1.0);
        let hsv: Hsv = Xyz::from(srgb).into();
        println!("hsv: {:?}", hsv);
        assert!(hsv.h >= 0.0 && hsv.h <= COLOR_EPSILON);
        assert!(hsv.s >= 0.0 && hsv.s <= COLOR_EPSILON);
        assert!(hsv.v > 1.0 - COLOR_EPSILON && hsv.v <= 1.0);
    }

    #[test]
    fn test_xyz_to_lab() {
        let xyz = D50_XYZ;
        let lab: Lab = xyz.into();
        println!("lab: {:?}", lab);
        assert!(lab.l > 100.0 - COLOR_EPSILON && lab.l <= 100.0);
        assert!(lab.a > -COLOR_EPSILON && lab.a < COLOR_EPSILON);
        assert!(lab.b > -COLOR_EPSILON && lab.b < COLOR_EPSILON);
    }

    #[test]
    fn test_color_space_enum() {
        let xyz = Xyz::new(1.0, 1.0, 1.0);
        let cs: ColorSpace = xyz.into();
        match cs {
            ColorSpace::Xyz(l) => assert_eq!(l, xyz),
            _ => panic!("Expected Xyz variant"),
        }
    }
}
