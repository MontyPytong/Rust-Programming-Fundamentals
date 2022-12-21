// Info VECTORS: 

//     Can store multiple pieces of data (must be the same type)
//     Used for lists of informations
//     Can add, remove and traverse the entries

// EXAMPLE
// fn main(){
//     let my_numbers = vec![1,2,3,];

//     let mut my_numbers = Vec::new();
//     my_numbers.push(1);
//     my_numbers.push(2);
//     my_numbers.push(3); // add 3 to the vector
//     my_numbers.pop(); // remove the last item added
//     my_numbers.len(); // the lenth of the item

//     let two = my_numbers[1]; // acces the vector item

//     // Itterating through a vector

//     let my_numbers = vec![1,2,3];

//     for num in my_numbers {
//         println!("{:?}", num);
//     }

// }

struct Test{
    score:i32
}

fn main(){
    let my_score = vec![
        Test {score:90},
        Test {score:88},
        Test {score:77},
        Test {score:93},
    ];

    for test in my_score{
        println!("Score = {:?}", test.score)
    }
}



//Recap 
    // The vec! macro can be used to make vectors 
    // Use for ..in to iterate through items of a vector 