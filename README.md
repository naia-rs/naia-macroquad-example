# naia-miniquad-example
![](https://tokei.rs/b1/github/naia-rs/naia-socket)
[![Discord chat](https://img.shields.io/discord/764975354913619988.svg?label=discord%20chat)](https://discord.gg/fD6QCtX)
[![MIT/Apache][s3]][l3]

Demonstrates using naia with miniquad to create a 2D multiplayer web game

### Server:

To run a UDP server on Linux: (that will be able to communicate with Linux clients)

    cargo run --features "use-udp"

To run a WebRTC server on Linux: (that will be able to communicate with Web clients)

    cargo run --features "use-webrtc"

### Client:

To run a UDP client on Linux: (that will be able to communicate with a UDP server)

    cargo run

To run a WebRTC client on Web: (that will be able to communicate with a WebRTC server)

    1. Enter in your IP Address at the appropriate spot in examples/client/src/app.rs
    2. cd client
    3. npm install              //should only need to do this once to install dependencies
    4. npm run start            //this will open a web browser, and hot reload


To simply build these examples instead of running them, substitute the above commands like so:

    `cargo build` for `cargo run`, and

    `npm run build` for `npm run start`
