enum Direction {
    North,
    South,
    East,
    West,
}

fn move_player(dir: Direction) {
    match dir {
        Direction::North => println!("Moving up! ⬆️"),
        Direction::South => println!("Moving Down! ⬇️"),
        Direction::East => println!("Moving Right! ➡️"),
        Direction::West => println!("Moving Left! ⬅️"),
    }
}

fn main() {
    let player_move = Direction::North;
    let another_move = Direction::East;

    move_player(player_move);
    move_player(another_move);
}