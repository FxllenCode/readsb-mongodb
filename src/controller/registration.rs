// Ported from @wiedehopf's tar1090 registration.js file
// https://github.com/wiedehopf/tar1090/blob/master/html/registrations.js
// All credit belongs to him for the basic implementation of this!
use radix_fmt::radix;

pub struct RegularPattern {
    pub start: u32,
    pub s1: u32,
    pub s2: u32,
    pub prefix: String,
    pub alphabet: Vec<char>,
    pub first: Option<String>,
    pub last: Option<String>,
    pub end: u32,
    pub offset: u32,
}

impl RegularPattern {
    // Derive the end value from the start, first, and last values
    pub fn derive_data(&mut self) {
        if self.first.is_some() {
            let mapping: Vec<char> = self.first.clone().unwrap().chars().collect();
            let c1 = self.alphabet.iter().position(|&s| s == mapping[0]);
            let c2 = self.alphabet.iter().position(|&s| s == mapping[1]);
            let c3 = self.alphabet.iter().position(|&s| s == mapping[2]);
            self.offset =
                c1.unwrap() as u32 * self.s1 + c2.unwrap() as u32 * self.s2 + c3.unwrap() as u32;
        } else {
            self.offset = 0;
        }
        if self.last.is_some() {
            let mapping: Vec<char> = self.last.clone().unwrap().chars().collect();
            let c1 = self.alphabet.iter().position(|&s| s == mapping[0]);
            let c2 = self.alphabet.iter().position(|&s| s == mapping[1]);
            let c3 = self.alphabet.iter().position(|&s| s == mapping[2]);
            self.end = self.start - self.offset
                + c1.unwrap() as u32 * self.s1
                + c2.unwrap() as u32 * self.s2
                + c3.unwrap() as u32;
        } else {
            self.end = self.start - self.offset
                + (self.alphabet.len() as u32 - 1) * self.s1
                + (self.alphabet.len() as u32 - 1) * self.s2
                + (self.alphabet.len() as u32 - 1)
        }
    }
}

pub struct NumericPattern {
    pub start: u32,
    pub first: u32,
    pub count: u32,
    pub template: String,
    pub end: u32,
}
// Derive the end value from the start, first, and count values
impl NumericPattern {
    pub fn derive(&mut self) {
        self.end = self.start + self.count - 1;
    }
}

// Create the NumericPattern array from the tuple of mappings
fn create_from_num(mappings: Vec<(u32, u32, u32, &str)>) -> Vec<NumericPattern> {
    let mut array: Vec<NumericPattern> = Vec::new();
    for map in mappings {
        let (start, first, count, template) = map;
        let pattern: NumericPattern = NumericPattern {
            start,
            first,
            count,
            template: template.to_string(),
            end: 0,
        };
        array.push(pattern)
    }
    array
}
// Create the RegularPattern array from the tuple of mappings
fn create_from_mappings(
    mappings: Vec<(u32, u32, u32, &str, Option<&str>, Option<&str>)>,
) -> Vec<RegularPattern> {
    let mut array: Vec<RegularPattern> = Vec::new();
    for map in mappings {
        let (start, s1, s2, prefix, first, last) = map;
        let regular_pattern = RegularPattern {
            start,
            s1,
            s2,
            prefix: prefix.to_string(),
            alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
            first: first.map(|first| first.to_string()),
            last: last.map(|last| last.to_string()),
            end: 0,
            offset: 0,
        };
        array.push(regular_pattern);
    }
    array
}

