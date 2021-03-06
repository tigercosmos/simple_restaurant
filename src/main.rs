//! A Simple Restaurant API Server
//!
//! This server will create a TCP listener, accept connections in a loop, and
//! write back everything that's read off of each TCP connection.
//!
//! Because the Tokio runtime uses a thread pool, each TCP connection is
//! processed concurrently with all other TCP connections across multiple
//! threads.
//!
//! To see this server in action, you can run this in one terminal:
//!
//!     cargo run
//!
//! and in another terminal you can run:
//!
//!    telnet 127.0.0.1 8080
//!
//! Each line you type in to the `connect` terminal should be echo'd back to
//! you! If you open up multiple terminals running the `connect` example you
//! should be able to see them all make progress simultaneously.

#![warn(rust_2018_idioms)]

use std::env;
use std::error::Error;
use std::str;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

mod api;
mod item;
mod restaurant;
mod table;

use restaurant::Restaurant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Allow passing an address to listen on as the first argument of this
    // program, but otherwise we'll just set up our TCP listener on
    // 127.0.0.1:8080 for connections.
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    // Next up we create a TCP listener which will listen for incoming
    // connections. This TCP listener is bound to the address we determined
    // above and must be associated with an event loop.
    let listener = TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);

    // create 200 tables for the restaurant
    let restaurant = Restaurant::new(200);

    loop {
        // Asynchronously wait for an inbound socket.
        let (mut socket, _) = listener.accept().await?;

        // And this is where much of the magic of this server happens. We
        // crucially want all clients to make progress concurrently, rather than
        // blocking one on completion of another. To achieve this we use the
        // `tokio::spawn` function to execute the work in the background.
        //
        // Essentially here we're executing a new task to run concurrently,
        // which will allow all of our clients to be processed concurrently.

        let restaurant = restaurant.clone();

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");

                if n == 0 {
                    return;
                }

                let response = request_parser(&mut buf[0..n], restaurant.clone());

                socket
                    .write_all(response.as_bytes())
                    .await
                    .expect("failed to write data to socket");
            }
        });
    }
}

#[derive(PartialEq, Debug)]
enum RequestMethod {
    Get,
    Post,
    Delete,
    Put,
    Unknown,
}

#[derive(PartialEq, Debug)]
enum RequestApi {
    Add,
    Remove,
    Query,
    Unknown,
}

fn parse_method(s: &str) -> RequestMethod {
    match s {
        "GET" => RequestMethod::Get,
        "POST" => RequestMethod::Post,
        "DELETE" => RequestMethod::Delete,
        "PUT" => RequestMethod::Put,
        _ => RequestMethod::Unknown,
    }
}

fn parse_api(s: &str) -> (RequestApi, Vec<&str>) {
    let split = s.split('/');

    let api_vec = split.collect::<Vec<&str>>();

    if api_vec.len() < 2 {
        return (RequestApi::Unknown, vec![]);
    }

    let api_param = if api_vec.len() > 2 {
        api_vec[2..api_vec.len()].to_vec()
    } else {
        vec![]
    };

    match api_vec[1] {
        "add" => (RequestApi::Add, api_param),
        "remove" => (RequestApi::Remove, api_param),
        "query" => (RequestApi::Query, api_param),
        _ => (RequestApi::Unknown, vec![]),
    }
}

