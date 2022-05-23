# Simple Restaurant API Server

This is the [Paidy assignment](https://github.com/paidy/interview/blob/1c28b4c/SimpleRestaurantApi.md).

## Build and Run

You need to have Rust and Cargo installed.

Run the server:

```
$ cargo run
```

Run the unittest:

```
$ cargo test
```

## Test with Clients

Run the server

```
$ cargo run
```

Open another shell and run the client:

```
$ python3 client/client.py
```

## API Design

- `POST /add/:table_id/<item>`: add an item on the certain table, currently `item` format is `item_id`, and it could be `item_id,name,favor,...` in the future
- `DELETE /romove/:table_id/:item_id` delete the certain item on the certain table
- `GET /query/:table_id/:item_id`: check if the certain item on the certain table
- `GET /query/:table_id`: show all items on the certain table

## License

MIT
