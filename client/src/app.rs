
use std::{net::SocketAddr, time::Duration};

use naia_client::{ClientConfig, ClientEvent, NaiaClient};

use naia_mq_example_shared::{get_shared_config, manifest_load, AuthEvent, ExampleActor, ExampleEvent, KeyCommand, shared_behavior, PointActorColor};

use miniquad::*;

const SERVER_PORT: u16 = 14191;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        use std::net::IpAddr;
    } else {
        use naia_client::find_my_ip_address;
    }
}

pub struct App {
    client: NaiaClient<ExampleEvent, ExampleActor>,
    pawn_key: Option<u16>,
    queued_command: Option<KeyCommand>,
}

impl App {
    pub fn new() -> Self {
        info!("Naia Miniquad Client Example Started");

        cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                // Put your Server's IP Address here!, can't easily find this automatically from the browser
                let server_ip_address: IpAddr = "192.168.86.38".parse().expect("couldn't parse input IP address");
            } else {
                let server_ip_address = find_my_ip_address().expect("can't find ip address");
            }
        }

        let server_socket_address = SocketAddr::new(server_ip_address, SERVER_PORT);

        let mut client_config = ClientConfig::default();
        client_config.heartbeat_interval = Duration::from_secs(2);
        client_config.disconnection_timeout_duration = Duration::from_secs(5);

        let auth = ExampleEvent::AuthEvent(AuthEvent::new("charlie", "12345"));

        let mut client = NaiaClient::new(
            server_socket_address,
            manifest_load(),
            Some(client_config),
            get_shared_config(),
            Some(auth),
        );

        let mut pawn_key: Option<u16> = None;
        let mut queued_command: Option<KeyCommand> = None;

        App {
            client,
            pawn_key,
            queued_command,
        }
    }

    pub fn update(&mut self) {
        if let Some(result) = self.client.receive() {
            match result {
                Ok(event) => {
                    match event {
                        ClientEvent::Connection => {
                            info!("Client connected to: {}", self.client.server_address());
                        }
                        ClientEvent::Disconnection => {
                            info!("Client disconnected from: {}", self.client.server_address());
                        }
                        ClientEvent::Tick => {
                            if let Some(pawn_key) = self.pawn_key {
                                if let Some(command) = self.queued_command.take() {
                                    self.client.send_command(pawn_key, &command);
                                }
                            }
                        }
                        ClientEvent::AssignPawn(local_key) => {
                            self.pawn_key = Some(local_key);
                            info!("assign pawn");
                        }
                        ClientEvent::UnassignPawn(_) => {
                            self.pawn_key = None;
                            info!("unassign pawn");
                        }
                        ClientEvent::Command(pawn_key, command_type) => {
                            match command_type {
                                ExampleEvent::KeyCommand(key_command) => {
                                    if let Some(typed_actor) = self.client.get_pawn_mut(&pawn_key) {
                                        match typed_actor {
                                            ExampleActor::PointActor(actor) => {
                                                shared_behavior::process_command(&key_command, actor);
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                Err(err) => {
                    info!("Client Error: {}", err);
                }
            }
        }
    }

    pub fn draw(&mut self, mut ctx: &Context) {
        ctx.clear(Some((0., 0., 0., 0.)), None, None);

//        if client.has_connection() {
//            // draw actors
//            for actor_key in client.actor_keys().unwrap() {
//                if let Some(actor) = client.get_actor(&actor_key) {
//                    match actor {
//                        ExampleActor::PointActor(point_actor) => {
//                            let rect = Rectangle::new(
//                                Vector::new(
//                                    f32::from(*(point_actor.as_ref().borrow().x.get())),
//                                    f32::from(*(point_actor.as_ref().borrow().y.get()))),
//                                square_size);
//                            match point_actor.as_ref().borrow().color.get() {
//                                PointActorColor::Red => gfx.fill_rect(&rect, Color::RED),
//                                PointActorColor::Blue => gfx.fill_rect(&rect, Color::BLUE),
//                                PointActorColor::Yellow => gfx.fill_rect(&rect, Color::YELLOW),
//                            }
//                        }
//                    }
//                }
//            }
//
//            // draw pawns
//            for pawn_key in client.pawn_keys().unwrap() {
//                if let Some(actor) = client.get_pawn(&pawn_key) {
//                    match actor {
//                        ExampleActor::PointActor(point_actor) => {
//                            let rect = Rectangle::new(
//                                Vector::new(
//                                    f32::from(*(point_actor.as_ref().borrow().x.get())),
//                                    f32::from(*(point_actor.as_ref().borrow().y.get()))),
//                                square_size);
//                            gfx.fill_rect(&rect, Color::WHITE);
//                        }
//                    }
//                }
//            }
//        }
    }
}

//pub async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
//
//    // Quicksilver
//
//    let square_size = Vector::new(32.0, 32.0);
//
//    let mut frame_timer = Timer::time_per_second(60.0);
//
//    let mut pawn_key: Option<u16> = None;
//    let mut queued_command: Option<KeyCommand> = None;
//
//    loop {
//        while let Some(_) = input.next_event().await {}
//
//        if frame_timer.exhaust().is_some() {
//
//            // input
//            let w = input.key_down(Key::W);
//            let s = input.key_down(Key::S);
//            let a = input.key_down(Key::A);
//            let d = input.key_down(Key::D);
//
//            if let Some(command) = &mut queued_command {
//                if w { command.w.set(true); }
//                if s { command.s.set(true); }
//                if a { command.a.set(true); }
//                if d { command.d.set(true); }
//            } else {
//                queued_command = Some(KeyCommand::new(w, s, a, d));
//            }
//
//            // update
//            loop {
//
//            }
//
//            // drawing
//            gfx.clear(Color::BLACK);
//
//
//        }
//    }
//}