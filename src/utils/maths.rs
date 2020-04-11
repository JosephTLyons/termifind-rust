pub fn get_average(data_vec: &[usize]) -> f32 {
    if data_vec.is_empty() {
        return 0.0;
    }

    let sum_of_file_name_lengths: usize = data_vec.iter().sum();
    sum_of_file_name_lengths as f32 / data_vec.len() as f32
}

#[test]
fn get_average_no_elements() {
    assert!((get_average(&[])).abs() < 0.0001);
}

#[test]
fn get_average_1() {
    assert!((get_average(&[11, 100, 21, 34]) - 41.5).abs() < 0.0001);
}

#[test]
fn get_average_2() {
    assert!((get_average(&[3, 0, 49, 2000]) - 513.0).abs() < 0.0001);
}
