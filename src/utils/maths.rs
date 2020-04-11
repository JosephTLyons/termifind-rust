use num::ToPrimitive;

pub fn get_average<T: ToPrimitive>(data_vec: &[T]) -> Result<f32, &'static str> {
    if data_vec.is_empty() {
        return Err("Cannot calculate the average of an empty data set");
    }

    let mut sum = 0.0;

    for value in data_vec {
        match ToPrimitive::to_f32(value) {
            Some(value_f32) => sum += value_f32,
            None => return Err("Had issues casting `T` to `f32`"),
        }
    }

    Ok(sum / data_vec.len() as f32)
}

#[test]
fn get_average_no_elements() {
    let data: [usize; 0] = [];
    assert!(get_average(&data).is_err());
}

#[test]
fn get_average_1() {
    assert!((get_average(&[11, 100, 21, 34]).expect("Oops") - 41.5).abs() < 0.0001);
}

#[test]
fn get_average_2() {
    assert!((get_average(&[3, 0, 49, 2000]).expect("Oops") - 513.0).abs() < 0.0001);
}

#[test]
fn get_average_float_negative() {
    assert!((get_average(&[-54.23, 0.19, -27.87, 2000.0]).expect("Oops") - 479.5225).abs() < 0.0001);
}
