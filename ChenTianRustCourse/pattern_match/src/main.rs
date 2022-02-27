fn main() {
    let join = Event::Join((1, 10001));
    let msg = Event::Message((1, 10001, String::from("Hello everyone. This is Morris.🥳")));
    let msg2 = Event::Message((2, 10001, String::from("Hi Morris. I'm Nico.")));
    let leave = Event::Leave((1, 10001));
    process_event(join);
    process_event(msg);
    process_message(msg2);
    process_event(leave);

}

// 标签联合
#[derive(Debug)]
enum Event {
    Join((u8, u32)),
    Leave((u8, u32)),
    Message((u8, u32, String)),
}

fn process_event(event: Event) {
    match event {
        Event::Join((uid, _tid)) => println!("User {:?} joined", uid),
        Event::Leave((uid, tid)) => println!("User {:?} left {:?}", uid, tid),
        Event::Message((_, _, msg)) => println!("broadcast: {}", msg),
    }
}

fn process_message(event: Event) {
    if let Event::Message((_, _, msg)) = event {
        println!("broadcast: {}", msg);
    }
}