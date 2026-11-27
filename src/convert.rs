//! Length conversion logic. Pure Rust, no Android code.

/// (name, symbol, size of one unit in metres)
pub const UNITS: [(&str, &str, f64); 12] = [
    ("Kilometre", "km", 1000.0),
    ("Metre", "m", 1.0),
    ("Decimetre", "dm", 0.1),
    ("Centimetre", "cm", 0.01),
    ("Millimetre", "mm", 0.001),
    ("Micrometre", "µm", 1e-6),
    ("Nanometre", "nm", 1e-9),
    ("Mile", "mi", 1609.344),
    ("Yard", "yd", 0.9144),
    ("Foot", "ft", 0.3048),
    ("Inch", "in", 0.0254),
    ("Nautical mile", "nmi", 1852.0),
];

/// Longest value the user can type.
pub const MAX_INPUT: usize = 15;

pub fn convert(value: f64, from: usize, to: usize) -> f64 {
    value * UNITS[from].2 / UNITS[to].2
}

/// Shows up to 10 significant digits; very large or small values use e-notation.
pub fn format_value(v: f64) -> String {
    if v == 0.0 {
        return "0".to_string();
    }
    let sci = format!("{:.9e}", v);
    let (mantissa, exp) = sci.split_once('e').expect("e-notation");
    let exp: i32 = exp.parse().expect("exponent");
    if !(-9..15).contains(&exp) {
        format!("{}e{}", trim(mantissa), exp)
    } else {
        let decimals = (9 - exp).max(0) as usize;
        let rounded: f64 = sci.parse().expect("number");
        trim(&format!("{:.*}", decimals, rounded))
    }
}

fn trim(s: &str) -> String {
    if s.contains('.') {
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    } else {
        s.to_string()
    }
}

/// The state of all the boxes: only the box being typed in is stored;
/// every other box is calculated from it.
#[derive(Default)]
pub struct Calculator {
    pub active: usize,
    input: String,
    /// True right after choosing a box: the next digit replaces its value.
    fresh: bool,
}

impl Calculator {
    pub fn text(&self, unit: usize) -> String {
        if unit == self.active {
            return self.input.clone();
        }
        match self.input.parse::<f64>() {
            Ok(v) => format_value(convert(v, self.active, unit)),
            Err(_) => String::new(),
        }
    }

    pub fn select(&mut self, unit: usize) {
        if unit != self.active {
            self.input = self.text(unit);
            self.active = unit;
            self.fresh = true;
        }
    }

    /// Accepts '0'..='9' and '.'
    pub fn press(&mut self, key: char) {
        if !(key.is_ascii_digit() || key == '.') {
            return;
        }
        if self.fresh {
            self.input.clear();
            self.fresh = false;
        }
        if self.input.len() >= MAX_INPUT || (key == '.' && self.input.contains('.')) {
            return;
        }
        if key == '.' && self.input.is_empty() {
            self.input.push('0');
        }
        if self.input == "0" && key != '.' {
            self.input.clear();
        }
        self.input.push(key);
    }

    pub fn backspace(&mut self) {
        self.fresh = false;
        self.input.pop();
        // Don't leave half of an e-notation number behind.
        if self.input.ends_with('-') || self.input.ends_with('e') {
            self.input = self.input.split('e').next().unwrap_or("").to_string();
        }
    }

    pub fn clear(&mut self) {
        self.input.clear();
        self.fresh = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn idx(sym: &str) -> usize {
        UNITS.iter().position(|u| u.1 == sym).unwrap()
    }

    fn typed(unit: &str, s: &str) -> Calculator {
        let mut c = Calculator::default();
        c.select(idx(unit));
        s.chars().for_each(|k| c.press(k));
        c
    }

    #[test]
    fn known_values() {
        let c = typed("km", "1");
        assert_eq!(c.text(idx("m")), "1000");
        assert_eq!(c.text(idx("cm")), "100000");
        assert_eq!(c.text(idx("mm")), "1000000");
        assert_eq!(c.text(idx("mi")), "0.6213711922");
        assert_eq!(typed("in", "1").text(idx("cm")), "2.54");
        assert_eq!(typed("mi", "1").text(idx("ft")), "5280");
        assert_eq!(typed("yd", "1").text(idx("in")), "36");
        assert_eq!(typed("ft", "1").text(idx("m")), "0.3048");
        assert_eq!(typed("nmi", "1").text(idx("km")), "1.852");
    }

    #[test]
    fn every_pair_round_trips() {
        for a in 0..UNITS.len() {
            for b in 0..UNITS.len() {
                let back = convert(convert(123.456, a, b), b, a);
                assert!((back - 123.456).abs() < 1e-9);
            }
        }
    }

    #[test]
    fn typing_rules() {
        assert_eq!(typed("m", "..5.").text(idx("m")), "0.5");
        assert_eq!(typed("m", "007").text(idx("m")), "7");
        assert_eq!(typed("m", &"9".repeat(30)).text(idx("m")).len(), MAX_INPUT);
    }

    #[test]
    fn empty_clears_all() {
        let mut c = typed("m", "5");
        c.backspace();
        assert!((0..UNITS.len()).all(|i| c.text(i).is_empty()));
        let mut c = typed("m", "5");
        c.clear();
        assert!((0..UNITS.len()).all(|i| c.text(i).is_empty()));
    }

    #[test]
    fn switching_boxes() {
        let mut c = typed("km", "2");
        c.select(idx("m"));
        assert_eq!(c.text(idx("m")), "2000");
        c.press('5'); // replaces the old value
        assert_eq!(c.text(idx("km")), "0.005");
    }

    #[test]
    fn extremes_and_backspace_on_exponent() {
        let mut c = typed("nm", "1");
        assert_eq!(c.text(idx("km")), "1e-12");
        c.select(idx("km"));
        for _ in 0..10 {
            c.backspace();
        }
        assert_eq!(c.text(idx("km")), "");
        assert_eq!(format_value(0.0), "0");
        assert_eq!(format_value(1e20), "1e20");
    }
}
