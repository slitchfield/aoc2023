use regex::Regex;

#[derive(Default)]
pub struct Problem {
    input_raw: String,
    pub log: String,
    pub part_1_answer: Option<i32>,
    pub part_2_answer: Option<i32>,

}

impl Problem {
    pub fn set_input(&mut self, new_input: &String) -> () {
        self.input_raw = new_input.clone();
    }

    pub fn process_as_part_1(&mut self) -> () {
        self.log.push_str("Processing as part 1\n");
        self.log.push_str("Input text: \n");
        self.log.push_str(format!("{}\n", self.input_raw).as_str());

        let mut running_sum: i32 = 0;
        for line in self.input_raw.lines() {
            let mut left_digit: Option<u32> = None;
            let mut right_digit: Option<u32> = None;

            for char in line.chars() {
                if char.is_digit(10) && matches!(left_digit, None) {
                    left_digit = Some(char.to_digit(10)).expect("Could not parse digit"); 
                }
                if char.is_digit(10) {
                    right_digit = Some(char.to_digit(10)).expect("Could not parse digit");
                }
            }
            if matches!(left_digit, Some(u32)) && matches!(right_digit, Some(u32)) {
                self.log.push_str(format!("Found {} and {}\n", left_digit.unwrap(), right_digit.unwrap()).as_str());
                running_sum += (left_digit.unwrap() * 10 + right_digit.unwrap()) as i32;
            }
        }
        self.part_1_answer = Some(running_sum);
        ()
    }
    
    pub fn process_as_part_2(&mut self) -> () {
        self.log.push_str("Processing as part 2\n");
        self.log.push_str("Input text: \n");
        self.log.push_str(format!("{}\n", self.input_raw).as_str());

        let mut running_sum: i32 = 0;
        for line in self.input_raw.lines() {

            let mut matches = vec![];
            for i in 0..line.chars().count() {
                if line[i..].starts_with(char::is_numeric) {
                    matches.push(line.chars().nth(i).unwrap().to_digit(10).unwrap());
                    continue
                }
                if line[i..].starts_with("one") {
                    matches.push(1);
                    continue
                }
                if line[i..].starts_with("two") {
                    matches.push(2);
                    continue
                }
                if line[i..].starts_with("three") {
                    matches.push(3);
                    continue
                }
                if line[i..].starts_with("four") {
                    matches.push(4);
                    continue
                }
                if line[i..].starts_with("five") {
                    matches.push(5);
                    continue
                }
                if line[i..].starts_with("six") {
                    matches.push(6);
                    continue
                }
                if line[i..].starts_with("seven") {
                    matches.push(7);
                    continue
                }
                if line[i..].starts_with("eight") {
                    matches.push(8);
                    continue
                }
                if line[i..].starts_with("nine") {
                    matches.push(9);
                    continue
                }
            }
            self.log.push_str(format!("Line \"{}\" matched on: {:?}\n", line, matches).as_str());
            let left_digit= matches.first();
            let lvalue;
            let right_digit = matches.last();
            let rvalue;

            match left_digit {
                Some(n) => {lvalue = n }
                None => {
                    lvalue = &0;
                    self.log.push_str(format!("WARNING: Could not find left value in \"{}\"", line).as_str());
                }
            }
            match right_digit {
                Some(n) => { rvalue = n }
                None => {
                    rvalue = &0;
                    self.log.push_str(format!("WARNING: Could not find right value in \"{}\"", line).as_str());
                }
            }

            running_sum += (lvalue * 10 + rvalue) as i32;
            self.log.push_str(format!("\tExtracted: {}\n", lvalue * 10 + rvalue).as_str());
        }
        self.log.push_str(format!("Final sum: {}\n", running_sum).as_str());

        self.part_2_answer = Some(running_sum);
        ()
    }
}