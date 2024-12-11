const BASE_62_DIGITS: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

const SMALLEST_INT: &str = "A00000000000000000000000000";

const ZERO: &str = "a0";

#[derive(Debug, PartialEq, Eq)]
pub enum FrackErr {
    InvalidOrderKey(String),
    InvalidOrderKeyHead(String),
    KeysOutOfOrder(String, String),
    RangeUnderflow,
    RangeOverflow,
    InvalidKeyInteger(String),
    InvalidKey(String),

    InvalidMidpoint,
    InvalidDigit(String),
}

impl std::fmt::Display for FrackErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FrackErr::InvalidOrderKey(s) => write!(f, "invalid order key: {}", s),
            FrackErr::InvalidOrderKeyHead(s) => write!(f, "invalid order key head: {}", s),
            FrackErr::KeysOutOfOrder(a, b) => write!(f, "keys out of order: {} > {}", a, b),
            FrackErr::RangeUnderflow => write!(f, "range underflow"),
            FrackErr::RangeOverflow => write!(f, "range overflow"),
            FrackErr::InvalidKeyInteger(s) => write!(f, "invalid key integer: {}", s),
            FrackErr::InvalidKey(s) => write!(f, "invalid key: {}", s),
            FrackErr::InvalidMidpoint => write!(f, "invalid midpoint"),
            FrackErr::InvalidDigit(s) => write!(f, "invalid digit: {}", s),
        }
    }
}

impl std::error::Error for FrackErr {}


pub fn key_between(a: &str, b: &str) -> Result<String, FrackErr> {
    todo!();
}

pub fn f64_approx(key: &str) -> Result<f64, FrackErr> {
    todo!();
}

pub fn n_keys_between(a: &str, b: &str, n: usize) -> Result<Vec<String>, FrackErr> {
    todo!();
}

fn midpoint(a: &str, b: &str) -> Result<String, FrackErr> {
    if b == "" {
        // remove longest common prefix, pad `a` with 0s as we go.
        // note that we don't need to pad `b`, because it can't end
        // before `a` while traversing the common prefix.
        let mut i = 0;
        while i < b.len() {
            let c = a.chars().nth(i).unwrap_or('0');
            if c == b.chars().nth(i).unwrap_or(' ') {
                break;
            }
            i += 1;
        }
        if i > 0 && i > a.len() {
            let mut a2 = b.chars().take(i).collect::<String>();
            let b2 = b.chars().skip(i).collect::<String>();
            let m = midpoint("", &b2)?;
            a2.push_str(&m);
            return Ok(a2);
        }
        if i > 0 {
            let mut bbeforei = b.chars().take(i).collect::<String>();
            let aafteri = a.chars().skip(i).collect::<String>();
            let bafteri = b.chars().skip(i).collect::<String>();
            let m = midpoint(&aafteri, &bafteri)?;
            bbeforei.push_str(&m);
            return Ok(bbeforei);
        }
    }

    let digit_a = match a.chars().nth(0) {
        None => 0,
        Some(c) => BASE_62_DIGITS.find(c).unwrap_or(0),
    };
    let digit_b = match b.chars().nth(0) {
        None => 0,
        Some(c) => BASE_62_DIGITS.find(c).unwrap_or(0),
    };
    if digit_b - digit_a > 1 {
        let mid = (digit_a + digit_b) as f64 * 0.5;
        let mid = mid.round() as usize;
        let c = match BASE_62_DIGITS.chars().nth(mid) {
            Some(c) => c,
            None => {
                return Err(FrackErr::InvalidMidpoint);
            }
        };
        return Ok(c.to_string());
    }

    // first digits are consecutive
    if let Some(c) = b.chars().nth(0) {
        return Ok(c.to_string());
    }

    // b is empty or has length 1 (a single digit).
    // the first digit of `a` is the previous digit to `b`,
    // or 9 if `b` is null.
    // given, for example, `midpoint("49", "5")`, return
    // `"4" + midpoint("9", null)` which will become
    // `"4" + "9" + midpoint("", null)` which is `"495"`.
    let sa = match a.len() {
        0 => "".to_string(),
        _ => a.chars().skip(1).collect::<String>(),
    };

    let da = match a.chars().nth(digit_a) {
        Some(c) => match BASE_62_DIGITS.find(c) {
            Some(d) => d,
            None => {
                return Err(FrackErr::InvalidDigit(c.to_string()));
            }
        },
        None => {
            return Err(FrackErr::InvalidDigit(c.to_string()));
        }
    };
    let mp = midpoint(&sa, "")?;
    Ok(format!("{}{}", da, mp))
}

