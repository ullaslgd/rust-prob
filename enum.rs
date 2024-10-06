enum Direction {
    Up,
    Down,
    Left,
    Right,

}

fn main(){
    let player_direction = Direction::Up;
    move_around(player_direction);
}

fn move_around(direction: Direction){
    match direction {
        Direction::Up => println!("Moving Up"),
        Direction::Down => println!("Moving Down"),
        Direction::Left => println!("Moving Left"),
        Direction::Right => println!("Moving Right"),
    }
}