// Handle the stride pattern type of icao hex
pub fn stride_reg(hex: u32) -> Option<String> {
    let stride_mappings = vec![
        // South African stride mapping apparently no longer in use
        // 0: first hexid o
        (0x380000, 1024, 32, "F-B", None, None),
        (0x388000, 1024, 32, "F-I", None, None),
        (0x390000, 1024, 32, "F-G", None, None),
        (0x398000, 1024, 32, "F-H", None, None),
        (0x3A0000, 1024, 32, "F-O", None, None),
        (0x3C4421, 1024, 32, "D-A", Some("AAA"), Some("OZZ")),
        (0x3C0001, 26 * 26, 26, "D-A", Some("PAA"), Some("ZZZ")),
        (0x3C8421, 1024, 32, "D-B", Some("AAA"), Some("OZZ")),
        (0x3C2001, 26 * 26, 26, "D-B", Some("PAA"), Some("ZZZ")),
        (0x3CC000, 26 * 26, 26, "D-C", None, None),
        (0x3D04A8, 26 * 26, 26, "D-E", None, None),
        (0x3D4950, 26 * 26, 26, "D-F", None, None),
        (0x3D8DF8, 26 * 26, 26, "D-G", None, None),
        (0x3DD2A0, 26 * 26, 26, "D-H", None, None),
        (0x3E1748, 26 * 26, 26, "D-I", None, None),
        (0x448421, 1024, 32, "OO-", None, None),
        (0x458421, 1024, 32, "OY-", None, None),
        (0x460000, 26 * 26, 26, "OH-", None, None),
        (0x468421, 1024, 32, "SX-", None, None),
        (0x490421, 1024, 32, "CS-", None, None),
        (0x4A0421, 1024, 32, "YR-", None, None),
        (0x4B8421, 1024, 32, "TC-", None, None),
        (0x740421, 1024, 32, "JY-", None, None),
        (0x760421, 1024, 32, "AP-", None, None),
        (0x768421, 1024, 32, "9V-", None, None),
        (0x778421, 1024, 32, "YK-", None, None),
        (0xC00001, 26 * 26, 26, "C-F", None, None),
        (0xC044A9, 26 * 26, 26, "C-G", None, None),
        (0xE01041, 4096, 64, "LV-", None, None),
    ];

    let mapped: Vec<RegularPattern> = create_from_mappings(stride_mappings);
    for mut map in mapped {
        map.derive_data();
        if hex < map.start || hex > map.end {
            continue;
        }
        let mut offset = hex - map.start + map.offset;
        let i1 = offset / map.s1;
        offset %= map.s1;
        let i2 = offset / map.s2;
        offset %= map.s2;
        let i3 = offset;
        if i1 >= map.alphabet.clone().len() as u32
            || i2 >= map.alphabet.clone().len() as u32
            || i3 >= map.alphabet.clone().len() as u32
        {
            continue;
        }
        return Some(
            map.prefix
                + map.alphabet[i1 as usize].to_string().as_str()
                + map.alphabet[i2 as usize].to_string().as_str()
                + map.alphabet[i3 as usize].to_string().as_str(),
        );
    }
    None
}
// Find the Registration for a given hexid that does not follow the common standards
fn num_reg(hex: u32) -> Option<String> {
    let num_mappings = vec![
        (0x140000, 0, 100000, "RA-00000"),
        (0x0B03E8, 1000, 1000, "CU-T0000"),
    ];

    let mapped: Vec<NumericPattern> = create_from_num(num_mappings);
    for mut map in mapped {
        map.derive();
        if hex < map.start || hex > map.end {
            continue;
        }
        let reg = hex - map.start + map.first;
        let reg = reg.to_string();
        // Replace every zero in the template with the respective digit in order from reg. Remember, there are multiple zeros per template and the amount differs, so handle that
        let mut num: Vec<char> = map.template.clone().chars().collect();
        let reg: Vec<char> = reg.chars().collect();
        let mut i = 0;
        for (index, c) in num.clone().iter().enumerate() {
            if *c == '0' {
                num[index] = reg[i];
                i += 1;
            }
        }

        let s: String = String::from_iter(num);

        return Some(s);
    }
    None
}
// US UTILITIES
fn n_letter(mut rem: u32) -> Option<String> {
    if rem == 0 {
        return None;
    }

    rem -= 1;
    let letter: Vec<char> = "ABCDEFGHJKLMNPQRSTUVWXYZ".chars().collect();
    Some(letter[rem as usize].to_string())
}
// US UTILITIES
fn n_letters(mut rem: u32) -> Option<String> {
    if rem == 0 {
        None
    } else {
        rem -= 1;
        let letter: Vec<char> = "ABCDEFGHJKLMNPQRSTUVWXYZ".chars().collect();
        let num: usize = (rem / 25) as usize;
        Some(letter[num].to_string() + &n_letter(rem % 25).unwrap())
    }
}

