use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Color {
    pub r: u32,
    pub g: u32,
    pub b: u32,
    pub a: u32,
}

impl Color {
    pub fn from_rgb(r: u32, g: u32, b: u32, a: u32) -> Self {
        Color { r, g, b, a }
    }

    pub fn from_hex(hex: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut include_alpha = false;
        // Have to convert to upper, as well as ensure len
        match hex.len() {
            6 => {}
            8 => include_alpha = true,
            _ => return Err("Hex is not of proper length".into()),
        }
        let keys = [
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
        ];
        let map: HashMap<char, u32> = keys
            .iter()
            .enumerate()
            .map(|(i, c)| (c.to_owned(), i as u32))
            .collect();

        let upper = hex.to_uppercase();

        let mut iter = upper.chars();

        let mut next_two = || -> Result<u32, Box<dyn std::error::Error>> {
            Ok(
                map.get(&iter.next().unwrap()).ok_or("invalid character")? * 16
                    + map.get(&iter.next().unwrap()).ok_or("invalid character")?,
            )
        };

        let r = next_two()?;
        let g = next_two()?;
        let b = next_two()?;
        let a = if include_alpha { next_two()? } else { 256 };

        Ok(Color { r, g, b, a })
    }

    pub fn fade(mut self, alpha: f32) -> Self {
        self.a = (alpha * 256.0).floor() as u32;
        self
    }
}

#[inline]
fn cv(n: f64) -> f64 {
    (n / 256.0).powf(2.2)
}

/// Converts color from srgb to wgpu color, but corrects for gamma.
/// sRGB is stored in relative color, while our eyes perceive the brightness differently, so we have to
/// modify the sRGB according to the gamma curve, with an exponent of ~ 2.2
/// See [learnopengl/gamma-correction](https://learnopengl.com/Advanced-Lighting/Gamma-Correction) & [learnwgpu/colorcorrection](https://sotrh.github.io/learn-wgpu/beginner/tutorial4-buffer/#color-correction)
/// for more information.
impl From<Color> for wgpu::Color {
    fn from(val: Color) -> Self {
        wgpu::Color {
            r: cv(val.r as f64),
            g: cv(val.g as f64),
            b: cv(val.b as f64),
            a: cv(val.a as f64),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Color;
    #[test]
    fn test_color_from_hex() {
        let color = Color::from_hex("292828").unwrap();
        assert_eq!(
            color,
            Color {
                r: 41,
                g: 40,
                b: 40,
                a: 256
            }
        );
    }
    #[test]
    fn test_color_to_wgpu_color() {
        let color = Color::from_hex("292828").unwrap();
        assert_eq!(
            wgpu::Color::from(color),
            wgpu::Color {
                r: 41.0 / 256.0,
                g: 40.0 / 256.0,
                b: 40.0 / 256.0,
                a: 1.0
            }
        );
    }
}
