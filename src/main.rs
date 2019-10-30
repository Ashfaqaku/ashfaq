use std::collections::HashMap;
use std::io;

fn main() {
    create_hash_map();
    use_zip_to_create_hash_map();
    colletcion_words_from_string();     
}

fn create_hash_map(){

    let mut scores = HashMap::new();
    let mut guess = String::new();

    io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    let guess = guess.trim().parse().unwrap();

    scores.insert(String::from("Red"), 12);
    scores.insert("blue".to_string(),24); 

    scores.entry("yellow".to_string()).or_insert(guess);

    println!("Creating hashmap{:?}", scores );
    let red_score = scores.get("red");
    println!("{:?}",red_score );
    for (key, value) in scores{
        println!("for loop key: {}, for loop value {}", key,value )
    }
}

fn use_zip_to_create_hash_map(){

    let team = vec!["red","blue"];
    let scores = vec![55,23];

    let team_hash: HashMap<_,_> = team.iter().zip(scores.iter()).collect();

    println!("using zip: {:?}",team_hash.values());

}

fn colletcion_words_from_string(){

    let mut ss = String::new();

    io::stdin()
            .read_line(&mut ss)
            .expect("Failed to read line");
    //let ss = ss.trim().parse().unwrap();

    let my_string = ss;

    let mut map= HashMap::new();

    for word in my_string.split(""){
        let count = map.entry(word).or_insert(0);
        *count +=1;
    }
    println!("The characters in the str : {:?}", map );
}