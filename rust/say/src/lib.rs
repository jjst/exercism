//
// See Rust Language Specific Instructions
// below normal exercise description.
//

pub fn encode(number: u32) -> String {
    let mut chunks = chunks(number);
    let powers = vec![
        "thousand",
        "million",
        "billion",
        "trillion",
        "quadrillion",
        "quintillion",
        "sextillion"
    ];
    let (&hundreds, rest) = chunks.split_first().unwrap();
    println!("{:?}", hundreds);
    println!("{:?}", chunks);
    let mut s = String::new();
    for (&chunk, &power) in rest.iter().zip(powers.iter()).rev() {
        println!("{:?} ${:?}", chunk, power);
        if chunk != 0 {
            s.push_str(&encode_hundreds(chunk as u32));
            s.push(' ');
            s.push_str(&power);
        }
    }
    if hundreds != 0 || number == 0 {
        s.push_str(&encode_hundreds(hundreds as u32));
    }
    s
}

pub fn encode_hundreds(number: u32) -> String {
    if number < 10 {
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
            _ => panic!("impossible")
        };
        String::from(s)
    } else {
        let digits = digits(number);
        let ones = digits[0];
        let tens = digits[1];
        let mut s = String::new();
        match digits.get(2) {
            Some(&d) => {
                s.push_str(&encode(d as u32));
                s.push_str(" hundred");
                if ones != 0 || tens != 0 {
                    s.push(' ');
                }
            }
            None => ()
        };
        if tens == 1 {
            let s2 = match ones {
                0 => "ten",
                1 => "eleven",
                2 => "twelve",
                3 => "thirteen",
                4 => "fourteen",
                5 => "fifteen",
                6 => "sixteen",
                7 => "seventeen",
                8 => "eighteen",
                9 => "nineteen",
                _ => panic!("can't happen")
            };
            s.push_str(s2);
        }
        else {
            let encoded_tens = match tens {
                2 => "twenty",
                3 => "thirty",
                4 => "forty",
                5 => "fifty",
                6 => "sixty",
                7 => "seventy",
                8 => "eighty",
                9 => "ninety",
                0 => "",
                _ => panic!("10 to 19 is special case")
            };
            s.push_str(encoded_tens);
            if ones > 0 {
                s.push('-');
                s.push_str(&encode(ones as u32));
            }
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

pub fn chunks(number: u32) -> Vec<u8> {
    let mut chunks = Vec::new();
    if number == 0 {
        chunks.push(0);
    } else {
        let mut n = number;
        while n > 0 {
            chunks.push((n % 1000) as u8);
            n /= 1000;
        }
    }
    chunks
}
