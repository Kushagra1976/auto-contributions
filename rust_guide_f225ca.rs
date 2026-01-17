// Learning Objective: This tutorial demonstrates how to build a basic real-time game state synchronization system in Rust
// using the actor model (with the `actix` framework) and WebSockets.
// We will focus on how actors can communicate and share state updates with connected clients.
// This is a foundational step for creating multiplayer games or collaborative applications.

// Import necessary crates
// `actix` provides the actor framework and web server capabilities.
// `actix-web-actors` provides WebSocket support for actix-web.
// `tokio` is the async runtime that actix uses.
// `serde` is used for serializing and deserializing data for network transmission.
use actix::*;
use actix_web::*;
use actix_web_actors::ws;
use std::time::{Duration, Instant};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

// Define a message type for game state updates.
// Players will send this to the server to announce their actions.
#[derive(Message, Serialize, Deserialize, Clone)]
#[rtype(result = "()")] // This message doesn't expect a direct reply from an actor.
struct GameStateUpdate {
    player_id: usize,
    // For simplicity, we'll just update a player's position.
    // In a real game, this would be more complex.
    x: i32,
    y: i32,
}

// Define a message for a player joining the game.
#[derive(Message, Serialize, Deserialize, Clone)]
#[rtype(result = "()")]
struct PlayerJoined {
    player_id: usize,
}

// Define a message for a player leaving the game.
#[derive(Message, Serialize, Deserialize, Clone)]
#[rtype(result = "()")]
struct PlayerLeft {
    player_id: usize,
}

// Represents the global game state.
// We'll use a HashMap to store the state of each connected player.
#[derive(Clone, Serialize, Deserialize)]
struct GameState {
    // Maps player_id to their current x, y coordinates.
    players: HashMap<usize, (i32, i32)>,
}

impl GameState {
    // Creates a new, empty game state.
    fn new() -> Self {
        GameState {
            players: HashMap::new(),
        }
    }
}

// The central `GameServer` actor.
// This actor will manage the global game state and broadcast updates to all connected players.
struct GameServer {
    // Stores the current state of the game.
    state: GameState,
    // Tracks connected players and their associated WebSocket actor addresses.
    // This allows us to send messages directly to specific players.
    // Key: player_id, Value: Actor address of the WebSocket session.
    sessions: HashMap<usize, Addr<WsSession>>,
    // A simple counter to assign unique player IDs.
    next_player_id: usize,
}

impl Actor for GameServer {
    // The `Context` is used for managing the actor's lifecycle and communication.
    type Context = Context<Self>;

    // `started` is called when the actor is spawned.
    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("GameServer started.");
        // Initialize with some dummy player for demonstration
        let initial_player_id = self.next_player_id;
        self.next_player_id += 1;
        self.state.players.insert(initial_player_id, (0, 0));
        println!("Initial player {} joined at (0, 0).", initial_player_id);
    }

    // `stopped` is called when the actor is stopping.
    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("GameServer stopped.");
    }
}

impl GameServer {
    // Factory function to create a new `GameServer`.
    fn new() -> Self {
        GameServer {
            state: GameState::new(),
            sessions: HashMap::new(),
            next_player_id: 0,
        }
    }

    // Broadcasts a message to all connected WebSocket sessions.
    // This is the core of our real-time synchronization.
    fn broadcast(&self, msg: &str) {
        // Iterate over all registered sessions and send the message.
        for (_session_id, addr) in &self.sessions {
            addr.do_send(ws::Message::Text(msg.to_string()));
        }
    }
}

// Handle messages sent to the `GameServer` actor.
impl Handler<PlayerJoined> for GameServer {
    type Result = (); // No result expected from this handler.

    // When a player joins, add them to the session list and update the game state.
    fn handle(&mut self, msg: PlayerJoined, _ctx: &mut Self::Context) -> Self::Result {
        let player_id = msg.player_id;
        println!("Player {} joined the game.", player_id);

        // Add player to game state if not already present (e.g., on initial join).
        if !self.state.players.contains_key(&player_id) {
            self.state.players.insert(player_id, (0, 0)); // Default position
        }

        // Prepare the game state for broadcasting.
        let game_state_json = serde_json::to_string(&self.state).unwrap();
        self.broadcast(&format!("STATE:{}", game_state_json));
        Ok(())
    }
}

impl Handler<PlayerLeft> for GameServer {
    type Result = ();

