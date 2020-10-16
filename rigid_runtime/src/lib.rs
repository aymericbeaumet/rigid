#[derive(Debug)]
pub struct Error {
    message: String,
    remaining: String,
}

impl Error {
    pub fn new<T: Into<String>>(m: T, r: &[u8]) -> Self {
        Self {
            message: m.into(),
            remaining: std::str::from_utf8(r).unwrap().into(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at position `{}`", self.message, self.remaining)
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
        Err(Error::new(
            format!(
                "eat_char expected {} ({}) but found {} ({})",
                c, c as char, bytes[0], bytes[0] as char
            ),
            bytes,
        ))
    }
}

#[inline]
pub fn eat_object_key_value_u8(bytes: &[u8], k: &[u8]) -> Result<(usize, u8)> {
    let mut idx = 0;
    idx += eat_object_key(&bytes[idx..], k)?;
    idx += eat_whitespaces(&bytes[idx..])?;
    idx += eat_char(&bytes[idx..], b':')?;
    idx += eat_whitespaces(&bytes[idx..])?;

    let (delta, value) = eat_number_u8(&bytes[idx..])?;
    idx += delta;

    Ok((idx, value))
}

#[inline]
pub fn eat_object_key_value_u16(bytes: &[u8], k: &[u8]) -> Result<(usize, u16)> {
    let mut idx = 0;
    idx += eat_object_key(&bytes[idx..], k)?;
    idx += eat_whitespaces(&bytes[idx..])?;
    idx += eat_char(&bytes[idx..], b':')?;
    idx += eat_whitespaces(&bytes[idx..])?;

    let (delta, value) = eat_number_u16(&bytes[idx..])?;
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
pub fn eat_number_u8(bytes: &[u8]) -> Result<(usize, u8)> {
    let mut idx = 0;
    let mut out = 0;
    while idx < bytes.len() && bytes[idx] >= b'0' && bytes[idx] <= b'9' {
        out = out * 10 + (bytes[idx] - b'0');
        idx += 1;
    }
    if idx > 0 {
        Ok((idx, out))
    } else {
        Err(Error::new("eat_number couldn't find any digit", bytes))
    }
}

pub fn eat_number_u16(bytes: &[u8]) -> Result<(usize, u16)> {
    let mut idx = 0;
    let mut out = 0;
    while idx < bytes.len() && bytes[idx] >= b'0' && bytes[idx] <= b'9' {
        out = out * 10 + (bytes[idx] - b'0') as u16;
        idx += 1;
    }
    if idx > 0 {
        Ok((idx, out))
    } else {
        Err(Error::new("eat_number couldn't find any digit", bytes))
    }
}

#[inline]
pub fn eat_slice(bytes: &[u8], s: &[u8]) -> Result<usize> {
    if bytes.starts_with(s) {
        Ok(s.len())
    } else {
        Err(Error::new(
            format!(
                "eat_slice could not match {:?} as a prefix of {:?}",
                s, bytes
            ),
            bytes,
        ))
    }
}
