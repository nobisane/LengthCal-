//! Length conversion logic. Pure Rust, no Android code.

/// (name, symbol, size of one unit in metres), in on-screen order.
pub const UNITS: [(&str, &str, f64); 8] = [
    ("Kilometer", "km", 1000.0),
    ("Meter", "m", 1.0),
    ("Yard", "yd", 0.9144),
    ("Foot", "ft", 0.3048),
    ("Inch", "in", 0.0254),
    ("Centimeter", "cm", 0.01),
    ("Millimeter", "mm", 0.001),
    ("Mile", "mi", 1609.344),
];

pub fn convert(value: f64, from: usize, to: usize) -> f64 {
    value * UNITS[from].2 / UNITS[to].2
}

/// 6 significant digits, without ever rounding off whole-number digits.
/// Very large or very small values use e-notation.
pub fn format_value(v: f64) -> String {
    if v == 0.0 {
        return "0".to_string();
    }
    let sci = format!("{:.5e}", v);
    let (mantissa, exp) = sci.split_once('e').expect("e-notation");
    let exp: i32 = exp.parse().expect("exponent");
    if !(-9..15).contains(&exp) {
        format!("{}e{}", trim(mantissa), exp)
    } else {
        trim(&format!("{:.*}", (5 - exp).max(0) as usize, v))
    }
}

fn trim(s: &str) -> String {
    if s.contains('.') {
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    } else {
        s.to_string()
    }
}

/// The text for every box after the user types `text` into box `from`.
/// Empty or invalid input gives None (the other boxes are blanked).
pub fn convert_all(text: &str, from: usize) -> Option<[String; 8]> {
    let value: f64 = text.trim().replace(',', ".").parse().ok()?;
    if !value.is_finite() || value < 0.0 {
        return None;
    }
    Some(std::array::from_fn(|to| {
        if to == from {
            text.to_string()
        } else {
            format_value(convert(value, from, to))
        }
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn idx(sym: &str) -> usize {
        UNITS.iter().position(|u| u.1 == sym).unwrap()
    }

    #[test]
    fn matches_design_image() {
        let got = convert_all("5.9", idx("ft")).unwrap();
        assert_eq!(
            got,
            [
                "0.00179832",
                "1.79832",
                "1.96667",
                "5.9",
                "70.8",
                "179.832",
                "1798.32",
                "0.00111742"
            ]
        );
    }

    #[test]
    fn known_values() {
        assert_eq!(convert_all("1", idx("km")).unwrap()[idx("mm")], "1000000");
        assert_eq!(convert_all("1", idx("mi")).unwrap()[idx("ft")], "5280");
        assert_eq!(convert_all("1", idx("in")).unwrap()[idx("cm")], "2.54");
        assert_eq!(format_value(1609344.0), "1609344");
    }

    #[test]
    fn every_pair_round_trips() {
        for a in 0..8 {
            for b in 0..8 {
                assert!((convert(convert(123.456, a, b), b, a) - 123.456).abs() < 1e-9);
            }
        }
    }

    #[test]
    fn partial_and_bad_input() {
        for bad in ["", ".", "abc", "-1", "inf", "NaN"] {
            assert!(convert_all(bad, 1).is_none(), "{bad}");
        }
        assert_eq!(convert_all("2.", 1).unwrap()[1], "2.");
        assert_eq!(convert_all("1,5", 1).unwrap()[idx("cm")], "150");
    }
}
