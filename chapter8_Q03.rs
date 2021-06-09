// Using a hash map and vectors, create a text interface to allow a user to add employee names to a 
// department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let 
// the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

use std::io;
use std::collections::HashMap;

fn main()
{

    let mut mapw: HashMap<&str,    Vec<&str> > = HashMap::new();

    let mut input_text = String::new();
    println!("Please enter your string: ");
    
    io::stdin()
        .read_line( &mut input_text)
        .expect("Failed to read line");


    let mut text_vec: Vec<&str> = Vec::new();

    for word in input_text.split_whitespace()
    {
        text_vec.push(word);
    }

    match mapw.get_mut(&text_vec[3])
    {
        Some(mapw) => 
        {
            mapw.push(text_vec[1]);
        }
        None => 
        {
            mapw.insert(text_vec[3],   vec![text_vec[1]]);
        }
    }

    println!("{:?}", mapw);

}