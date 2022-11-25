pub fn to_roman(number: u32) -> String {
    let mut rest = number;

    let thousands = rest / 1000;
    rest %= 1000;

    let five_hundreds = rest / 500;
    rest %= 500;

    let hundreds = rest / 100;
    rest %= 100;

    let fifties = rest / 50;
    rest %= 50;

    let tens = rest / 10;
    rest %= 10;

    let fives = rest / 5;
    let ones = rest % 5;

    let m: String = (0..thousands).map(|_x| String::from("M")).collect();
    let d: String = (0..five_hundreds).map(|_x| String::from("D")).collect();
    let c: String = (0..hundreds).map(|_x| String::from("C")).collect();
    let l: String = (0..fifties).map(|_x| String::from("L")).collect();
    let x: String = (0..tens).map(|_x| String::from("X")).collect();
    let v: String = (0..fives).map(|_x| String::from("V")).collect();
    let i: String = (0..ones).map(|_x| String::from("I")).collect();

    m + &d + &c + &l + &x + &v + &i
}

pub fn to_modern_roman(number: u32) -> String {
    let mut rest = number;

    let thousands = rest / 1000;
    rest %= 1000;

    let mut nine_hundreds: u32 = 0;

    if rest >= 900 {
        nine_hundreds = 1;
        rest %= 900;
    }

    let five_hundreds = rest / 500;
    rest %= 500;

    let mut four_hundreds: u32 = 0;

    if rest >= 400 {
        four_hundreds = 1;
        rest %= 400;
    }

    let hundreds = rest / 100;
    rest %= 100;

    let mut nineties: u32 = 0;

    if rest >= 90 {
        nineties = 1;
        rest %= 90;
    }

    let fifties = rest / 50;
    rest %= 50;

    let mut forties: u32 = 0;

    if rest >= 40 {
        forties = 1;
        rest %= 40;
    }

    let tens = rest / 10;
    rest %= 10;

    let mut nines: u32 = 0;

    if rest >= 9 {
        nines = 1;
        rest %= 9;
    }

    let fives = rest / 5;
    rest %= 5;

    let mut fours: u32 = 0;

    if rest >= 4 {
        fours = 1;
        rest %= 4;
    }

    let ones = rest % 5;

    let m: String = (0..thousands).map(|_x| String::from("M")).collect();
    let cm: String = (0..nine_hundreds).map(|_x| String::from("CM")).collect();
    let d: String = (0..five_hundreds).map(|_x| String::from("D")).collect();
    let cd: String = (0..four_hundreds).map(|_x| String::from("CD")).collect();
    let c: String = (0..hundreds).map(|_x| String::from("C")).collect();
    let xc: String = (0..nineties).map(|_x| String::from("XC")).collect();
    let l: String = (0..fifties).map(|_x| String::from("L")).collect();
    let xl: String = (0..forties).map(|_x| String::from("XL")).collect();
    let x: String = (0..tens).map(|_x| String::from("X")).collect();
    let ix: String = (0..nines).map(|_x| String::from("IX")).collect();
    let v: String = (0..fives).map(|_x| String::from("V")).collect();
    let iv: String = (0..fours).map(|_x| String::from("IV")).collect();
    let i: String = (0..ones).map(|_x| String::from("I")).collect();

    m + &cm + &d + &cd + &c + &xc + &l + &xl + &x + &ix + &v + &iv + &i
}

#[cfg(test)]
mod tests {
    // This brings exteernal code into scope
    use super::*;

    #[test]
    fn test_to_roman() {
        assert_eq!(to_roman(4), "IIII");
        assert_eq!(to_roman(9), "VIIII");
        assert_eq!(to_roman(82), "LXXXII");
        assert_eq!(to_roman(748), "DCCXXXXVIII");
        assert_eq!(to_roman(1466), "MCCCCLXVI");
        assert_eq!(to_roman(2921), "MMDCCCCXXI");
    }

    #[test]
    fn test_to_modern_roman() {
        assert_eq!(to_modern_roman(4), "IV");
        assert_eq!(to_modern_roman(9), "IX");
        assert_eq!(to_modern_roman(14), "XIV");
        assert_eq!(to_modern_roman(82), "LXXXII");
        assert_eq!(to_modern_roman(499), "CDXCIX");
        assert_eq!(to_modern_roman(748), "DCCXLVIII");
        assert_eq!(to_modern_roman(944), "CMXLIV");
        assert_eq!(to_modern_roman(1466), "MCDLXVI");
        assert_eq!(to_modern_roman(2921), "MMCMXXI");
    }
}
