
fn main() {
    //  for i in 0..11{
    //     print!("{} ",i)
    //  }
let sentence = String::from("my first for loop with iteration in string");
let first_word =get_first_word(sentence);
print!("{}",first_word)
}

fn get_first_word(sentence:String)->String{
    let mut ans = String::from("");
    for char in sentence.chars(){
        ans.push_str(char.to_string().as_str());
        if char ==' '{
            break;
        }
    }
    return  ans;
}