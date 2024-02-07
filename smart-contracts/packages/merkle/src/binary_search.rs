/*
search search for item in the given first number of elements
within items
*/
pub fn search<T: Ord>(items: &[T], num_first_items: u128, item: &T) -> Option<u128> {
    if num_first_items > items.len() as u128 || num_first_items == 0 {
        return None;
    }

    let mut lo: u128 = 0;
    let mut hi: u128 = num_first_items;
    let mut mid_idx;
    let mut cur_mid_item: &T;

    while hi - lo > 0 {
        mid_idx = lo + (hi - lo) / 2;

        cur_mid_item = &items[mid_idx as usize];

        if *cur_mid_item == *item {
            return Some(mid_idx);
        }

        if *cur_mid_item < *item {
            lo = mid_idx + 1;
        } else {
            hi = mid_idx;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn search_empty() {
        let items = vec![];
        let num_first_items: u128 = 0;
        let item = 0;

        let actual_result = search(&items, num_first_items, &item);

        assert_eq!(false, actual_result.is_some());
    }

    #[test]
    fn search_num_first_items_zero() {
        let items = vec![1, 2, 3];
        let num_first_items: u128 = 0;
        let item = 2;

        let actual_result = search(&items, num_first_items, &item);

        assert_eq!(false, actual_result.is_some());
    }

    #[test]
    fn search_len_less_than_num_first_items() {
        let items = vec![1, 2, 3, 4, 5, 6];
        let num_first_items: u128 = 3;
        let item = 2;

        let expected_result: u128 = 1;

        let actual_result = search(&items, num_first_items, &item);

        assert_eq!(true, actual_result.is_some());
        assert_eq!(expected_result, actual_result.unwrap());
    }

    #[test]
    fn search_one() {
        let items = vec![1];
        let num_first_items: u128 = 1;
        let item = 1;

        let expected_result: u128 = 0;

        let actual_result = search(&items, num_first_items, &item);

        assert_eq!(true, actual_result.is_some());
        assert_eq!(expected_result, actual_result.unwrap());
    }

    #[test]
    fn search_two() {
        let items = vec![1, 2];
        let num_first_items: u128 = 2;
        let item = 2;

        let expected_result: u128 = 1;

        let actual_result = search(&items, num_first_items, &item);

        assert_eq!(true, actual_result.is_some());
        assert_eq!(expected_result, actual_result.unwrap());
    }

    #[test]
    fn search_seven_mid() {
        let items = vec![11, 23, 45, 54, 56, 89, 100];
        let num_first_items: u128 = 7;
        let item = 54;

        let expected_result: u128 = 3;

        let actual_result = search(&items, num_first_items, &item);

        assert_eq!(true, actual_result.is_some());
        assert_eq!(expected_result, actual_result.unwrap());
    }

    #[test]
    fn search_eight() {
        let items = vec![11, 23, 45, 54, 56, 89, 100, 9999];
        let num_first_items: u128 = 8;
        let item = 9999;

        let expected_result: u128 = 7;

        let actual_result = search(&items, num_first_items, &item);

        assert_eq!(true, actual_result.is_some());
        assert_eq!(expected_result, actual_result.unwrap());
    }

    #[test]
    fn search_eight_not_found() {
        let items = vec![11, 23, 45, 54, 56, 89, 100, 9999];
        let num_first_items: u128 = 8;
        let item = 500;

        let actual_result = search(&items, num_first_items, &item);

        assert_eq!(false, actual_result.is_some());
    }

    #[test]
    fn search_seven_when_nine_total() {
        let items = vec![11, 23, 45, 54, 56, 89, 100, 9999, 10000];
        let num_first_items: u128 = 7;
        let item = 54;

        let expected_result: u128 = 3;

        let actual_result = search(&items, num_first_items, &item);

        assert_eq!(true, actual_result.is_some());
        assert_eq!(expected_result, actual_result.unwrap());
    }
}
