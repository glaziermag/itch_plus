// use std::io::{self, Read};

// struct ITCHHandler {
//     size: u64,
//     cache: Vec<u8>,
// }

// impl ITCHHandler {
//     pub fn new() -> Self {
//         ITCHHandler {
//             size: 0,
//             cache: Vec::new(),
//         }
//     }

//     pub fn process<R: Read>(&self, mut reader: R) -> io::Result<()> {
//         let mut buffer = [0; 1024]; // Adjust buffer size as needed
//         loop {
//             let size = reader.read(& buffer)?;
//             if size == 0 {
//                 break;
//             }
//             self.process_buffer(&buffer[..size])?;
//         }
//         Ok(())
//     }

//     fn process_buffer(&self, data: &[u8]) -> io::Result<()> {
//         let mut index = 0;
//         while index < data.len() {
//             if self.size == 0 {
//                 let remaining = data.len() - index;
//                 if (self.cache.is_empty() && remaining < 3) || self.cache.len() == 1 {
//                     self.cache.push(data[index]);
//                     index += 1;
//                     continue;
//                 }

//                 let message_size = if self.cache.is_empty() {
//                     // Read the message size directly from the input buffer
//                     read_big_endian(&data[index..])
//                 } else {
//                     // Read the message size from the cache
//                     let message_size = read_big_endian(&self.cache);
//                     self.cache.clear();
//                     message_size
//                 };
//                 self.size = message_size as u64;
//             }

//             if self.size > 0 {
//                 let remaining = data.len() - index;
//                 if !self.cache.is_empty() {
//                     let tail = std::cmp::min(self.size - self.cache.len(), remaining);
//                     self.cache.extend_from_slice(&data[index..index + tail]);
//                     index += tail;
//                     if self.cache.len() < self.size {
//                         continue;
//                     }
//                 } else if self.size > remaining {
//                     self.cache.reserve(self.size);
//                     self.cache.extend_from_slice(&data[index..]);
//                     index = data.len();
//                     continue;
//                 }

//                 if self.cache.is_empty() {
//                     // Process the current message size directly from the input buffer
//                     self.process_message(&data[index..index + self.size])?;
//                     index += self.size;
//                 } else {
//                     // Process the current message size directly from the cache
//                     self.process_message(&self.cache)?;
//                     self.cache.clear();
//                 }
//                 self.size = 0;
//             }
//         }
//         Ok(())
//     }

//     fn process_message(&self, buffer: &[u8]) -> Result<ITCHMessage, &'static str> {
//         match buffer.get(0) {
//             Some(&b'H') => self.process_stock_trading_action_message(buffer),
//             Some(&b'Y') => self.process_reg_sho_message(buffer),
//             // ... other message types
//             Some(_) => Err("Unknown message type"),
//             None => Err("Empty buffer"),
//         }
//     }

//     fn process_stock_trading_action_message(&self, buffer: &[u8]) -> Result<ITCHMessage, &'static str> {
//         if buffer.len() != 25 {
//             return Err("Invalid size for ITCH message type 'H'");
//         }
//         // Process the message
//         // ... 
//         Ok(ITCHMessage::StockTradingAction(StockTradingActionMessage {
//             // set fields here
//         }))
//     }

//     fn process_reg_sho_message(&self, buffer: &[u8]) -> Result<ITCHMessage, &'static str> {
//         if buffer.len() != 20 {
//             return Err("Invalid size for ITCH message type 'Y'");
//         }
//         // Process the message
//         // ... 
//         Ok(ITCHMessage::RegSHO(RegSHOMessage {
//             // set fields here
//         }))
//     }

//     fn process_order_delete_message(&self, buffer: &[u8]) -> Result<OrderDeleteMessage, &'static str> {
//         if buffer.len() != 19 {
//             return Err("Invalid size of the ITCH message type 'D'");
//         }

//         let mut cursor = Cursor::new(buffer);
//         cursor.set_position(1); // Skip message type
//         let stock_locate = cursor.read_u16::<BigEndian>();
//         let tracking_number = cursor.read_u16::<BigEndian>();
//         let timestamp = cursor.read_u64::<BigEndian>();
//         let order_reference_number = cursor.read_u64::<BigEndian>();

//         Ok(OrderDeleteMessage {
//             stock_locate,
//             tracking_number,
//             timestamp,
//             order_reference_number,
//         })
//     }

//     // ... other process_*_message methods ...

//     fn on_message<T>(&self, message: T) -> bool
//     where
//         T: std::fmt::Debug,
//     {
//         println!("{:?}", message);
//         true
//     }
// }
// RESTful
// use warp::Filter;

// #[tokio::main]
// async fn main() {
//     // POST /itch with JSON-encoded ITCH message
//     let itch = warp::post()
//         .and(warp::path("itch"))
//         .and(warp::body::json())
//         .map(|message: Vec<u8>| {
//             match process_itch_message(&message) {
//                 Ok(response) => warp::reply::json(&response),
//                 Err(err) => warp::reply::with_status(
//                     warp::reply::json(&err),
//                     warp::http::StatusCode::BAD_REQUEST,
//                 ),
//             }
//         });

//     // Start the server
//     warp::serve(itch)
//         .run(([127, 0, 0, 1], 3030))
//         .await;
// }

// fn process_itch_message(buffer: &[u8]) -> Result<HashMap<String, String>, &'static str> {
//     // Your ITCH processing logic here

//     // For simplicity, we just return a dummy response
//     Ok(HashMap::from([("message", "Processed successfully".to_string())]))
// }