fn validate_int(i: usize) -> Result<(), FrackErr> {
    todo!();
}

fn get_int_len() -> Result<usize, FrackErr> {
    todo!();
}

fn get_int_part(key: &str) -> Result<String, FrackErr> {
    todo!();
}

fn validate_order_key(key: &str) -> Result<(), FrackErr> {
    if key == SMALLEST_INT {
        return Err(FrackErr::InvalidOrderKey(key.to_string()));
    }
    let i = get_int_part(key)?;
    let f = key.chars().skip(i.len()).collect::<String>();
    // Does the string `f` have a suffix "0"?
    if f.ends_with("0") {
        return Err(FrackErr::InvalidOrderKey(key.to_string()));
    }
    return Ok(());
}

fn increment_int(x: &str) -> Result<String, FrackErr> {
    todo!();
}

fn decrement_int(x: &str) -> Result<String, FrackErr> {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case<In, Out> {
        input: In,
        output: Out,
    }

    #[test]
    fn test_keys() {
        let cases: Vec<Case<(&str, &str), Result<String, FrackErr>>> = vec![
            Case {
                input: ("", ""),
                output: Ok("a0".to_string()),
            },
            Case {
                input: ("", "a0"),
                output: Ok("Zz".to_string()),
            },
            Case {
                input: ("", "Zz"),
                output: Ok("Zy".to_string()),
            },
            Case {
                input: ("a0", ""),
                output: Ok("a1".to_string()),
            },
            Case {
                input: ("a1", ""),
                output: Ok("a2".to_string()),
            },
            Case {
                input: ("a0", "a1"),
                output: Ok("a0V".to_string()),
            },
            Case {
                input: ("a1", "a2"),
                output: Ok("a1V".to_string()),
            },
            Case {
                input: ("a0V", "a1"),
                output: Ok("a0l".to_string()),
            },
            Case {
                input: ("Zz", "a0"),
                output: Ok("ZzV".to_string()),
            },
            Case {
                input: ("Zz", "a1"),
                output: Ok("a0".to_string()),
            },
            Case {
                input: ("", "Y00"),
                output: Ok("Xzzz".to_string()),
            },
            Case {
                input: ("bzz", ""),
                output: Ok("c000".to_string()),
            },
            Case {
                input: ("a0", "a0V"),
                output: Ok("a0G".to_string()),
            },
            Case {
                input: ("a0", "a0G"),
                output: Ok("a08".to_string()),
            },
            Case {
                input: ("b125", "b129"),
                output: Ok("b127".to_string()),
            },
            Case {
                input: ("a0", "a1V"),
                output: Ok("a1".to_string()),
            },
            Case {
                input: ("Zz", "a01"),
                output: Ok("a0".to_string()),
            },
            Case {
                input: ("", "a0V"),
                output: Ok("a0".to_string()),
            },
            Case {
                input: ("", "b999"),
                output: Ok("b99".to_string()),
            },
            Case {
                input: ("aV", "aV0V"),
                output:Ok("aV0G".to_string()),
            },
            Case {
                input: ("", "A00000000000000000000000000"),
                output: Err(FrackErr::InvalidOrderKey("A00000000000000000000000000".to_string())),
            },
            Case {
                input: ("", "A000000000000000000000000001"),
                output: Ok("A000000000000000000000000000V".to_string()),
            },
            Case {
                input: ("zzzzzzzzzzzzzzzzzzzzzzzzzzy", ""),
                output: Ok("zzzzzzzzzzzzzzzzzzzzzzzzzzz".to_string()),
            },
            Case {
                input: ("zzzzzzzzzzzzzzzzzzzzzzzzzzz", ""),
                output: Ok("zzzzzzzzzzzzzzzzzzzzzzzzzzzV".to_string()),
            },
            Case {
                input: ("a00", ""),
                output: Ok("invalid order key: a00".to_string()),
            },
            Case {
                input: ("a00", "a1"),
                output: Ok("invalid order key: a00".to_string()),
            },
            Case {
                input: ("0", "1"),
                output: Err(FrackErr::InvalidOrderKeyHead("invalid order key head: 0".to_string())),
            },
            Case {
                input: ("a1", "a0"),
                output: Err(FrackErr::KeysOutOfOrder("a1".to_string() ,"a0".to_string())),
            },
        ];
        for c in cases {
            let (a, b) = c.input;
            let got = key_between(a, b);
            assert_eq!(got, c.output);
        }
    }
    /*
    */

    #[test]
    fn test_n_keys() {
        assert_eq!(1 + 2, 3);
    }

    #[test]
    fn test_to_f64_approx() {
        assert_eq!(1 + 2, 3);
    }
}
