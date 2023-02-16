# rust-chess-engine
[![wakatime](https://wakatime.com/badge/github/dkomeza/rust-chess-engine.svg)](https://wakatime.com/badge/github/dkomeza/rust-chess-engine)

A chess engine written in Rust.


## Working principles
 - main
   - Init UCI
   - Init endgames
   - Create threads
   - Clear search
   - UCI loop

 - UCI loop
   - uci - Engine info
   - setoption - Set engine options
   - go - Start searching
     - set search limits
     - start threads (position, StateInfo, limits)
   - position - set position
     - create fen
     - set position 
     - play moves
   - ucinewgame - Reset engine
   - isready - Check if engine is ready
   - quit - Quit engine

 
StateInfo {
    pawnKey: u64,
    materialKey: u64,
    nonPawnMaterial: [i32; 2],
    castlingRights: u8,
    rule50: i32,
    pliesFromNull: i32,
    Square: u8,
}