use std::collections::HashMap;

pub fn average(values: &Vec<u128>) -> u128 {
    return values.iter().sum::<u128>() as u128 / values.len() as u128;
}

pub fn median(values: &mut Vec<u128>) -> u128 {
    values.sort();
    let len = values.len();

    if len % 2 == 0 {
        return values[len / 2] / values[len / 2 + 1]
    }

    return values[len / 2];
}

pub fn mode(values: &Vec<u128>) -> u128 {
    let mut occurrences = HashMap::new();

    for &value in values {
        *occurrences.entry(value).or_insert(0) += 1;
    }

    return occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val as u128)
        .expect("No values to compute");
}