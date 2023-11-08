use anyhow::anyhow;

#[derive(Clone, Debug)]
pub enum JetPattern {
    Right,
    Left,
}

impl TryFrom<char> for JetPattern {
    type Error = anyhow::Error;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '>' => Ok(Self::Right),
            '<' => Ok(Self::Left),
            _ => Err(anyhow!("Undefined pattern")),
        }
    }
}

impl JetPattern {
    pub fn apply(&self, current: isize) -> isize {
        match self {
            Self::Left => current - 1,
            Self::Right => current + 1,
        }
    }
}
