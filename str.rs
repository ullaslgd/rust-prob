fn main(){
     let name = String::from ("ullas");
     let length = str_length(name);
        println!("Length of the string is: {}", length);
}

fn str_length(str: String)->usize{
    str.len()
}
