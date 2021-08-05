use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
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
            6 => (),
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
}

impl From<Color> for wgpu::Color {
    fn from(val: Color) -> Self {
        let max = 256.0;
        wgpu::Color {
            r: val.r as f64 / max,
            g: val.g as f64 / max,
            b: val.b as f64 / max,
            a: val.a as f64 / max,
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
        dbg!(color);
    }
}
