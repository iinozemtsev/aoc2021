use std::result::Result;
use std::str::FromStr;

struct Input {
    numbers: Vec<usize>,
    boards: Vec<Board>,
}

impl Input {
    fn any_wins(&self) -> bool {
        self.boards.iter().any(|board| board.is_winner())
    }

    fn mark(&mut self, num: usize) {
        for board in &mut self.boards {
            board.mark(num)
        }
    }
}
impl FromStr for Input {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, String> {
        let mut result = Input {
            numbers: Vec::new(),
            boards: Vec::new(),
        };
        let mut parts = input.split("\n\n");
        let nums_str = parts.next().ok_or(format!("Empty input"))?;
        for num_str in nums_str.split(",") {
            result
                .numbers
                .push(num_str.parse::<usize>().map_err(|e| e.to_string())?)
        }

        for board_str in parts {
            result.boards.push(board_str.parse::<Board>()?);
        }

        return Ok(result);
    }
}
struct Board {
    cols: usize,
    rows: usize,
    numbers: Vec<BoardNumber>,
}

impl FromStr for Board {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, String> {
        let mut result = Board {
            cols: 0,
            rows: 0,
            numbers: Vec::new(),
        };

        for row_str in text.lines() {
            result.rows += 1;
            result.cols = 0;
            for num_str in row_str.split_whitespace() {
                result.cols += 1;
                result.numbers.push(BoardNumber {
                    is_marked: false,
                    value: num_str.parse::<usize>().map_err(|e| e.to_string())?,
                });
            }
        }

        return Ok(result);
    }
}

pub fn part1(text: &str) -> usize {
    let mut input = text.parse::<Input>().unwrap();
    let len = input.numbers.len();
    for i in 0..len {
        let number = input.numbers[i];
        input.mark(number);

        if input.any_wins() {
            let sum = input
                .boards
                .iter()
                .find(|b| b.is_winner())
                .unwrap()
                .unmarked_sum();
            println!("sum: {}, number: {}", sum, number);
            return sum * number;
        }
    }
    return 0;
}

pub fn part2(text: &str) -> usize {
    let mut input = text.parse::<Input>().unwrap();
    let len = input.numbers.len();
    let mut remaining_boards: Vec<Board> = Vec::new();
    let mut winned_boards: Vec<Board> = Vec::new();
    remaining_boards.append(&mut input.boards);
    let mut last_number = 0;
    for i in 0..len {
        if remaining_boards.len() == 0 {
            break;
        }
        last_number = input.numbers[i];
        for board in &mut remaining_boards {
            board.mark(last_number)
        }

        let mut j = 0;
        while j < remaining_boards.len() {
            if remaining_boards[j].is_winner() {
                winned_boards.push(remaining_boards.remove(j));
            } else {
                j += 1;
            }
        }
    }
    return winned_boards.last().unwrap().unmarked_sum() * last_number;
}

impl Board {
    fn mark(&mut self, num: usize) {
        for mut number in &mut self.numbers {
            if number.value == num {
                number.is_marked = true
            }
        }
    }

    fn get_number(&self, row: usize, col: usize) -> &BoardNumber {
        &self.numbers[row * self.cols + col]
    }

    fn is_row_full(&self, row: usize) -> bool {
        for col in 0..self.cols {
            if !self.get_number(row, col).is_marked {
                return false;
            }
        }
        return true;
    }

    fn is_col_full(&self, col: usize) -> bool {
        for row in 0..self.rows {
            if !self.get_number(row, col).is_marked {
                return false;
            }
        }
        return true;
    }

    fn is_winner(&self) -> bool {
        for row in 0..self.rows {
            if self.is_row_full(row) {
                return true;
            }
        }

        for col in 0..self.cols {
            if self.is_col_full(col) {
                return true;
            }
        }
        return false;
    }

    fn unmarked_sum(&self) -> usize {
        let mut sum = 0usize;
        for num in &self.numbers {
            if !num.is_marked {
                sum += num.value
            }
        }
        return sum;
    }
}

struct BoardNumber {
    value: usize,
    is_marked: bool,
}
#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        println!("part1 sample: {}", super::part1(include_str!("sample.txt")));
        println!("part1: {}", super::part1(include_str!("input.txt")));
    }

    #[test]
    fn part2() {
        println!("part2 sample: {}", super::part2(include_str!("sample.txt")));
        println!("part2: {}", super::part2(include_str!("input.txt")));
    }
}
