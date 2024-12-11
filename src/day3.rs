pub fn run(input: String) -> usize {
    // "mul(", contains 1-3 digit number, ",", contains 1-3 digit number, ")"
    let mut total = 0usize;
    let chars: Vec<char> = input.chars().collect();

    let mut chars_it = chars.iter();
    while let Some(c) = chars_it.next() {
        let mut cloned_chars_it = chars_it.clone();
        match c {
            'm' => {
                match cloned_chars_it.next().unwrap() {
                    'u' => {
                        match cloned_chars_it.next().unwrap() {
                            'l' => match cloned_chars_it.next().unwrap() {
                                '(' => {
                                    let number_it =
                                        cloned_chars_it.clone().take_while(|x| x.is_ascii_digit());
                                    let mut a: Vec<&char> = Vec::new();
                                    for x in number_it {
                                        a.push(x);
                                    }
                                    let a_len = a.len();
                                    if (1..=3).contains(&a_len) {
                                        match cloned_chars_it.next().unwrap() {
                                            ',' => {
                                                let number_it = cloned_chars_it
                                                    .clone()
                                                    .take_while(|x| x.is_ascii_digit());
                                                let mut b: Vec<&char> = Vec::new();
                                                for x in number_it {
                                                    b.push(x);
                                                }
                                                let b_len = b.len();
                                                if (1..=3).contains(&b_len) {
                                                    match cloned_chars_it.next().unwrap() {
                                                        ')' => {
                                                            // MATCH !
                                                            let a = a.into_iter().collect::<String>().parse::<usize>().expect("error converting string to usize");
                                                            let b = b.into_iter().collect::<String>().parse::<usize>().expect("error converting string to usize");
                                                            total += a * b;
                                                        }
                                                        _ => todo!(),
                                                    }
                                                }
                                            }
                                            _ => todo!(),
                                        }
                                    } else {
                                        todo!()
                                    }
                                }
                                _ => todo!(),
                            },
                            _ => todo!(),
                        }
                    }
                    _ => continue,
                }
            }
            _ => continue,
        }
    }

    0
}

fn get_test_string() -> String {
    String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5)")
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(run(get_test_string()), 161)
    }
}
