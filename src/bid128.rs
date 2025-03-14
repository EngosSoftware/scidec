//! # Recognizer for 128-bit floating-point decimals

use crate::recognizer::{recognize, Value, FLAG_INEXACT, FLAG_OVERFLOW, FLAG_UNDERFLOW};
use crate::Rounding;

/// 128-bit decimal in binary format.
pub struct Bid128 {
  pub w: [u64; 2],
}

const BID128_BIAS: i32 = 6176;

const BID128_EMAX: i32 = 6144;

const BID128_NAX_DIGITS: i32 = 34;

const BID128_SIGN: u64 = 0x8000000000000000;

const MAX_COEFFICIENT: u128 = 9999999999999999999999999999999999;

const MAX_EXPONENT: i32 = BID128_EMAX - BID128_NAX_DIGITS + 1;

const BID128_NAN: Bid128 = Bid128 {
  w: [0x0000000000000000, 0x7c00000000000000],
};

const BID128_NEG_NAN: Bid128 = Bid128 {
  w: [0x0000000000000000, 0xfc00000000000000],
};

const BID128_SNAN: Bid128 = Bid128 {
  w: [0x0000000000000000, 0x7e00000000000000],
};

const BID128_NEG_SNAN: Bid128 = Bid128 {
  w: [0x0000000000000000, 0xfe00000000000000],
};

const BID128_INF: Bid128 = Bid128 {
  w: [0x0000000000000000, 0x7800000000000000],
};

const BID128_NEG_INF: Bid128 = Bid128 {
  w: [0x0000000000000000, 0xf800000000000000],
};

const BID128_MIN: Bid128 = Bid128 {
  w: [0x0000000000000000, 0x0000000000000000],
};

const BID128_NEG_MIN: Bid128 = Bid128 {
  w: [0x0000000000000000, 0x8000000000000000],
};

/// Parses a 128-bit floating-point decimal from text in scientific notation.
pub fn bid128_from_string(input: &str) -> (Bid128, u32) {
  bid128_from_string_rnd(input, Rounding::ToNearest)
}

/// Parses a 128-bit floating-point decimal from text in scientific notation, with rounding mode.
pub fn bid128_from_string_rnd(input: &str, rnd: Rounding) -> (Bid128, u32) {
  match recognize(input, BID128_NAX_DIGITS as usize, rnd) {
    Value::Finite(sign, mut value, mut exponent, status) => {
      let mut flags = status;
      let e;
      if value == 0 {
        if exponent < -(BID128_BIAS + BID128_NAX_DIGITS) {
          flags |= FLAG_UNDERFLOW | FLAG_INEXACT;
          e = 0;
        } else if exponent < -BID128_BIAS {
          e = 0;
        } else if exponent < MAX_EXPONENT {
          e = (BID128_BIAS + exponent) as u64;
        } else {
          e = (BID128_BIAS + MAX_EXPONENT) as u64;
        }
      } else {
        if exponent > MAX_EXPONENT {
          // try to normalize before reporting an overflow
          while exponent > MAX_EXPONENT {
            let n = value * 10;
            if n <= MAX_COEFFICIENT {
              value = n;
              exponent -= 1;
            } else {
              break;
            }
          }
          if exponent > MAX_EXPONENT {
            // +inf, overflow, inexact
            flags |= FLAG_OVERFLOW | FLAG_INEXACT;
            return if sign {
              (BID128_NEG_INF, flags)
            } else {
              (BID128_INF, flags)
            };
          }
        }
        if exponent < -BID128_BIAS {
          // try to normalize before reporting an underflow
          while exponent < -BID128_BIAS {
            let r = value % 10;
            if r == 0 {
              value /= 10;
              exponent += 1;
            } else {
              break;
            }
          }
          if exponent < -BID128_BIAS {
            //  underflow, inexact
            flags |= FLAG_UNDERFLOW | FLAG_INEXACT;
            return if sign {
              (BID128_NEG_MIN, flags)
            } else {
              (BID128_MIN, flags)
            };
          }
        }
        e = (BID128_BIAS + exponent) as u64;
      }
      let s = if sign { BID128_SIGN } else { 0 };
      (
        Bid128 {
          w: [value as u64, ((value >> 64) as u64) | e << 49 | s],
        },
        flags,
      )
    }
    Value::Infinity(sign) => {
      if sign {
        (BID128_NEG_INF, 0)
      } else {
        (BID128_INF, 0)
      }
    }
    Value::NaN(sign, signaling) => match (sign, signaling) {
      (false, false) => (BID128_NAN, 0),
      (false, true) => (BID128_SNAN, 0),
      (true, false) => (BID128_NEG_NAN, 0),
      (true, true) => (BID128_NEG_SNAN, 0),
    },
  }
}
