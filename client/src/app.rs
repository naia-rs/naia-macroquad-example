use std::{
    net::{IpAddr, SocketAddr},
    time::Duration,
};

use macroquad::prelude::*;

use naia_client::{ClientConfig, ClientEvent, NaiaClient};

use naia_mq_example_shared::{
    get_shared_config, manifest_load, shared_behavior, AuthEvent, ExampleActor, ExampleEvent,
    KeyCommand, PointActorColor,
};

const SERVER_PORT: u16 = 14191;

pub struct App {
    client: NaiaClient<ExampleEvent, ExampleActor>,
    pawn_key: Option<u16>,
    queued_command: Option<KeyCommand>,
}

impl App {
    pub fn new() -> Self {
        info!("Naia Macroquad Client Example Started");

        // Put your Server's IP Address here!, can't easily find this automatically from the browser
        let server_ip_address: IpAddr = "127.0.0.1"
            .parse()
            .expect("couldn't parse input IP address");
        let server_socket_address = SocketAddr::new(server_ip_address, SERVER_PORT);

        let mut client_config = ClientConfig::default();
        client_config.heartbeat_interval = Duration::from_secs(2);
        client_config.disconnection_timeout_duration = Duration::from_secs(5);

        let auth = ExampleEvent::AuthEvent(AuthEvent::new("charlie", "12345"));

        let client = NaiaClient::new(
            server_socket_address,
            manifest_load(),
            Some(client_config),
            get_shared_config(),
            Some(auth),
        );

        App {
            client,
            pawn_key: None,
            queued_command: None,
        }
    }

    pub fn update(&mut self) {
        // input
        let w = is_key_down(KeyCode::W);
        let s = is_key_down(KeyCode::S);
        let a = is_key_down(KeyCode::A);
        let d = is_key_down(KeyCode::D);

        if let Some(command) = &mut self.queued_command {
            if w {
                command.w.set(true);
            }
            if s {
                command.s.set(true);
            }
            if a {
                command.a.set(true);
            }
            if d {
                command.d.set(true);
            }
        } else {
            self.queued_command = Some(KeyCommand::new(w, s, a, d));
        }

        // update
        loop {
            if let Some(result) = self.client.receive() {
                match result {
                    Ok(event) => match event {
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
                        ClientEvent::Command(pawn_key, command_type) => match command_type {
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
                        },
                        _ => {}
                    },
                    Err(err) => {
                        info!("Client Error: {}", err);
                    }
                }
            } else {
                break;
            }
        }

        // drawing
        clear_background(BLACK);

        let square_size = 32.0;

        if self.client.has_connection() {
            // draw actors
            for actor_key in self.client.actor_keys().unwrap() {
                if let Some(actor) = self.client.get_actor(&actor_key) {
                    match actor {
                        ExampleActor::PointActor(actor_ref) => {
                            let point_actor = actor_ref.borrow();
                            let color = match point_actor.color.get() {
                                PointActorColor::Red => RED,
                                PointActorColor::Blue => BLUE,
                                PointActorColor::Yellow => YELLOW,
                            };
                            draw_rectangle(
                                f32::from(*(point_actor.x.get())),
                                f32::from(*(point_actor.y.get())),
                                square_size,
                                square_size,
                                color,
                            );
                        }
                    }
                }
            }

            // draw pawns
            for pawn_key in self.client.pawn_keys().unwrap() {
                if let Some(actor) = self.client.get_pawn(&pawn_key) {
                    match actor {
                        ExampleActor::PointActor(actor_ref) => {
                            let point_actor = actor_ref.borrow();
                            draw_rectangle(
                                f32::from(*(point_actor.x.get())),
                                f32::from(*(point_actor.y.get())),
                                square_size,
                                square_size,
                                WHITE,
                            );
                        }
                    }
                }
            }
        }
    }
}
