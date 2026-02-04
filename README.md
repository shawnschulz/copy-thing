# copy-thing

Very simple text copy and pasting on a self hosted website using rust actix and html. Not secure to internet (I use behind a vpn to quickly copy stuff) but eh what's the worst that could happen.

Either make a .env file or set the IP_ADDRESS environment variable manually with the address you want to send to clients to fetch and post to. The port will automatically be set to 3790. Then just run cargo to spin up the server.

``` sh
IP_ADDRESS=127.0.0.1
cargo run
```
