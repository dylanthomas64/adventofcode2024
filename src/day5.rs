use std::collections::HashMap;




pub fn run(input: String) -> usize {
    let mut total: usize = 0;
    let mut order_map = HashMap::<u8, Vec<u8>>::new();
    let mut it = input.split("\n\n");
    let rules = it.next().unwrap();
    let updates = it.next().unwrap();
    for line in rules.lines() {
        let mut rule= line.split("|").map(|s| s.parse::<u8>().expect("error parsing string"));
        let value = rule.next().unwrap();
        let key = rule.next().unwrap();
        if order_map.contains_key(&key) {
            order_map.entry(key).and_modify(|v| v.push(value));
        } else {
            order_map.insert(key, vec![value]);
        }
    }


    // for each number, if values of key match any of the next numbers it's wrong
    for line in updates.lines() {
        println!("{line}");
        let mut report_it = line.split(",").map(|c| c.parse::<u8>().expect("couldn;t parse as number"));
        let report: Vec<u8> = report_it.clone().collect();

        let mut success: bool = true;

        while let Some(c) = report_it.next() {
            if let Some(v) = order_map.get(&c) {
                // for each value entry check if it exists somewhere in the remainder of to sequence iterator
                for i in v {
                    if let Some(_) = report_it.clone().find(|item| item == i) {
                        success = false;
                    }
                }
            }
        }
        if success {
            let middle = report.len()/2;
            total += report[middle] as usize;
        }

    }
    total
}





#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(143, run(get_test_input()))
    }
}

fn get_test_input() -> String {
    String::from("47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47")
}