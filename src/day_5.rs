use crate::util;

const ROW_COUNT: i32 = 128;
const COL_COUNT: i32 = 8;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct SeatId {
    row: i32,
    col: i32,
    id: i32,
}

pub fn read_input() -> Vec<String> {
    util::file_to_vec("day_5_input.txt")
}

fn get_row(input: &str) -> (i32, i32) {
    let (mut low, mut high) = (0, ROW_COUNT - 1);

    for cmd in input.chars() {
        let step = ((high - low) as f32) / 2.0;
        match cmd {
            'F' => high = low + step.floor() as i32,
            'B' => low = low + step.ceil() as i32,
            _ => panic!("Doh!")
        }
    }
    (low, high)
}

fn get_col(input: &str) -> (i32, i32) {
    let (mut left, mut right) = (0, COL_COUNT - 1);

    for cmd in input.chars() {
        let step = ((right - left) as f32) / 2.0;
        match cmd {
            'L' => right = left + step.floor() as i32,
            'R' => left = left + step.ceil() as i32,
            _ => panic!("Doh!")
        }
    }
    (left, right)
}

fn get_seat_id(input: &str) -> SeatId {
    let vertical_input: String = input.chars()
        .take_while(|&x| x == 'F' || x == 'B')
        .collect();
    let horizontal_input: String = input.chars()
        .filter(|&x| x == 'L' || x == 'R')
        .collect();
    let (row, _) = get_row(&vertical_input);
    let (col, _) = get_col(&horizontal_input);
    SeatId { row, col, id: get_id(row, col) }
}

fn get_id(row: i32, col: i32) -> i32 {(row * 8) + col}

pub fn max_seat_id(input: &Vec<String>) -> SeatId {
    let ids: Vec<SeatId> = input.iter()
        .map(|x| get_seat_id(x))
        .collect();
    let max = ids.iter()
        .max_by(|x,y| x.id.cmp(&y.id))
        .unwrap();
    max.clone()
}

pub fn get_missing_seat(input: &Vec<String>) -> SeatId {
    let mut seats: Vec<SeatId> = input.iter()
        .map(|x| get_seat_id(x))
        .collect();
    seats.sort_by(|x, y| x.id.cmp(&y.id));

    let first_row = seats.first().unwrap().row;
    let last_row = seats.last().unwrap().row;

    let mut expected = SeatId { row: first_row + 1, col: 0, id: get_id(first_row + 1, 0) };
    for seat in seats.iter().skip(8) { // skip te first row
        if seat.row == last_row {
            panic!("Doh!")
        }

        if *seat != expected {
            return expected;
        }

        let mut next_col = expected.col + 1;
        let mut next_row = expected.row;
        if expected.col == COL_COUNT - 1 { // drop to the next row
            next_col = 0;
            next_row = expected.row + 1
        }
        expected = SeatId { row: next_row, col: next_col, id: get_id(next_row, next_col) };
    }

    panic!("Didn't find missing seat")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_1() {
        let low_high = get_row("F");
        assert_eq!((0, 63), low_high);
    }

    #[test]
    fn step_2() {
        let low_high = get_row("FB");
        assert_eq!((32, 63), low_high);
    }

    #[test]
    fn step_3() {
        let low_high = get_row("FBF");
        assert_eq!((32, 47), low_high);
    }

    #[test]
    fn step_4() {
        let low_high = get_row("FBFB");
        assert_eq!((40, 47), low_high);
    }

    #[test]
    fn step_5() {
        let low_high = get_row("FBFBB");
        assert_eq!((44, 47), low_high);
    }

    #[test]
    fn step_6() {
        let low_high = get_row("FBFBBF");
        assert_eq!((44, 45), low_high);
    }

    #[test]
    fn step_7() {
        let low_high = get_row("FBFBBFF");
        assert_eq!((44, 44), low_high);
    }

    #[test]
    fn get_col_1() {
        let left_right = get_col("R");
        assert_eq!((4, 7), left_right);
    }

    #[test]
    fn get_col_2() {
        let left_right = get_col("RL");
        assert_eq!((4, 5), left_right);
    }

    #[test]
    fn get_col_3() {
        let left_right = get_col("RLR");
        assert_eq!((5, 5), left_right);
    }

    #[test]
    fn get_seat_id_1() {
        let expected = SeatId {row:70, col: 7, id: 567 };
        let seat = get_seat_id("BFFFBBFRRR");
        assert_eq!(expected, seat);
    }

    #[test]
    fn get_seat_id_2() {
        let expected = SeatId {row:14, col: 7, id: 119 };
        let seat = get_seat_id("FFFBBBFRRR");
        assert_eq!(expected, seat);
    }

    #[test]
    fn get_seat_id_3() {
        let expected = SeatId {row:102, col: 4, id: 820 };
        let seat = get_seat_id("BBFFBBFRLL");
        assert_eq!(expected, seat);
    }
}