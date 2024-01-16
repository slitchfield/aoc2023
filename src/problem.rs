use regex::Regex;

#[derive(Default)]
pub struct Problem {
    input_raw: String,
    pub log: String,
    pub part_1_answer: Option<i32>,
    pub part_2_answer: Option<i32>,
    games: Vec<Game>,
}

#[derive(Default)]
pub struct Game {
    index: u32,
    sets: Vec<CubeSet>,
}

#[derive(Default, Debug)]
pub struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl Problem {
    pub fn set_input(&mut self, new_input: &String) -> () {
        self.input_raw = new_input.clone();
    }

    pub fn parse_set(set_desc: &str) -> CubeSet {

        let mut retset: CubeSet = CubeSet::default();
        let splits: Vec<&str> = set_desc.split(", ").collect();
        if splits.len() > 3 {
            println!("Got more entries than RGB would allow!");
        }
        for item in splits {
            let item_split: Vec<&str> = item.split(" ").collect();
            let value = item_split.first().unwrap().parse::<u32>().unwrap();
            let color = item_split.last().unwrap();
            if color.ends_with("red") {
                retset.red = value;
                continue;
            }
            if color.ends_with("green") {
                retset.green = value;
                continue;
            }
            if color.ends_with("blue") {
                retset.blue = value;
                continue;
            }
        }
        retset
    }

    pub fn parse_game(line: &str) -> Game {
        // Split about Game X: 
        println!("Parsing: {}", line);
        let split = line.split(": ");
        let clauses: Vec<&str> = split.collect();
        let game_clause = clauses.first().unwrap();
        let sets_clause = clauses.last().unwrap();

        let split = game_clause.split(" ");
        let clauses: Vec<&str> = split.collect();
        let index = clauses.last().unwrap().parse::<u32>().unwrap();

        let split = sets_clause.split("; ");
        let sets: Vec<&str> = split.collect();
        println!("Found index: {}", index);
        println!("\tFound sets: {:?}", sets);

        let parsed_sets: Vec<CubeSet> = sets.into_iter().map(Self::parse_set).collect();
        println!("\tParsed sets as: {:?}", parsed_sets);
        Game {
            index,
            sets: parsed_sets
        }
    }

    pub fn is_game_possible(game: &Game) -> bool {
        let mut retbool = true;
        for set in &game.sets { 
            if set.red > 12 {
                println!("\tToo many reds; Invalid!");
                retbool = false;
            } 
            if set.green > 13 {
                println!("\tToo many greens; Invalid!");
                retbool = false;
            }
            if set.blue > 14 {
                println!("\tToo many blues; Invalid!");
                retbool = false;
            }
        }
        retbool
    }

    pub fn process_as_part_1(&mut self) -> () {
        self.log.push_str("Processing as part 1\n");
        self.log.push_str("Input text: \n");
        self.log.push_str(format!("{}\n", self.input_raw).as_str());

        self.games.clear();

        for line in self.input_raw.lines() {
            self.log.push_str(format!("Processing line: \"{}\"\n", line).as_str());
            let line_game = Problem::parse_game(line);
            self.games.push(line_game);
        }

        let mut id_sums = 0u32;

        for game in &self.games {
            println!("Processing Game {}: {:?}", game.index, game.sets);
            if Self::is_game_possible(game) {
                println!("\tGame is valid!");
                id_sums += game.index;
            }
        }
        self.part_1_answer = Some(id_sums as i32);
        ()
    }

    pub fn calculate_game_power(game: &Game) -> u32 {
        let mut max_r = 0;
        let mut max_g = 0;
        let mut max_b = 0;
        
        for set in &game.sets { 
            if set.red > max_r {
                max_r = set.red;
            } 
            if set.green > max_g {
                max_g = set.green;
            }
            if set.blue > max_b {
                max_b = set.blue
            }
        }

        max_r * max_g * max_b
    }

    pub fn process_as_part_2(&mut self) -> () {
        self.log.push_str("Processing as part 2\n");
        self.log.push_str("Input text: \n");
        self.log.push_str(format!("{}\n", self.input_raw).as_str());
        
        self.games.clear();

        for line in self.input_raw.lines() {
            self.log.push_str(format!("Processing line: \"{}\"\n", line).as_str());
            let line_game = Problem::parse_game(line);
            self.games.push(line_game);
        }

        let mut running_sum = 0u32;

        for game in &self.games {
            println!("Processing Game {}: {:?}", game.index, game.sets);
            running_sum += Self::calculate_game_power(game);
        }

        self.part_2_answer = Some(running_sum as i32);
        ()
    }
}