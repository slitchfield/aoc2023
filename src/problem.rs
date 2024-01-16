use regex::Regex;

#[derive(Default)]
pub struct Problem {
    input_raw: String,
    pub log: String,
    pub part_1_answer: Option<i32>,
    pub part_2_answer: Option<i32>,
    pub board: Board,
}

#[derive(Default)]
pub struct Board {
    pub part_numbers: Vec<PartNumber>,
    pub symbols: Vec<Symbol>,
}

#[derive(Default)]
pub struct PartNumber {
    number: u32,
    row: usize,
    span: (usize, usize)
}

#[derive(Default, Debug)]
pub struct Symbol {
    symbol: char,
    row: usize,
    col: usize 
}

impl Problem {
    pub fn set_input(&mut self, new_input: &String) -> () {
        self.input_raw = new_input.clone();
    }

    pub fn parse_board(&mut self, input: String) -> Board {

        let mut symbol_vec: Vec<Symbol> = vec![];
        let mut part_vec: Vec<PartNumber> = vec![];

        pub enum parse_state {
            IN_NUMBER,
            IN_WS,
        };
        let mut cur_state: parse_state = parse_state::IN_WS;
        for (row, line) in input.lines().enumerate() {
            self.log.push_str(format!("Processing line {}: \"{}\"\n", row, line).as_str());

            let mut cur_num: String = String::new();
            let mut cur_num_start: usize = 0;
            let mut cur_num_end: usize = 0;

            for (col, chr) in line.chars().enumerate() {
                let next_state: parse_state;
                match cur_state {
                    parse_state::IN_WS => {
                        if chr.is_numeric() {
                            cur_num.push(chr);
                            cur_num_start = col;
                            
                            next_state = parse_state::IN_NUMBER;
                        }
                        else if chr == '.' {
                            // Do nothing
                            next_state = parse_state::IN_WS;
                        }
                        else {
                            // Push symbol
                            self.log.push_str(format!("Found symbol ({}, {}): {}\n", row, col, chr).as_str());
                            symbol_vec.push(Symbol { symbol: chr, row, col });

                            next_state = parse_state::IN_WS;
                        }
                    }, 
                    parse_state::IN_NUMBER => {
                        if chr.is_numeric() {
                            cur_num.push(chr);

                            next_state = parse_state::IN_NUMBER;
                        }
                        else {
                            println!("Exiting a number on line {}", row);
                            // Just finished a number! Parse value, construct object, clean up
                            let cur_num_value = cur_num.parse::<u32>().unwrap();
                            cur_num_end = col - 1;

                            self.log.push_str(format!("Found complete number ({}, ({}, {}) ): {}\n", row, cur_num_start, cur_num_end, cur_num_value).as_str());
                            part_vec.push(PartNumber { number: cur_num_value, row, span: (cur_num_start, cur_num_end) });

                            cur_num.clear();
                            cur_num_start = 0;
                            cur_num_end = 0;

                            // Now we switch on whether this was ws or not 
                            if chr != '.' {
                                // Not ws, push a symbol!
                                self.log.push_str(format!("Found symbol ({}, {}): {}\n", row, col, chr).as_str());
                                symbol_vec.push(Symbol { symbol: chr, row, col });
                            }

                            next_state = parse_state::IN_WS;
                            
                        } 

                    },
                }
                cur_state = next_state;
            }
            // If we ended in IN_NUMBER, finish off the number at the end of the line
            if matches!(cur_state, parse_state::IN_NUMBER) {
                let cur_num_value = cur_num.parse::<u32>().unwrap();
                cur_num_end = line.len() - 1;

                self.log.push_str(format!("Found complete number ({}, ({}, {}) ): {}\n", row, cur_num_start, cur_num_end, cur_num_value).as_str());
                part_vec.push(PartNumber { number: cur_num_value, row, span: (cur_num_start, cur_num_end) });

                cur_num.clear();
                cur_num_start = 0;
                cur_num_end = 0;
            }

            // Prep the parse state for starting a new row.
            cur_state = parse_state::IN_WS;

        }
        Board { part_numbers: part_vec, symbols: symbol_vec }
    }

    pub fn is_adjacent(part: &PartNumber, symbol: &Symbol) -> bool {
        let part_row = part.row;
        let (start_col, end_col) = part.span;
        if (symbol.row >= part_row.saturating_sub(1)) && (symbol.row <= part_row.saturating_add(1)) {

            // Is this symbol within the column span of the number +/- 1?
            if (symbol.col >= start_col.saturating_sub(1)) && (symbol.col <= end_col.saturating_add(1)) {

                return true
            }
        }
        false
    }
    pub fn process_as_part_1(&mut self) -> () {
        self.log.push_str("Processing as part 1\n");
        self.log.push_str("Input text: \n");
        self.log.push_str(format!("{}\n", self.input_raw).as_str());

        self.board.part_numbers.clear();
        self.board.symbols.clear();

        self.board = self.parse_board(self.input_raw.clone());

        // Iterate through part numbers, check if each one has a symbol +/- 1 coord
        let mut running_sum: u32 = 0u32;

        for part in &self.board.part_numbers {
            for symbol in &self.board.symbols {
                if Self::is_adjacent(part, symbol) {
                    running_sum += part.number;
                }
            }
        }
        self.part_1_answer = Some(running_sum as i32);
        ()
    }

    pub fn process_as_part_2(&mut self) -> () {
        self.log.push_str("Processing as part 2\n");
        self.log.push_str("Input text: \n");
        self.log.push_str(format!("{}\n", self.input_raw).as_str());
        
        self.board.part_numbers.clear();
        self.board.symbols.clear();

        self.board = self.parse_board(self.input_raw.clone());

        let mut running_sum: i32 = 0i32;
        // Iterate through part numbers, check if each one has a symbol +/- 1 coord
        for symbol in &self.board.symbols {
            let mut adjacent_parts: Vec<&PartNumber> = vec![];

            for part in &self.board.part_numbers {
                if Self::is_adjacent(part, symbol) {
                    adjacent_parts.push(part);
                }
            }

            if adjacent_parts.len() == 2 {
                self.log.push_str(format!("Found gear!\n").as_str());
                let gear_value = adjacent_parts.iter().fold(1, |res, part| res * part.number);
                self.log.push_str(format!("\tValue: {}\n", gear_value).as_str());
                running_sum += gear_value as i32;
            }
        }

        self.part_2_answer = Some(running_sum);
        ()
    }
}