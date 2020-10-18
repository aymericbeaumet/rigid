pub type Error = ();

pub type Result<T> = std::result::Result<T, Error>;

#[inline]
#[must_use]
pub fn skip_whitespaces(bytes: &[u8]) -> usize {
    bytes
        .iter()
        .take_while(|b| matches!(b, b' ' | 0x0A | 0x0D | 0x09))
        .count()
}

#[inline]
pub const fn eat_char(bytes: &[u8], c: u8) -> Result<usize> {
    if !bytes.is_empty() && bytes[0] == c {
        Ok(1)
    } else {
        Err(())
    }
}

#[inline]
pub const fn eat_u8(bytes: &[u8]) -> Result<(usize, u8)> {
    let mut idx = 0;
    let mut out = 0;
    while idx < bytes.len() && bytes[idx] >= b'0' && bytes[idx] <= b'9' {
        out = out * 10 + (bytes[idx] - b'0');
        idx += 1;
    }
    if idx > 0 {
        Ok((idx, out))
    } else {
        Err(())
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
        Err(())
    }
}

#[inline]
pub fn eat_string(bytes: &[u8]) -> Result<(usize, String)> {
    let mut idx = 0;
    idx += skip_whitespaces(&bytes[idx..]);
    idx += eat_char(&bytes[idx..], b'"')?;

    let (delta, found) = eat_u8_slice_until_char(&bytes[idx..], b'"')?;
    idx += delta;

    idx += eat_char(&bytes[idx..], b'"')?;
    idx += skip_whitespaces(&bytes[idx..]);

    Ok((idx, std::str::from_utf8(found).unwrap().to_string()))
}

#[inline]
pub const fn eat_bool(bytes: &[u8]) -> Result<(usize, bool)> {
    let len = bytes.len();

    if len >= 4 {
        if bytes[0] == b't' && bytes[1] == b'r' && bytes[2] == b'u' && bytes[3] == b'e' {
            return Ok((4, true));
        }

        if len >= 5
            && bytes[0] == b'f'
            && bytes[1] == b'a'
            && bytes[2] == b'l'
            && bytes[3] == b's'
            && bytes[4] == b'e'
        {
            return Ok((5, false));
        }
    }

    Err(())
}

#[inline]
pub fn eat_u8_slice(bytes: &[u8], s: &[u8]) -> Result<usize> {
    if bytes.starts_with(s) {
        Ok(s.len())
    } else {
        Err(())
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
        Err(())
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
