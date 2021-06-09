
// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” 
// is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead 
// (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

fn is_consonant(w: & str) -> bool 
{
   let vowel: String = "AEIOUaeiou".to_string();
   
   for ch in vowel.chars()
   {
       if w.starts_with(ch)
       {
           return false;
       }
   }
   return true;
}

fn main()
{
    let s = String::from("apple first i am who first");
    let mut vec: Vec<String> = Vec::new();

    for word in s.split_whitespace()
    {
        if !is_consonant(word)
        {
            let result = format!("{}{}",word,"-hay");
            vec.push(result);
        }    
        //vec.push(word); 
        else  
        {
            //let mut first_c = word.chars().nth(0);

            let (first, last) = word.split_at(1);
            let result = format!("{}{}",first,"ay");
            //let (first, last) = word.split_at(0);
            
            let result_final = format!("{}{}{}",last,"-",result);
            vec.push(result_final);
        }
    }

    let mut final_string: String = String::new();

    for word in vec.iter()
    {
        //println!("{}", word);
        final_string = format!("{} {}", final_string, word);
    }

    println!("{}",final_string);
    

}