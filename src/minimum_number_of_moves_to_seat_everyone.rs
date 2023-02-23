struct Solution {}

impl Solution {
    pub fn min_moves_to_seat(seats: Vec<i32>, students: Vec<i32>) -> i32 {
        let mut seats_sorted = seats;
        seats_sorted.sort_unstable();
        let mut students_sorted = students;
        students_sorted.sort_unstable();

        let mut count = 0;

        for i in 0..seats_sorted.len() {
            count += (students_sorted.get(i).unwrap() - seats_sorted.get(i).unwrap()).abs()
        }
        count
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::min_moves_to_seat(vec![3, 1, 5], vec![2, 7, 4]), 4);
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::min_moves_to_seat(vec![4, 1, 5, 9], vec![1, 3, 2, 6]),
            7
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::min_moves_to_seat(vec![2, 2, 6, 6], vec![1, 3, 2, 6]),
            4
        );
    }
}
