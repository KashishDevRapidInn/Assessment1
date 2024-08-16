use std::collections::HashMap;

// fn sort_by_ages(names: Vec<&str>, ages: Vec<i32>)->Vec<&str>{
//     let mut answer= Vec::new();
//     let mut positions= HashMap::new();
//     for i in 0..ages.len(){
//         let mut count=0;
//         for j in 0..ages.len(){
//             if ages[i]<ages[j]{
//                 count+=1;
//             }
//         }
//         positions.insert(ages.len()-count, names[i]);
//     }
//     let mut sorted_positions: Vec<usize> = positions.keys().cloned().collect();
//     sorted_positions.sort();
    
//     for i in sorted_positions {
//         if let Some(&name) = positions.get(&i) {
//             answer.push(name);
//         }
//     }
    
//     answer
// }

fn sort_by_ages(names: Vec<&str>, ages: Vec<i32>)->Vec<&str>{
    let mut name_age: Vec<(&str, i32)>= names.iter()
                                                .zip(ages.iter())
                                                .map(|(&name, &age)|(name, age))
                                                .collect();
    name_age.sort_by_key(|&(_,age)|age);
    let answer= name_age.iter()
                            .map(|&(name,_)|name)
                            .collect();
    answer
}
fn main() {
    let names= vec!["person1", "person2", "person3", "person4"];
    let ages= vec![15, 32, 29, 12];
    let answer= sort_by_ages(names, ages);
    println!("{:?}", answer);
}
