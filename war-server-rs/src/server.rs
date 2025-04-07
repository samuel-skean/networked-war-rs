use std::{
    io::{Cursor, Write},
    net::SocketAddr,
};

use rand::seq::SliceRandom;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::format::*;

pub struct Game {
    pub player_one: (TcpStream, SocketAddr),
    pub player_two: (TcpStream, SocketAddr),
}

pub async fn serve_game(mut game: Game) {
    // TODO: Is there any benefit to doing this in `tokio::select!`, or somehow
    // otherwise making it concurrrent? Stuffs gonna go into my kernel buffers
    // anyway, and we can't make progress, right? Well, we could kill the game
    // earlier if we read from either one.
    let mut scratch = [0; 27];
    game.player_one
        .0
        .read_exact(&mut scratch[..2])
        .await
        .unwrap();
    assert_eq!(&scratch[..2], Message::WantGame.as_ref());

    game.player_two
        .0
        .read_exact(&mut scratch[..2])
        .await
        .unwrap();

    assert_eq!(&scratch[..2], Message::WantGame.as_ref());

    // TODO: Consider https://docs.rs/rand/latest/rand/seq/trait.IteratorRandom.html#method.choose_multiple_fill.
    let mut all_cards_cursor = Cursor::new([0u8; NUM_CARDS_TOTAL as usize]);
    for c in 0..51 {
        all_cards_cursor.write_all(&[c]).unwrap();
    }
    // Forreal? There's *gotta* be a safe way to do this.
    let mut all_cards = unsafe {
        std::mem::transmute::<_, [Card; NUM_CARDS_TOTAL as usize]>(all_cards_cursor.into_inner())
    };
    // Does this care at all about PartialEq? Surely not?
    all_cards.shuffle(&mut rand::rng());

    dbg!(all_cards);

    let mut player_one_hand = [Card::default(); 26];
    let mut player_two_hand = [Card::default(); 26];
    player_one_hand.copy_from_slice(&all_cards[..26]);
    player_two_hand.copy_from_slice(&all_cards[26..]);

    game.player_one
        .0
        .write_all(Message::GameStart(player_one_hand).as_ref())
        .await
        .unwrap();
    game.player_two
        .0
        .write_all(Message::GameStart(player_two_hand).as_ref())
        .await
        .unwrap();
    loop {}
}