    // When a player leaves, remove them from the session list and game state.
    fn handle(&mut self, msg: PlayerLeft, _ctx: &mut Self::Context) -> Self::Result {
        let player_id = msg.player_id;
        println!("Player {} left the game.", player_id);

        // Remove player from sessions.
        self.sessions.remove(&player_id);
        // Remove player from game state.
        self.state.players.remove(&player_id);

        // Broadcast the updated game state to all remaining players.
        let game_state_json = serde_json::to_string(&self.state).unwrap();
        self.broadcast(&format!("STATE:{}", game_state_json));
        Ok(())
    }
}

impl Handler<GameStateUpdate> for GameServer {
    type Result = ();

    // When a game state update is received, process it and broadcast to all.
    fn handle(&mut self, msg: GameStateUpdate, _ctx: &mut Self::Context) -> Self::Result {
        let player_id = msg.player_id;
        let (x, y) = (msg.x, msg.y);
        println!("Received update for player {}: ({}, {}).", player_id, x, y);

        // Update the player's position in the global game state.
        if let Some(player_pos) = self.state.players.get_mut(&player_id) {
            *player_pos = (x, y);
        } else {
            // If player is not in state, add them (e.g., if they just joined and sent an update)
            self.state.players.insert(player_id, (x, y));
        }

        // Broadcast the entire updated game state to all connected clients.
        // This ensures everyone sees the latest state.
        let game_state_json = serde_json::to_string(&self.state).unwrap();
        self.broadcast(&format!("STATE:{}", game_state_json));
        Ok(())
    }
}

// The `WsSession` actor represents a single WebSocket connection from a client.
// Each client will have its own `WsSession` actor instance.
struct WsSession {
    // A reference to the `GameServer` actor.
    // This allows the `WsSession` to send messages to the server.
    // `Option` because it's set asynchronously after handshake.
    server: Option<Addr<GameServer>>,
    // A unique identifier for this player's session.
    player_id: usize,
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    // `started` is called when the actor is spawned.
    fn started(&mut self, ctx: &mut Self::Context) {
        println!("WebSocket session started.");
        // We'll set the server address and player ID later when the handshake is complete.
        // For now, we do nothing here directly, but this is where you'd initialize resources.
    }

    // `stopped` is called when the actor is stopping.
    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("WebSocket session stopped.");
        // When a session stops, we need to notify the GameServer that the player has left.
        if let Some(server) = self.server.as_ref() {
            server.do_send(PlayerLeft { player_id: self.player_id });
        }
    }
}

impl WsSession {
    // Factory function to create a new `WsSession`.
    fn new() -> Self {
        WsSession {
            server: None, // Will be set by the `index` handler.
            player_id: 0, // Will be set by the `index` handler.
        }
    }

    // Method to handle text messages received from the client.
    // We expect messages in the format "PLAYER:<id>" for initial connection,
    // and "UPDATE:<x>,<y>" for game state updates.
    fn handle_text_message(&mut self, ctx: &mut <Self as Actor>::Context, msg: &str) {
        println!("Received message: {}", msg);

        // Handle initial player ID assignment.
        if msg.starts_with("PLAYER:") {
            if let Ok(player_id_str) = msg.trim_start_matches("PLAYER:").parse::<usize>() {
                self.player_id = player_id_str;
                println!("Client assigned player ID: {}", self.player_id);
                // Now that we have the player ID, we can get the GameServer address.
                // The GameServer address is passed to the session via `set_server_addr`.
                if let Some(server) = self.server.as_ref() {
                    // Notify the server that this player has joined.
                    server.do_send(PlayerJoined { player_id: self.player_id });
                } else {
                    println!("Error: GameServer address not set for session.");
                }
            } else {
                println!("Invalid PLAYER ID format: {}", msg);
            }
            return; // Stop processing this message further.
        }

        // Handle game state updates from the client.
        if msg.starts_with("UPDATE:") {
            let coords_str = msg.trim_start_matches("UPDATE:");
            let parts: Vec<&str> = coords_str.split(',').collect();
            if parts.len() == 2 {
                if let (Ok(x), Ok(y)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                    // If we have a server connection, send the update to the GameServer.
                    if let Some(server) = self.server.as_ref() {
                        server.do_send(GameStateUpdate {
                            player_id: self.player_id,
                            x,
                            y,
                        });
                    } else {
                        println!("Error: GameServer address not set for session.");
                    }
                } else {
                    println!("Invalid coordinate format: {}", msg);
                }
            } else {
                println!("Invalid UPDATE format: {}", msg);
            }
            return; // Stop processing this message further.
        }

        // If message is not recognized, echo it back or log an error.
        ctx.text(format!("Echo: {}", msg));
    }