fn request_parser(req: &mut [u8], restaurant: Restaurant) -> String {
    let req_str = str::from_utf8(req).unwrap();
    println!("Request: {}", req_str);

    let split = req_str.split(' ');

    let req_vec = split.collect::<Vec<&str>>();

    if req_vec.len() < 2 {
        return "some error".to_string();
    };

    let method = parse_method(req_vec[0]);
    let (api, api_param) = parse_api(req_vec[1]);

    match method {
        RequestMethod::Get => match api {
            RequestApi::Query => match api_param.len() {
                1 => {
                    // TODO: error handling for not a number
                    let tid: u32 = api_param[0].parse::<u32>().unwrap();

                    // `/query/:table_id`
                    return api::query_all(tid, restaurant);
                }
                2 => {
                    let tid: u32 = api_param[0].parse::<u32>().unwrap();
                    let iid: u32 = api_param[1].parse::<u32>().unwrap();

                    // `/query/:table_id/:item_id`
                    return api::query_one(tid, iid, restaurant);
                }
                _ => return "wrong api".to_string(),
            },
            _ => {}
        },
        RequestMethod::Post => match api {
            RequestApi::Add => match api_param.len() {
                2 => {
                    let tid: u32 = api_param[0].parse::<u32>().unwrap();
                    let item_data: &str = api_param[1];

                    // `/add/:table_id/<item>`
                    return api::add_item(tid, item_data, restaurant);
                }
                _ => return "wrong api".to_string(),
            },
            _ => {}
        },
        RequestMethod::Delete => match api {
            RequestApi::Remove => match api_param.len() {
                2 => {
                    let tid: u32 = api_param[0].parse::<u32>().unwrap();
                    let iid: u32 = api_param[1].parse::<u32>().unwrap();

                    // `/romove/:table_id/:item_id`
                    return api::remove_item(tid, iid, restaurant);
                }
                _ => return "wrong api".to_string(),
            },
            _ => {}
        },
        RequestMethod::Put => {}
        _ => {
            return "unknown method".to_string();
        }
    }

    "unknown request".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_parse_method() -> Result<(), String> {
        assert_eq!(parse_method("GET"), RequestMethod::Get);
        assert_eq!(parse_method("POST"), RequestMethod::Post);
        assert_eq!(parse_method("DELETE"), RequestMethod::Delete);
        assert_eq!(parse_method("PUT"), RequestMethod::Put);
        assert_eq!(parse_method("ABC"), RequestMethod::Unknown);
        Ok(())
    }

    #[test]
    fn test_parse_api() -> Result<(), String> {
        assert_eq!(parse_api("/add/xxx"), (RequestApi::Add, vec!["xxx"]));
        assert_eq!(parse_api("/query/xxx"), (RequestApi::Query, vec!["xxx"]));
        assert_eq!(parse_api("/remove/xxx"), (RequestApi::Remove, vec!["xxx"]));
        assert_eq!(
            parse_api("/add/xxx/yyy"),
            (RequestApi::Add, vec!["xxx", "yyy"])
        );
        assert_eq!(
            parse_api("/add/xxx/yyy/"),
            (RequestApi::Add, vec!["xxx", "yyy", ""])
        );
        assert_eq!(parse_api("add"), (RequestApi::Unknown, vec![]));
        assert_eq!(parse_api("add/xxx"), (RequestApi::Unknown, vec![]));
        assert_eq!(parse_api("/"), (RequestApi::Unknown, vec![]));
        assert_eq!(parse_api(""), (RequestApi::Unknown, vec![]));
        Ok(())
    }

    fn get_restaruant_ready(desire_table_id: u32, add_amount: usize) -> Restaurant {
        let restaurant = Restaurant::new(200);

        let mut handles = vec![];

        for test_id in 0..add_amount {
            let restaurant = restaurant.clone();

            let req = format!("POST /add/{}/{}", desire_table_id, test_id);
            let mut bytes: Vec<u8> = req.as_bytes().to_vec();

            let handle = thread::spawn(move || {
                let _res = request_parser(&mut bytes, restaurant.clone());
            });

            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }

        restaurant
    }

    #[test]
    fn integration_test_add_item() -> Result<(), String> {
        let desire_table_id = 0;
        let add_amount = 100;
        let restaurant = get_restaruant_ready(desire_table_id, add_amount);

        let t = restaurant.get_table(desire_table_id);
        let len = t.lock().unwrap().items_size();
        assert_eq!(len, add_amount);

        Ok(())
    }

    #[test]
    fn integration_test_remove_item() -> Result<(), String> {
        let desire_table_id = 0;
        let add_amount = 100;
        let remove_amount = 76;
        let restaurant = get_restaruant_ready(desire_table_id, add_amount);

        let mut handles = vec![];

        for test_id in 0..remove_amount {
            let restaurant = restaurant.clone();

            let req = format!("DELETE /remove/{}/{}", desire_table_id, test_id);
            let mut bytes: Vec<u8> = req.as_bytes().to_vec();

            let handle = thread::spawn(move || {
                let _res = request_parser(&mut bytes, restaurant.clone());
                println!("{}", _res);
            });

            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }

        let t = restaurant.get_table(desire_table_id);
        let len = t.lock().unwrap().items_size();
        assert_eq!(len, add_amount - remove_amount);

        Ok(())
    }

    #[test]
    fn integration_test_check_item() -> Result<(), String> {
        let desire_table_id = 0;
        let add_amount = 20;
        let restaurant = get_restaruant_ready(desire_table_id, add_amount);

        let mut handles = vec![];

        for test_id in 0..add_amount {
            let restaurant = restaurant.clone();

            let req = format!("GET /query/{}/{}", desire_table_id, test_id);
            let mut bytes: Vec<u8> = req.as_bytes().to_vec();

            let handle = thread::spawn(move || {
                let res = request_parser(&mut bytes, restaurant.clone());
                let s = format!("\"item_id\": {}", test_id);
                assert_eq!(res.contains(&s), true);
            });

            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }

        Ok(())
    }

    #[test]
    fn integration_test_check_all_item() -> Result<(), String> {
        let desire_table_id = 0;
        let add_amount = 20;
        let remove_amount = 17;
        let restaurant = get_restaruant_ready(desire_table_id, add_amount);

        let mut handles = vec![];

        for test_id in 0..remove_amount {
            let restaurant = restaurant.clone();

            let req = format!("DELETE /remove/{}/{}", desire_table_id, test_id);
            let mut bytes: Vec<u8> = req.as_bytes().to_vec();

            let handle = thread::spawn(move || {
                let _res = request_parser(&mut bytes, restaurant.clone());
                println!("{}", _res);
            });

            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }

        // should only have 17 18 19
        {
            let restaurant = restaurant.clone();

            let req = format!("GET /query/{}", desire_table_id);
            let mut bytes: Vec<u8> = req.as_bytes().to_vec();

            let _ = thread::spawn(move || {
                let res = request_parser(&mut bytes, restaurant.clone());
                let s0 = "\"item_id\": 16";
                let s1 = "\"item_id\": 17";
                let s2 = "\"item_id\": 18";
                let s3 = "\"item_id\": 19";
                let s4 = "\"item_id\": 20";
                assert_eq!(res.contains(&s0), false);
                assert_eq!(res.contains(&s1), true);
                assert_eq!(res.contains(&s2), true);
                assert_eq!(res.contains(&s3), true);
                assert_eq!(res.contains(&s4), false);
            });
        }

        Ok(())
    }
}
