use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum GridError {
    InvalidLength(usize),
    InvalidFormat,
}
impl fmt::Display for GridError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GridError::InvalidLength(size) => write!(f, "Invalid Length: {}", size),
            GridError::InvalidFormat => write!(f, "Invalid Format"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Grid {
    grid: String,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.grid)
    }
}

impl<'de> Deserialize<'de> for Grid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v: String = Deserialize::deserialize(deserializer)?;
        Grid::new(v).map_err(D::Error::custom)
    }
}
impl Serialize for Grid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.grid)
    }
}

impl Grid {
    pub fn new<S: Into<String>>(grid: S) -> Result<Grid, GridError> {
        let grid = grid.into();
        let length = grid.len();
        if length >= 4 && length <= 10 && (length % 2 == 0) {
            let grid = Grid { grid: grid };
            match grid.coord() {
                Ok(_) => Ok(grid),
                Err(e) => Err(e),
            }
        } else {
            Err(GridError::InvalidLength(length))
        }
    }
    pub fn short(&self) -> String {
        let grid = self.grid.to_string();
        let first = &self.grid[0..4];
        let end: String = grid.chars().skip(4).take(2).collect();
        format!("{}{}", first.to_uppercase(), end.to_lowercase())
    }

    pub fn full(&self) -> String {
        self.grid.to_string()
    }

    pub fn coord(&self) -> Result<(f32, f32), GridError> {
        let grid = self.grid.clone();
        let grid: Vec<char> = grid.to_uppercase().chars().collect();

        match grid.len() {
            4 => match (
                self.maybe_alpha_block([grid[0], grid[1]]),
                self.maybe_numeric_block([grid[2], grid[3]]),
            ) {
                (Some((a, b)), Some((c, d))) => {
                    let long = ((a - ('A' as u8)) as f32) * 20.0
                        + ((c as f32) - (('0' as u8) as f32) + 0.5) * 2.0
                        - 180.0;
                    let lat = ((b - ('A' as u8)) as f32) * 10.0
                        + ((d as f32) - (('0' as u8) as f32) + 0.5)
                        - 90.0;
                    Ok((lat, long))
                }
                _ => Err(GridError::InvalidFormat),
            },
            6 => match (
                self.maybe_alpha_block([grid[0], grid[1]]),
                self.maybe_numeric_block([grid[2], grid[3]]),
                self.maybe_alpha_block([grid[4], grid[5]]),
            ) {
                (Some((a, b)), Some((c, d)), Some((e, f))) => {
                    let long = ((a - ('A' as u8)) as f32) * 20.0
                        + ((c - ('0' as u8)) as f32) * 2.0
                        + ((e as f32) - (('A' as u8) as f32) + 0.5) / 12.0
                        - 180.0;
                    let lat = ((b - ('A' as u8)) as f32) * 10.0
                        + ((d - ('0' as u8)) as f32)
                        + ((f as f32) - (('A' as u8) as f32) + 0.5) / 24.0
                        - 90.0;
                    Ok((lat, long))
                }
                _ => Err(GridError::InvalidFormat),
            },
            8 => match (
                self.maybe_alpha_block([grid[0], grid[1]]),
                self.maybe_numeric_block([grid[2], grid[3]]),
                self.maybe_alpha_block([grid[4], grid[5]]),
                self.maybe_numeric_block([grid[6], grid[7]]),
            ) {
                (Some((a, b)), Some((c, d)), Some((e, f)), Some((g, h))) => {
                    let long = ((a - ('A' as u8)) as f32) * 20.0
                        + ((c - ('0' as u8)) as f32) * 2.0
                        + (e as f32 - (('A' as u8) as f32) + 0.0) / 12.0
                        + (g as f32 - (('0' as u8) as f32) + 0.5) / 120.0
                        - 180.0;
                    let lat = ((b - ('A' as u8)) as f32) * 10.0
                        + ((d - ('0' as u8)) as f32)
                        + (f as f32 - (('A' as u8) as f32) + 0.0) / 24.0
                        + (h as f32 - (('0' as u8) as f32) + 0.5) / 240.0
                        - 90.0;
                    Ok((lat, long))
                }
                _ => Err(GridError::InvalidFormat),
            },
            10 => match (
                self.maybe_alpha_block([grid[0], grid[1]]),
                self.maybe_numeric_block([grid[2], grid[3]]),
                self.maybe_alpha_block([grid[4], grid[5]]),
                self.maybe_numeric_block([grid[6], grid[7]]),
                self.maybe_alpha_block([grid[8], grid[9]]),
            ) {
                (Some((a, b)), Some((c, d)), Some((e, f)), Some((g, h)), Some((i, j))) => {
                    let long = ((a - ('A' as u8)) as f32) * 20.0
                        + ((c - ('0' as u8)) as f32) * 2.0
                        + (e as f32 - (('A' as u8) as f32) + 0.0) / 12.0
                        + (g as f32 - (('0' as u8) as f32) + 0.0) / 120.0
                        + (i as f32 - (('A' as u8) as f32) + 0.5) / 120.0 / 24.0
                        - 180.0;
                    let lat = ((b - ('A' as u8)) as f32) * 10.0
                        + ((d - ('0' as u8)) as f32)
                        + (f as f32 - (('A' as u8) as f32) + 0.0) / 24.0
                        + (h as f32 - (('0' as u8) as f32) + 0.0) / 240.0
                        + (j as f32 - (('A' as u8) as f32) + 0.5) / 240.0 / 24.0
                        - 90.0;
                    Ok((lat, long))
                }
                _ => Err(GridError::InvalidFormat),
            },
            _ => Err(GridError::InvalidLength(grid.len())),
        }
    }

    fn maybe_alpha_block(&self, input: [char; 2]) -> Option<(u8, u8)> {
        match input {
            [f, s] if f >= 'A' && f <= 'Z' && s >= 'A' && s <= 'Z' => Some((f as u8, s as u8)),
            _ => None,
        }
    }

    fn maybe_numeric_block(&self, input: [char; 2]) -> Option<(u8, u8)> {
        match input {
            [f, s] if f >= '0' && f <= '9' && s >= '0' && s <= '9' => Some((f as u8, s as u8)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_gridsquare() {
        let grid = Grid::new("EM73t");
        assert_eq!(grid, Err(GridError::InvalidLength(5)));

        let grid = Grid::new("EMtk73");
        assert_eq!(grid, Err(GridError::InvalidFormat));
    }

    #[test]
    fn test_short_formatting() {
        let grid = Grid::new("em73tk").unwrap();
        assert_eq!(grid.short(), "EM73tk");

        eprintln!("{:?}", grid.coord().unwrap());
    }
}
