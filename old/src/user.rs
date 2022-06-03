// pub struct User {
//     username: String,
//     password: String,
//     user_friends: Vec<String>,
//     ip_address: String,
//     client_port: String,
//     client_socket_descriptor: i16,
//     info_string: String,
// }
//
// impl User {
//     fn get_username(&self) -> &String {
//         &self.username
//     }
//
//     fn get_password(&self) -> &String {
//         &self.password
//     }
//
//     fn set_password(&mut self, pwd: String) {
//         self.password = pwd
//     }
//
//     fn get_friends(&self) -> String {
//         self.user_friends.join(";")
//     }
//
//     fn add_new_friend(&mut self, username: String) {
//         let username_vec = vec![username.clone()];
//         self.user_friends.extend(username_vec);
//         self.info_string.push(';');
//         self.info_string.push_str(&username);
//     }
//
//     fn get_ip(&self) -> &String {
//         &self.ip_address
//     }
//
//     fn set_ip(&mut self, ip: String) {
//         self.ip_address = ip
//     }
//
//     fn get_port(&self) -> &String {
//         &self.client_port
//     }
//
//     fn set_port(&mut self, port: String) {
//         self.client_port = port
//     }
//
//     fn get_socket(&self) -> &i16 {
//         &self.client_socket_descriptor
//     }
//
//     fn set_socket(&mut self, socket_descriptor: i16) {
//         self.client_socket_descriptor = socket_descriptor
//     }
//
//     fn get_info_string(&self) -> &String {
//         &self.info_string
//     }
//
//     fn set_info_string(&mut self, string: String) {
//         self.info_string = string
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use crate::user::User;
//
//     // TODO: dedup setup code
//     #[test]
//     fn test_set_password() {
//         let mut test_user = User{
//             username: "test_user".to_string(),
//             password: "test_password".to_string(),
//             user_friends: vec!["friend1".to_string(), "friend2".to_string()],
//             ip_address: "0.0.0.0".to_string(),
//             client_port: "0".to_string(),
//             client_socket_descriptor: -1,
//             info_string: "test_info".to_string(),
//         };
//
//         test_user.set_password("password".to_string());
//         assert_eq!(test_user.password, "password");
//         assert_ne!(test_user.password, "test_password")
//     }
//
//     #[test]
//     fn test_set_ip() {
//         let mut test_user = User{
//             username: "test_user".to_string(),
//             password: "test_password".to_string(),
//             user_friends: vec!["friend1".to_string(), "friend2".to_string()],
//             ip_address: "0.0.0.0".to_string(),
//             client_port: "0".to_string(),
//             client_socket_descriptor: -1,
//             info_string: "test_info".to_string(),
//         };
//
//         test_user.set_ip("0.0.0.1".to_string());
//         assert_eq!(test_user.ip_address, "0.0.0.1");
//         assert_ne!(test_user.ip_address, "0.0.0.0")
//     }
//
//     #[test]
//     fn test_set_port() {
//         let mut test_user = User{
//             username: "test_user".to_string(),
//             password: "test_password".to_string(),
//             user_friends: vec!["friend1".to_string(), "friend2".to_string()],
//             ip_address: "0.0.0.0".to_string(),
//             client_port: "0".to_string(),
//             client_socket_descriptor: -1,
//             info_string: "test_info".to_string(),
//         };
//
//         test_user.set_port("9000".to_string());
//         assert_eq!(test_user.client_port, "9000");
//         assert_ne!(test_user.client_port, "0")
//     }
//
//     #[test]
//     fn test_set_socket() {
//         let mut test_user = User{
//             username: "test_user".to_string(),
//             password: "test_password".to_string(),
//             user_friends: vec!["friend1".to_string(), "friend2".to_string()],
//             ip_address: "0.0.0.0".to_string(),
//             client_port: "0".to_string(),
//             client_socket_descriptor: -1,
//             info_string: "test_info".to_string(),
//         };
//
//         test_user.set_socket(200);
//         assert_eq!(test_user.client_socket_descriptor, 200);
//         assert_ne!(test_user.client_socket_descriptor, -1)
//     }
//
//     #[test]
//     fn test_set_info_string() {
//         let mut test_user = User{
//             username: "test_user".to_string(),
//             password: "test_password".to_string(),
//             user_friends: vec!["friend1".to_string(), "friend2".to_string()],
//             ip_address: "0.0.0.0".to_string(),
//             client_port: "0".to_string(),
//             client_socket_descriptor: -1,
//             info_string: "test_info".to_string(),
//         };
//
//         test_user.set_info_string("username;password".to_string());
//         assert_eq!(test_user.info_string, "username;password");
//         assert_ne!(test_user.info_string, "test_info")
//     }
// }