    // Method to send the current game state to a specific client.
    // This is used when a new client connects or when we want to send the full state.
    fn send_game_state(&self, ctx: &mut <Self as Actor>::Context, state: &GameState) {
        let state_json = serde_json::to_string(state).unwrap();
        ctx.text(format!("STATE:{}", state_json));
    }

    // This method is called when the server needs to send a message *to* this session.
    // It's also called from the `GameServer`'s `broadcast` method.
    fn handle_server_message(&mut self, ctx: &mut <Self as Actor>::Context, message: &str) {
        println!("Received message from server: {}", message);
        ctx.text(message); // Send the message from server to client.
    }
}

// Define the actor messages that a `WsSession` can receive.
// These are messages *sent to* the `WsSession` actor.
impl Handler<ws::Message> for WsSession {
    type Result = (); // No result expected from handling WebSocket messages.

    // This handler processes incoming WebSocket messages from the client.
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            // If the client sends a text message.
            ws::Message::Text(text) => {
                // Our `WsSession` actor has a specific method to handle text messages.
                self.handle_text_message(ctx, &text);
            }
            // If the client sends a binary message.
            ws::Message::Binary(bin) => {
                println!("Received binary data: {:?}. Not supported.", bin);
                // In a real game, you might handle binary data for performance.
            }
            // If the client closes the connection.
            ws::Message::Close(close_option) => {
                println!("Client closed connection: {:?}", close_option);
                // The `stopped` method will be called automatically by actix when the actor stops.
                ctx.close(close_option.as_ref().map(|m| m.code));
                ctx.stop(); // Explicitly stop the actor.
            }
            // Other WebSocket message types.
            ws::Message::Ping(ping_data) => {
                // Respond to pings to keep the connection alive.
                ctx.pong(&ping_data);
            }
            ws::Message::Pong(_) => {
                // Received a pong, connection is alive.
            }
            ws::Message::Continuation(_) => {
                // Handle continuation frames if needed for large messages.
            }
            ws::Message::Nop => {
                // No operation.
            }
        }
        Ok(())
    }
}

// This struct is used to represent an actor that can send messages to a `GameServer`.
// It's used by the `index` handler to create and register a `WsSession`.
struct RegisterWebsocket {
    addr: Addr<GameServer>,
}

// The `RegisterWebsocket` message is sent to a `WsSession` to establish the connection to the `GameServer`.
impl Message for RegisterWebsocket {
    type Result = usize; // Returns the assigned player ID from the server.
}

// Handler for the `RegisterWebsocket` message on the `WsSession` actor.
impl Handler<RegisterWebsocket> for WsSession {
    type Result = usize; // This handler will return the player ID.

    fn handle(&mut self, msg: RegisterWebsocket, _ctx: &mut Self::Context) -> Self::Result {
        println!("Registering WebSocket session with GameServer.");
        // Store the address of the GameServer.
        self.server = Some(msg.addr.clone());
        // The server will assign a player_id when it's ready. For now, we use a placeholder.
        // We'll send the actual player_id back to the client via the server.
        // The server will assign a player_id and add the session to its list.
        // The GameServer's `PlayerJoined` handler will ensure the player_id is set.
        // For now, we return 0 as a placeholder. The actual assignment happens through client messages.
        0 // Placeholder player ID. The actual assignment happens via "PLAYER:id" from client.
    }
}

// This `WsConnection` struct handles the initial WebSocket handshake and spawns the `WsSession` actor.
// It's called by the `index` handler.
async fn ws_index(
    req: HttpRequest,
    stream: web::Payload,
    // Access to the `GameServer` actor system. This allows us to get its address.
    srv: web::Data<Addr<GameServer>>,
) -> Result<HttpResponse, ActixError> {
    println!("WebSocket connection requested.");

    // Perform the WebSocket handshake.
    let ws = ws::handshake(req.get_header("sec-websocket-key").unwrap(), stream)?
        .respond_with_protocol("game"); // Optional: specify a subprotocol.

    // Spawn a new `WsSession` actor for this connection.
    let session_actor = WsSession::new().start();

    // Send the `GameServer` address to the new `WsSession` actor so it can communicate with the server.
    // This also implicitly registers the session with the server.
    srv.get()
        .send(RegisterWebsocket { addr: srv.get_ref().clone() })
        .into_actor(session_actor.recipient()) // Send to the session actor.
        .then(move |res, actor, ctx| {
            match res {
                Ok(player_id) => {
                    println!("Session registered. Assigned player ID (placeholder): {}", player_id);
                    // We don't get the player_id here directly from this, but it establishes communication.
                    // The client will send "PLAYER:<id>" to confirm its ID.
                }
                Err(e) => {
                    println!("Error registering session: {}", e);
                }
            }
            // Return the handshake response to the client.
            future::ok(ws.finish(session_actor.start(ws::MessageReceiver::new(ctx.address()))))
        })
        .await
}

