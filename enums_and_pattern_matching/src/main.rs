enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

/*
 Above enum equivalent to below structs:

 struct QuitMessage; // unit struct
 struct MoveMessage {
    x: i32,
    y: i32,
 }
 struct WriteMessage(String); // tuple struct
 struct ChangeColorMessage(i32, i32, i32); // tuple struct
*/

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}


fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
