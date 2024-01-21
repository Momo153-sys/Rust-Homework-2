use std::io::{self,Write};
fn main(){ 
    print!("Enter a text: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut user_input =String::new();
    
    io::stdin().read_line(&mut user_input)
    .expect("Failed to read line");
    
    let input = WordCounter::new(&user_input);
    
    let number_of_words=input.count_words();
    
    println!("The number of words is: {}",number_of_words);

}

struct WordCounter{
    text:String,
}

impl WordCounter{
    fn new(text:&str) ->WordCounter{
        let new_word = WordCounter{
            text:text.to_string(),
        };
        new_word
    }

    fn count_words(&self) -> usize{
       let words :Vec<&str>= self.text.split_whitespace().collect();
      let count = words.len();
      if count==0{
        println!("The text is empty !!");
      }
      count
    }

}