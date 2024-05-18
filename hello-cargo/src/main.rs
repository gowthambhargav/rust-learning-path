fn main (){
    let mut c = String::from("value");
    update_str(&mut c);
    println!("{}",c)
}

fn update_str(str:&mut String){
    str.push_str(" world")
}