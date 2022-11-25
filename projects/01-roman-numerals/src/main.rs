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
}
