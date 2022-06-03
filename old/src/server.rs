// use std::{
//     collections::{HashMap, HashSet},
//     net::SocketAddr,
//     sync::{Arc, Mutex},
// };
//
// // TODO: determine how much of this from the axum websocket example is needed
// use axum::{
//     extract::{
//         ws::{Message, WebSocket, WebSocketUpgrade},
//         Extension,
//         TypedHeader,
//     },
//     http::StatusCode,
//     response::{Html, IntoResponse},
//     routing::{get, get_service},
//     Router,
// };
// use futures::{sink::SinkExt, stream::StreamExt};
// use tokio::sync::broadcast;
// use tower_http::{
//     services::ServeDir,
//     trace::{DefaultMakeSpan, TraceLayer},
// };
// use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
//
// use crate::user::User;
//
// pub struct Server {
//     user_info_file:String,
//     ip_address:String,
//     server_port:String,
//     master_socket_descriptor:i16,
//     registered_users:HashMap<String, User>,
//     open_connections:HashMap<thread, User>,
//     connected_users:i16
// }
//
// impl Server{
//     fn setup_config(&self, file:String) {}
//     fn setup_users(&self, file:String) {}
//     fn init_server(&self) {}
//     fn shutdown(&self) {}
//
//     // TODO: determine if these are necessary
//     fn get_ip(&self) -> &String { &self.ip_address }
//     fn set_ip(&mut self, address:String) { self.ip_address = address }
//     fn get_port(&self) -> &String { &self.server_port }
//     fn set_port(&mut self, port:String) { self.server_port = port }
//     fn get_socket(&self) -> &i16{ &self.master_socket_descriptor }
//     fn set_socket(&mut self, socket:i16) { self.master_socket_descriptor = socket}
//     fn get_num_connections(&self) -> &i16 { &self.connected_users }
//     fn set_num_connections(&mut self, users:i16) { self.connected_users = users }
//
//     fn add_registered_user(&mut self, user_info:(String, User)) { self.registered_users.insert(user_info[0], user_info[1]); }
//     fn get_user_details(&self, username:String) -> Option<&User> {
//         if self.registered_users.contains_key(&username) {
//             return self.registered_users[username];
//         } else {
//             return None;
//         }
//     }
//     fn add_new_connection(&mut self, connection:(thread, User)) {
//         self.open_connections.insert(connection[0], connection[1]);
//         self.connected_users += 1;
//     }
//     fn close_connection(&self, connection_thread:thread, socket_descriptor:i16) {
//
//     }
//     fn process_message(&self) {}
//     fn send_all_users(&self) {}
//     fn send_friends(&self) {}
//     fn sent_message(&self) {}
//     fn register_new_user(&self) {}
//     fn login_user(&self) {}
//     fn send_invitation(&self) {}
//     fn send_invite_acceptance(&self) {}
//
//     fn SIGINT_handler(&self, s:i16) {
//         if (s) {
//             println!("SIGINT received, shutting down.");
//             self.shutdown()
//         }
//     }
// }
//
// fn process_connection() {}
//
// #[tokio:main]
// async fn main() {
//     tracing_subscriber::registry()
//     .with(tracing_subscriber::EnvFilter::new(
//         std::env::var("RUST_LOG")
//             .unwrap_or_else(|_| "example_websockets=debug,tower_http=debug".into()),
//     ))
//     .with(tracing_subscriber::fmt::layer())
//     .init();
//
//     // build our application with some routes
//     let app = Router::new()
//         .fallback(
//             get_service(
//                 ServeDir::new("examples/websockets/assets").append_index_html_on_directories(true),
//             )
//                 .handle_error(|error: std::io::Error| async move {
//                     (
//                         StatusCode::INTERNAL_SERVER_ERROR,
//                         format!("Unhandled internal error: {}", error),
//                     )
//                 }),
//         )
//         // routes are matched from bottom to top, so we have to put `nest` at the
//         // top since it matches all routes
//         .route("/ws", get(ws_handler))
//         // logging so we can see whats going on
//         .layer(
//             TraceLayer::new_for_http()
//                 .make_span_with(DefaultMakeSpan::default().include_headers(true)),
//         );
//
//     // run it with hyper
//     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
//     tracing::debug!("listening on {}", addr);
//     axum::Server::bind(&addr)
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
// }
//
// async fn ws_handler(
//     ws: WebSocketUpgrade,
//     user_agent: Option<TypedHeader<headers::UserAgent>>,
// ) -> impl IntoResponse {
//     if let Some(TypedHeader(user_agent)) = user_agent {
//         println!("`{}` connected", user_agent.as_str());
//     }
//
//     ws.on_upgrade(handle_socket)
// }
//
// async fn handle_socket(mut socket: WebSocket) {
//     if let Some(msg) = socket.recv().await {
//         if let Ok(msg) = msg {
//             match msg {
//                 Message::Text(t) => {
//                     println!("client sent str: {:?}", t);
//                 }
//                 Message::Binary(_) => {
//                     println!("client sent binary data");
//                 }
//                 Message::Ping(_) => {
//                     println!("socket ping");
//                 }
//                 Message::Pong(_) => {
//                     println!("socket pong");
//                 }
//                 Message::Close(_) => {
//                     println!("client disconnected");
//                     return;
//                 }
//             }
//         } else {
//             println!("client disconnected");
//             return;
//         }
//     }
//
//     loop {
//         if socket
//             .send(Message::Text(String::from("Hi!")))
//             .await
//             .is_err()
//         {
//             println!("client disconnected");
//             return;
//         }
//         tokio::time::sleep(std::time::Duration::from_secs(3)).await;
//     }
// }