// The main application entry point.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start the GameServer actor. This actor will manage the game state.
    let game_server = GameServer::new().start();

    // Configure and start the Actix web server.
    HttpServer::new(move || {
        App::new()
            // Add a shared state to the application for the GameServer actor address.
            // This makes the `game_server` address accessible to handlers.
            .app_data(web::Data::new(game_server.clone()))
            // Define the route for our WebSocket endpoint.
            .route("/ws/", web::get().to(ws_index))
            // Serve a static HTML file for the client to connect to.
            // This assumes you have an `index.html` in a `static` folder.
            .service(fs::Files::new("/static", "static/").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")? // Bind the server to a local address and port.
    .run() // Start the server.
    .await
}

// Example Usage (Conceptual):
// 1. Save this code as `src/main.rs`.
// 2. Create a `static` directory in the root of your project.
// 3. Inside `static`, create `index.html` with the following content:
//
// <!DOCTYPE html>
// <html>
// <head>
//     <title>Game Client</title>
// </head>
// <body>
//     <h1>Real-time Game State</h1>
//     <div id="game-state">Waiting for connection...</div>
//     <button onclick="sendUpdate()">Update Position</button>
//     <input type="number" id="playerId" placeholder="Your Player ID (e.g., 0)">
//     <input type="number" id="xPos" placeholder="X Position">
//     <input type="number" id="yPos" placeholder="Y Position">
//
//     <script>
//         let websocket;
//         const playerIdInput = document.getElementById('playerId');
//         const xPosInput = document.getElementById('xPos');
//         const yPosInput = document.getElementById('yPos');
//
//         function connect() {
//             const wsUrl = `ws://${window.location.host}/ws/`;
//             websocket = new WebSocket(wsUrl);
//
//             websocket.onopen = function(event) {
//                 console.log("WebSocket connected!");
//                 const playerId = parseInt(playerIdInput.value) || 0; // Default to 0 if not set
//                 websocket.send("PLAYER:" + playerId);
//             };
//
//             websocket.onmessage = function(event) {
//                 console.log("Message from server: ", event.data);
//                 if (event.data.startsWith("STATE:")) {
//                     const state = JSON.parse(event.data.substring("STATE:".length));
//                     document.getElementById('game-state').innerText = JSON.stringify(state, null, 2);
//                 }
//             };
//
//             websocket.onerror = function(event) {
//                 console.error("WebSocket error: ", event);
//             };
//
//             websocket.onclose = function(event) {
//                 console.log("WebSocket closed: ", event.code, event.reason);
//             };
//         }
//
//         function sendUpdate() {
//             const playerId = parseInt(playerIdInput.value);
//             const x = parseInt(xPosInput.value);
//             const y = parseInt(yPosInput.value);
//
//             if (isNaN(playerId) || isNaN(x) || isNaN(y)) {
//                 alert("Please enter valid Player ID, X, and Y positions.");
//                 return;
//             }
//
//             if (websocket && websocket.readyState === WebSocket.OPEN) {
//                 websocket.send(`UPDATE:${x},${y}`);
//             } else {
//                 alert("WebSocket is not connected.");
//             }
//         }
//
//         // Connect on page load
//         window.onload = connect;
//
//     </script>
// </body>
// </html>
//
// 4. Add `actix-web-actors`, `tokio`, `serde`, `serde_json` to your `Cargo.toml`:
//
// [dependencies]
// actix = "0.13"
// actix-web = "4"
// actix-web-actors = "4"
// tokio = { version = "1", features = ["full"] }
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"
//
// 5. Run the application: `cargo run`
// 6. Open your browser to `http://127.0.0.1:8080/static/`.
//    You should see the `index.html` page.
//    Enter a Player ID and click "Update Position" to send updates.
//    Open another tab/browser and connect to the same URL, assign a DIFFERENT Player ID.
//    You should see both player's positions update in real-time in both tabs.