pub fn get_outliers(mut data_vec: Vec<usize>, is_sorted: bool) -> Option<(Vec<usize>, Vec<usize>)> {
    if data_vec.is_empty() {
        return None;
    }

    if !is_sorted {
        data_vec.sort();
    }

    let (q1_value, _, q3_value) = get_q1_q2_q3_values(&data_vec);
    let interquartile_range = q3_value - q1_value;

    let intermediate_value = 1.5 * interquartile_range;
    let lower_range = q1_value - intermediate_value;
    let upper_range = q3_value + intermediate_value;

    let mut lower_outliers: Vec<usize> = Vec::new();
    let mut upper_outliers: Vec<usize> = Vec::new();

    for data in data_vec {
        if (data as f32) < lower_range {
            lower_outliers.push(data);
        } else if (data as f32) > upper_range {
            upper_outliers.push(data);
        }
    }

    Some((lower_outliers, upper_outliers))
}

#[test]
fn get_outliers_empty_data_set() {
    assert_eq!(get_outliers([].to_vec(), true), None);
}

#[test]
fn get_outliers_none() {
    assert_eq!(
        get_outliers([1, 2, 4, 10].to_vec(), true),
        Some(([].to_vec(), [].to_vec()))
    );
}

#[test]
fn get_outliers_some_1() {
    assert_eq!(
        get_outliers(
            [10, 12, 11, 15, 11, 14, 13, 17, 12, 22, 14, 11].to_vec(),
            false
        ),
        Some(([].to_vec(), [22].to_vec()))
    );
}

#[test]
fn get_outliers_some_2() {
    assert_eq!(
        get_outliers(
            [0, 3, 3, 3, 11, 12, 13, 15, 19, 20, 29, 40, 79].to_vec(),
            false
        ),
        Some(([].to_vec(), [79].to_vec()))
    );
}

fn get_q1_q2_q3_values(data_vec: &[usize]) -> (f32, f32, f32) {
    let data_vec_length = data_vec.len();
    let mut halfway = data_vec_length / 2;

    let q1_value = get_median(&data_vec[0..halfway]);
    let q2_value = get_median(&data_vec);

    if data_vec_length % 2 != 0 {
        halfway += 1;
    }

    let q3_value = get_median(&data_vec[halfway..data_vec_length]);

    (q1_value, q2_value, q3_value)
}

#[test]
fn get_q1_q2_q3_values_even_set_even_halves() {
    assert_eq!(
        get_q1_q2_q3_values(&[1, 2, 3, 4, 5, 6, 7, 8]),
        (2.5, 4.5, 6.5)
    );
}

#[test]
fn get_q1_q2_q3_values_even_set_odd_halves() {
    assert_eq!(
        get_q1_q2_q3_values(&[1, 2, 3, 4, 5, 6]),
        (2.0, 3.5, 5.0)
    );
}

#[test]
fn get_q1_q2_q3_values_odd_set_odd_halves() {
    assert_eq!(
        get_q1_q2_q3_values(&[1, 2, 3, 4, 5, 6, 7, 8, 9]),
        (2.5, 5.0, 7.5)
    );
}

fn get_median(data_vec: &[usize]) -> f32 {
    let data_vec_length = data_vec.len();
    let half_way = data_vec_length / 2;

    if data_vec.len() % 2 == 0 {
        return (data_vec[half_way - 1] as f32 + data_vec[half_way] as f32) / 2.0;
    }

    data_vec[half_way] as f32
}

#[test]
fn get_median_even_set() {
    assert!((get_median(&[1, 2, 3, 4, 5, 6]) - 3.5).abs() < 0.0001);
}

#[test]
fn get_median_odd_set() {
    assert!((get_median(&[1, 2, 3, 4, 5]) - 3.0).abs() < 0.0001);
}

#[test]
fn get_median_random_numbers_even_set() {
    assert!((get_median(&[1, 11, 34, 66, 209, 353, 1067, 10_453]) - 137.5).abs() < 0.0001);
}

#[test]
fn get_median_random_numbers_odd_set() {
    assert!((get_median(&[1, 23, 24, 45, 200, 343, 1001]) - 45.0).abs() < 0.0001);
}
