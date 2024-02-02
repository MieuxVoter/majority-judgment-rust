use std::collections::BTreeMap;


fn main() {

    // This is two options did not work
    let mut poll_data : BTreeMap<String, Vec<i32> > = BTreeMap::new();
    // let poll_data = HashMap::new();
    // let mut poll_data = HashMap::new();
    poll_data.insert("Pizza".to_string(), vec![0, 0, 3, 0, 2, 0, 3, 1, 2, 3]);
    poll_data.insert("Chips".to_string(), vec![0, 1, 0, 2, 1, 2, 2, 3, 2, 3]);
    poll_data.insert("Pasta".to_string(), vec![0, 1, 0, 1, 2, 1, 3, 2, 3, 3]);
    poll_data.insert("Bread".to_string(), vec![0, 1, 2, 1, 1, 2, 1, 2, 2, 3]);

    println!("{:?}", poll_data);
    let _ = check_poll_length(&poll_data);

    // poll_data.insert("Salad".to_string(), vec![0, 1, 2, 1]);
    // let the_error = check_poll_length(&poll_data);
    // println!("{:?}", the_error);

    let vote: Vec<i32> = vec![0, 1, 2, 1, 1, 2, 1, 2, 2, 3];
    println!(
        "{:?}",
        compute_frequency_of_grades(
            vote)
    );

    majority_judgment(&poll_data);
}

/// Function that checks that all the lengths of the polls are the same otherwise it throws an error
/// # Arguments
/// * `poll_data`: a BTreeMap with the poll data
///
/// # Returns
/// * `Result<(), &str>`: an empty result or an error message
///
/// # Example (no panic)
/// ```
/// let mut poll_data = BTreeMap::new();
/// poll_data.insert("Pizza", vec![0, 0, 3, 0, 2, 0, 3, 1, 2, 3]);
/// poll_data.insert("Chips", vec![0, 1, 0, 2, 1, 2, 2, 3, 2, 3]);
/// check_poll_length(&poll_data);
/// ```
///
/// # Example (panic)
/// ```
/// let mut poll_data = BTreeMap::new();
/// poll_data.insert("Pizza", vec![0, 2, 3]);
/// poll_data.insert("Chips", vec![0, 3, 2, 3, 4]);
/// check_poll_length(&poll_data);
/// ```
///
fn check_poll_length(poll_data: &BTreeMap<String, Vec<i32>>) -> Result<(), &str> {
    let first_poll_length = poll_data.values().next().unwrap().len();
    for poll in poll_data.values() {
        if poll.len() != first_poll_length {
            panic!("The polls have different lengths!")
        }
    }
    Ok(())

}

/// Function that calculates the majority judgment of a poll
/// # Arguments
/// * `poll_data`: a BTreeMap<String, Vec<i32>> with the poll data
///
/// # Returns
/// * `BTreeMap<String, i32>`: a BTreeMap with ranked items/candidates and their ranking
fn majority_judgment(poll_data: &BTreeMap<String, Vec<i32>>) -> BTreeMap<&String, u32> {

    let _ = check_poll_length(&poll_data);

    let mut majority_values = BTreeMap::new();
    for (item, grades) in poll_data {
        // majority_values.insert(item, compute_frequency_of_grades(grades.to_vec()));
        majority_values.insert(item, compute_majority_values(grades.to_vec()));
    }
    println!("{:?}", majority_values);
    // keys
    // ranked_scores = ranking_scores(&majority_values);
    //
    // return ranked_scores
    return majority_values
}

fn compute_majority_values(grades: Vec<i32>) -> u32 {
    let mut count_number_of_grades: BTreeMap<i32, i32> = BTreeMap::new();
    let tally = compute_frequency_of_grades(grades.clone());
    println!("{:?}", tally);
    let keys = tally.keys();
    let values = tally.values();

    let total_votes = grades.len() as u32;

    let mut total : i32 = 0;
    // let mut cumsum : i32 = 0;
    let mut idx : u32 = 0;

    for _ in (0..total_votes) {
        total = values.clone().sum();
        let mut total_f32 = total as f32;

        let values_f32: Vec<f32> = values.clone().into_iter().map(|x| *x as f32).collect();
        let cumsum: Vec<f32> = values_f32.clone().into_iter().scan(0.0, |sum, val| {
            *sum += val / total_f32;
            Some(*sum)
        }).collect();

        idx = median_grade(cumsum);
    }

    return idx

}

/// Function that compute the frequency of each grade in BTreeMap structure
///
/// # Arguments
/// * `grades`:  Vec<i32> unsorted numbers representing the grades
///
/// # Returns
/// * BTreeMap<i32, i32>, first is the grade, the second is the number of time, it has been given
///
fn compute_frequency_of_grades(mut grades: Vec<i32>) -> BTreeMap<i32, i32> {
    let mut tally: BTreeMap<i32, i32> = BTreeMap::new();

    grades.sort();
    let grades_group = group_by(grades);

    for grades in grades_group.iter() {
        tally.insert( grades[0]
            , grades.len().try_into().unwrap());
    }
    return tally
}
/// Function that group the sorted vector in to a vector of sub vectors
/// I couldn't replicate the group_by function of python, so I reimplemented an equivalent
///
/// # Arguments
/// * `vector`:  Vec<T> of sorted number
///
/// # Returns
/// * Vec<Vec<T>> vector of vectors, each vector contains the number
///
fn group_by<T: PartialEq + Clone>(vector: Vec<T>) -> Vec<Vec<T>> {
    let mut result: Vec<Vec<T>> = Vec::new();

    for item in vector.iter() {
        if let Some(group) = result.last_mut() {
            if group[0] == *item {
                group.push(item.clone());
                continue;
            }
        }

        result.push(vec![item.clone()]);
    }

    result
}

/// Evaluate the median grade from a cumulative sum of grades
/// # Arguments
/// * `cumsum_vec`:  Vec<f32> of cumulative sum of grades
///
/// # Returns
/// * u32, the index of the median grade
fn median_grade(cumsum_vec: Vec<f32>) -> u32 {
    // verify the last element is a 1
    if cumsum_vec.last() != Some(&1.0) {
        panic!("The last element of the cumulative sum vector is not 1.0. \
        Please normalize the vector before calling the function fn median_grade.")
    }
    // verify all element are positive
    if cumsum_vec.iter().any(|&x| x < 0.0) {
        panic!("The cumulative sum vector contains negative values. \
        Please make sure all values are positive before calling the function fn median_grade.")
    }

    for (idx, &val) in cumsum_vec.iter().enumerate() {
        if val >= 0.5 {
            return idx.try_into().unwrap()
        }
    }
    return cumsum_vec.len() as u32 - 1u32
}
// fn main() {
//     let numbers = vec![0, 0, 0, 1, 1, 2, 22];
//
//     let grouped = group_consecutive_equals(numbers);
//
//     for group in grouped {
//         println!("{:?}", group);
//     }
// }