// United States

pub fn n_reg(hex: u32) -> Option<String> {
    if !(0xA00001..=0xA8FFFF).contains(&hex) {
        return None;
    }
    let mut offset = hex - 0xA00001;

    if offset >= 915399 {
        return None;
    }

    let digit1 = (offset / 101711) + 1;
    let mut reg = "N".to_string() + &digit1.to_string();
    offset %= 101711;
    if offset <= 600 {
        return Some(reg + &n_letters(offset).unwrap());
    }

    offset -= 601;
    let digit2 = offset / 10111;
    reg = reg + &digit2.to_string();
    offset %= 10111;
    if offset <= 600 {
        return Some(reg + &n_letters(offset).unwrap());
    }
    offset -= 601;
    let digit3 = offset / 951;
    reg = reg + &digit3.to_string();
    offset %= 951;
    if offset <= 600 {
        return Some(reg + &n_letters(offset).unwrap());
    }
    offset -= 601;
    let digit4 = offset / 35;
    reg = reg + &digit4.to_string();
    offset %= 35;
    if offset <= 24 {
        return Some(reg + &n_letter(offset).unwrap());
    }

    offset -= 25;
    Some(reg + &offset.to_string())
}

// South Korean

fn hl_reg(hex: u32) -> Option<String> {
    if (0x71BA00..=0x71bf99).contains(&hex) {
        return Some("HL".to_string() + &radix(hex - 0x71BA00 + 0x7200, 16).to_string());
    }

    if (0x71C000..=0x71C099).contains(&hex) {
        return Some("HL".to_string() + &radix(hex - 0x71C000 + 0x8000, 16).to_string());
    }

    if (0x71C200..=0x71C299).contains(&hex) {
        return Some("HL".to_string() + &radix(hex - 0x71C200 + 0x8200, 16).to_string());
    }

    None
}

// Japan
pub fn ja_reg(hex: u32) -> Option<String> {
    if hex < 0x840000 {
        return None;
    }
    let mut offset = hex - 0x840000;
    if offset >= 229840 {
        return None;
    }
    let mut reg = "JA".to_string();
    let digit1 = offset / 22984;
    if digit1 > 9 {
        return None;
    }
    reg = reg + &digit1.to_string();
    offset %= 22984;

    let digit2 = offset / 916;
    if digit2 > 9 {
        return None;
    }
    reg = reg + &digit2.to_string();
    offset %= 916;
    let limited: Vec<char> = "ABCDEFGHJKLMNPQRSTUVWXYZ".chars().collect();

    if offset < 340 {
        let digit3 = offset / 34;
        reg += &digit3.to_string();
        offset %= 34;
        if offset < 10 {
            return Some(reg + offset.to_string().as_str());
        }
        offset -= 10;
        return Some(reg + limited[offset as usize].to_string().as_str());
    }

    offset -= 340;
    let digit3 = offset / 24;
    Some(reg + &digit3.to_string() + &limited[offset as usize % 24].to_string())
}

pub async fn registration(main: &str) -> Option<String> {
    let hex = u32::from_str_radix(main, 16).unwrap();
    if let Some(reg) = n_reg(hex) {
        return Some(reg);
    }
    if let Some(reg) = ja_reg(hex) {
        return Some(reg);
    }
    if let Some(reg) = hl_reg(hex) {
        return Some(reg);
    }
    if let Some(reg) = num_reg(hex) {
        return Some(reg);
    }
    if let Some(reg) = stride_reg(hex) {
        return Some(reg);
    }

    None
}
