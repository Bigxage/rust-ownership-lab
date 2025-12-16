enum GameEvent {
    Quit,
    PlayerJoined(String),
    Move { x:i32, y:i32 },
    Chat(String, String),
}

fn process_event(event: GameEvent) {
    match event {
        GameEvent::Quit => {
            println!("Game Over! Shutting down...");
        }

        GameEvent::PlayerJoined(name) => {
            println!("New challenger approached: {}!", name);
        }

        GameEvent::Move { x,y } => {
            println!("Player moved to coordinates X: {}, Y: {}", x,y);
        }

        GameEvent::Chat(name, msg) => {
            println!("[Global Chat] {}: {}", name, msg);
        }
    }
}

fn main() {
    let e1 = GameEvent::PlayerJoined("Rahim".to_string());
    let e2 = GameEvent::Move { x: 10, y: 20};
    let e3 = GameEvent::Chat("Inarah".to_string(), "Good luck with Rust!".to_string());
    let e4 = GameEvent::Quit;

    process_event(e1);
    process_event(e2);
    process_event(e3);
    process_event(e4);
}