
// compiler version: rustc 1.52.1

//Warning: not happy about this code.

// Lyric: https://www.google.com/search?q=The+Twelve+Days+of+Christmas&oq=The+Twelve+Days+of+Christmas&aqs=chrome..69i57j69i59&sourceid=chrome&ie=UTF-8

fn main()
{
    let array: [&str; 13] = [" of Christmas my true love gave to me",
    "partridge in a pear tree", "turtle doves", "French hens", "calling birds",
    "gold rings", "geese a laying", "swans a swimming","maids a milking", "ladies dancing",
    "lords a leaping", "pipers piping", "drummers drumming" ];


    for n in 0..14
    {
        if n == 0
        {
            print!("On the first day{}\nA {}\n \n", &array[n], &array[n+1]);
        }
        else if n ==1 
        {
            print!("On the second day{}\nTwo {} and a {}\n \n", &array[0], &array[n+1], &array[n]);

        }
        else if n == 2
        {
            print!("On the third day{}\nThree {}, two {} and a {}\n \n", 
            &array[0], &array[n+1], &array[n], &array[1] );

        }
        else if n == 3 
        {
            print!("On the fourth day{}\nFour {}, three {}\nTwo {} and a {}\n \n", 
            &array[0], &array[n+1],&array[n],&array[2],&array[1]);
        }
        else if n == 4
        {
            print!("On the fifth day{}\nFive {}, four {}, three {}\nTwo {} and a {}\n\n",
        &array[0], &array[n+1], &array[n], &array[n-1], &array[n-2], &array[n-3]);
        }
        else if n == 5
        {
            print!("On the sixth day{}\nSix {}, five {},\nFour {}, three {}\nTwo {} and a {}\n\n",
            &array[0], &array[n+1], &array[n], &array[n-1], &array[n-2], &array[n-3], &array[n-4]);
        }
        else if n == 6
        {
            print!("On the seventh day{}\nSeven {}, six {}, five {}\nFour {}, three {}\nTwo {} and a {}\n\n",
            &array[0], &array[n+1], &array[n], &array[n-1], &array[n-2], &array[n-3], &array[n-4], &array[n-5]);
        }
        else if n  == 7
        {
            print!("On the eight day{}\nEight {}, seven {}\nSix {}, five {}\nFour {}, three {}\nTwo {} and a {}\n\n", 
            &array[0], &array[n+1], &array[n], &array[n-1], &array[n-2],
            &array[n-3], &array[n-4], &array[n-5], &array[n - 6]);
        }
        else if n == 8
        {
            print!("On the ninth day{}\nNine {}, eight {}\nSeven {},  {}five {}, \nFour {}, three {}\nTwo {}and a {}\n\n", 
            &array[0], &array[n+1], &array[n], &array[n-1], &array[n-2],
            &array[n-3], &array[n-4], &array[n-5], &array[n - 6], &array[n - 7]);
        }
        else if n == 9
        {
            print!("On the tenth day{}\nTen {}, nine {}, eight {} \nSeven {}, six {}, five {}\nFour {}, three {}\nTwo {} and a {}\n\n", 
            &array[0], &array[n+1], &array[n], &array[n-1], &array[n-2],
            &array[n-3], &array[n-4], &array[n-5], &array[n - 6], &array[n - 7], &array[n - 8]);
        }
        else if n == 10
        {
            print!("On the eleventh day{}\nEleven {}, ten {}\nNine {}, eight {} \nSeven {}, six {}, five {}\nFour {}, three {}\nTwo {} and a {}\n\n", 
            &array[0], &array[n+1], &array[n], &array[n-1], &array[n-2],
            &array[n-3], &array[n-4], &array[n-5], &array[n - 6], &array[n - 7], &array[n - 8], &array[n - 9]);
        }

        else if n == 11
        {
            print!("On the twelfth day{}\nTwelve {}, eleven {}\nTen {}, nine {}, eight {} \nSeven {}, six {}, five {}\nFour {}, three {}\nTwo {} and a {}\n\n", 
            &array[0], &array[n+1], &array[n], &array[n-1], &array[n-2],
            &array[n-3], &array[n-4], &array[n-5], &array[n - 6], &array[n - 7], &array[n - 8], &array[n - 9], &array[n - 10] );
        }
    }
}