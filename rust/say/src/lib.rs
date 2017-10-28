//
// See Rust Language Specific Instructions
// below normal exercise description.
//

pub fn encode(number: u32) -> String {
    if number < 20 {
        let s = match number {
            0 => "zero",
            1 => "one",
            2 => "two",
            3 => "three",
            4 => "four",
            5 => "five",
            6 => "six",
            7 => "seven",
            8 => "eight",
            9 => "nine",
            10 => "ten",
            11 => "eleven",
            12 => "twelve",
            13 => "thirteen",
            14 => "fourteen",
            15 => "fifteen",
            16 => "sixteen",
            17 => "seventeen",
            18 => "eighteen",
            19 => "nineteen",
            _ => panic!("impossible")
        };
        String::from(s)
    } else {
        let digits = digits(number);
        let ones = digits[0];
        let tens = digits[1];
        let encoded_tens = match tens {
            2 => "twenty",
            3 => "thirty",
            4 => "forty",
            5 => "fifty",
            6 => "sixty",
            7 => "seventy",
            8 => "eighty",
            9 => "ninety",
            other => panic!("unsupported digit for tens : {}", other)
        };
        let mut s = String::from(encoded_tens);
        if ones > 0 {
            s.push('-');
            s.push_str(&encode(ones as u32));
        }
        s
    }
}

fn digits(number: u32) -> Vec<u8> {
    let mut digits = Vec::new();
    let mut n = number;
    while n > 0 {
        digits.push((n % 10) as u8);
        n /= 10;
    }
    digits
}
