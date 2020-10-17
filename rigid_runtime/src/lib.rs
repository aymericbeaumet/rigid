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
pub const fn skip_whitespaces(bytes: &[u8]) -> Result<usize> {
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
pub fn eat_u8(bytes: &[u8]) -> Result<(usize, u8)> {
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

#[inline]
pub fn eat_u16(bytes: &[u8]) -> Result<(usize, u16)> {
    let mut idx = 0;
    let mut out = 0;
    while idx < bytes.len() && bytes[idx] >= b'0' && bytes[idx] <= b'9' {
        out = out * 10 + u16::from(bytes[idx] - b'0');
        idx += 1;
    }
    if idx > 0 {
        Ok((idx, out))
    } else {
        Err(Error::new("eat_number couldn't find any digit", bytes))
    }
}

#[inline]
pub fn eat_string(bytes: &[u8]) -> Result<(usize, String)> {
    let mut idx = 0;
    idx += skip_whitespaces(&bytes[idx..])?;
    idx += eat_char(&bytes[idx..], b'"')?;

    let (delta, found) = eat_u8_slice_until_char(&bytes[idx..], b'"')?;
    idx += delta;

    idx += eat_char(&bytes[idx..], b'"')?;
    idx += skip_whitespaces(&bytes[idx..])?;

    Ok((idx, std::str::from_utf8(found).unwrap().to_string()))
}

#[inline]
pub fn eat_bool(bytes: &[u8]) -> Result<(usize, bool)> {
    let mut idx = 0;
    idx += skip_whitespaces(&bytes[idx..])?;

    let (delta, out) = if bytes.starts_with(&[b't', b'r', b'u', b'e']) {
        (4, true)
    } else if bytes.starts_with(&[b'f', b'a', b'l', b's', b'e']) {
        (5, false)
    } else {
        return Err(Error::new("eat_bool cannot find a boolean", &bytes[idx..]));
    };
    idx += delta;

    idx += skip_whitespaces(&bytes[idx..])?;
    Ok((idx, out))
}

#[inline]
pub fn eat_u8_slice(bytes: &[u8], s: &[u8]) -> Result<usize> {
    if bytes.starts_with(s) {
        Ok(s.len())
    } else {
        Err(Error::new(
            format!(
                "eat_u8_slice could not match {:?} as a prefix of {:?}",
                s, bytes
            ),
            bytes,
        ))
    }
}

#[inline]
pub fn eat_u8_slice_until_char(bytes: &[u8], c: u8) -> Result<(usize, &[u8])> {
    let mut idx = 0;
    while idx < bytes.len() && bytes[idx] != c {
        idx += 1;
    }
    if idx > 0 {
        Ok((idx, &bytes[0..idx]))
    } else {
        Err(Error::new(
            format!("eat_until_char could not find {:?}", c),
            &bytes[idx..],
        ))
    }
}

#[inline]
pub fn eat_object_key(bytes: &[u8], k: &[u8]) -> Result<usize> {
    let mut idx = 0;
    idx += eat_char(&bytes[idx..], b'"')?;
    idx += eat_u8_slice(&bytes[idx..], k)?;
    idx += eat_char(&bytes[idx..], b'"')?;
    Ok(idx)
}
