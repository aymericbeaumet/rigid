#[derive(Debug)]
pub struct Error(String);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        s.to_string().into()
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[inline]
pub const fn eat_whitespaces(bytes: &[u8]) -> Result<usize> {
    let mut idx = 0;
    while idx < bytes.len() && bytes[idx] == b' ' {
        idx += 1;
    }
    Ok(idx)
}

#[inline]
pub fn eat_char(bytes: &[u8], c: u8) -> Result<usize> {
    if !bytes.is_empty() && bytes[0] == c {
        Ok(1)
    } else {
        Err(format!(
            "eat_char expected {} ({}) but found {} ({})",
            c, c as char, bytes[0], bytes[0] as char
        )
        .into())
    }
}

#[inline]
pub fn eat_object_key_value(bytes: &[u8], k: &[u8]) -> Result<(usize, u8)> {
    let mut idx = 0;
    idx += eat_object_key(&bytes[idx..], k)?;
    idx += eat_whitespaces(&bytes[idx..])?;
    idx += eat_char(&bytes[idx..], b':')?;
    idx += eat_whitespaces(&bytes[idx..])?;

    let (delta, value) = eat_number(&bytes[idx..])?;
    idx += delta;

    Ok((idx, value))
}

#[inline]
pub fn eat_object_key(bytes: &[u8], k: &[u8]) -> Result<usize> {
    let mut idx = 0;
    idx += eat_char(&bytes[idx..], b'"')?;
    idx += eat_slice(&bytes[idx..], k)?;
    idx += eat_char(&bytes[idx..], b'"')?;
    Ok(idx)
}

#[inline]
pub fn eat_number(bytes: &[u8]) -> Result<(usize, u8)> {
    let mut idx = 0;
    let mut out: u8 = 0;
    while idx < bytes.len() && bytes[idx] >= b'0' && bytes[idx] <= b'9' {
        out = out * 10 + (bytes[idx] - b'0');
        idx += 1;
    }
    if idx > 0 {
        Ok((idx, out))
    } else {
        Err("eat_number couldn't find any digit".into())
    }
}

#[inline]
pub fn eat_slice(bytes: &[u8], s: &[u8]) -> Result<usize> {
    if bytes.starts_with(s) {
        Ok(s.len())
    } else {
        Err(format!(
            "eat_slice could not match {:?} as a prefix of {:?}",
            s, bytes
        )
        .into())
    }
}
