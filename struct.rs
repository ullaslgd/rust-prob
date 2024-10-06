struct Point{
    x:i32,
    y:i32,
}

impl Point {
    fn add(&self) -> i32{
        return self.x + self.y;
    }

    fn sub(&self)   -> i32{
       return self.x - self.y;
    }

}

fn  main(){
 let p = Point{x:10, y:20};
    println!("Addition of x and y is: {}", p.add());
    println!("Subtraction of x and y is: {}", p.sub());
}
