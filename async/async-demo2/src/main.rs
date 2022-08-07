use futures::executor::block_on;

#[derive(Debug)]
struct Song;

async fn learn_song() -> Song {
    println!("learn song");
    Song{}
}

async fn sing_song(song: Song) {
    println!("sing song: {:?}", song);
}

async fn dance() {
    println!("dance");
}

async fn learn_and_sing_song() {
    let s = learn_song().await;
    sing_song(s).await;
}

async fn async_main() {
    let f1 = learn_and_sing_song();
    let f2 = dance();
    futures::join!(f1, f2);
}

fn main() {
    block_on(async_main());
}