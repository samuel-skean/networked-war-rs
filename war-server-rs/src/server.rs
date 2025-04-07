use std::{
    io::{Cursor, Write},
    net::SocketAddr,
};

use rand::seq::SliceRandom;
use tokio::{
    io::{AsyncRead, AsyncWriteExt},
    net::TcpStream,
};

use crate::format::*;

pub struct Game {
    pub player_one: (TcpStream, SocketAddr),
    pub player_two: (TcpStream, SocketAddr),
}

pub async fn serve_game(mut game: Game) {
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

    // **Forreal??** *No* idea if this is safe, but it seems to give the right result now.
    let player_one_hand_msg =
        unsafe { std::mem::transmute::<_, [u8; 27]>(Message::GameStart(player_one_hand)) };

    dbg!(player_one_hand_msg);

    game.player_one
        .0
        .write_all(&player_one_hand_msg)
        .await
        .unwrap();
    loop {}
}
