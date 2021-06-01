// Given a list of integers, use a vector and return the mean (the average value), 
// median (when sorted, the value in the middle position), and mode (the value that 
// occurs most often; a hash map will be helpful here) of the list.

// compiler version: rustc 1.52.1 (9bc8c42bb 2021-05-09)


use std::collections::HashMap;

fn vector_mean(v: &mut [i64]) -> f64                        // &[i64] to pass vector as mutable reference
{
    assert!(v.len() != 0, "Vetor can't be empty");
    let vec_len = v.len() as f64;
    let mut sum_element = 0;

    for element in v
    {
        sum_element += *element;
    }

    sum_element as f64 / vec_len
}

fn vector_median(v: &mut [i64]) -> f64
{
    assert!(v.len() != 0, "Vetor can't be empty");
    v.sort();
    let vec_len = v.len();


    if vec_len % 2 != 0
    {
        let median = &v[(vec_len/2 ) as usize];
        *median as f64 
    }
    else
    {
        let middle1: &i64= &v[(vec_len/2) as usize];

        let middle2: &i64 = &v[(vec_len/2) as usize -1 ];

        let med: f64  = (middle1 + middle2) as f64;

        med/2.0
    }
}

fn vector_mode(vec: Vec<i64>) -> i64
{
    let mut map = HashMap::new();
    for number in vec.iter()
    {
        let count = map.entry(number).or_insert(0);
        *count += 1;
    }

    let mut max_count: i64 = 0;
    let mut max_keyy: i64 = 0;

    for (key, value) in map.into_iter()
    {
        //println!("{} {}", key, value);
        if max_count < value
        {
            max_count = value;
            //println!("{}", max_count);
            max_keyy = *key;
        }
        //println!("maximum key {}, apears {} times", max_keyy, max_count);
    }

    max_keyy
}


fn main()
{
    let mut vec: Vec<i64> = [1,2,3,4,5,5,5,6,7,7,7,7,7].to_vec();
    println!("Mean of the vector from function: {}", vector_mean(&mut vec));
    println!("Median of the vector from function: {}", vector_median(&mut vec));
    println!("Mode of the vector from function: {}", vector_mode(vec.clone()));
    //println!("{:?}", map);
}