# Rex Chat
This is a chat application which included a server and client.\
Each client has its own public and private key.\
Server authenticates clients and handles the forwarding of messages.

## RP (Rex Protocol)
A vey simple protocol on TCP which is used to communicate with the server and client.
The protocol uses blocksizes of 64 bytes.

Init Message:
client -> server
used for connecting to the server
```
I {USERNAME}
```

Say Message:
client <-> server
```
S {USERNAME} {MESSAGE}
```

Error Message:
client <-> server
```
E {MESSAGE}
```

Disconnect Message:
client -> server
```
C {REASON}
```

## Usage
```
cargo build --release
./target/release/rchat
```

Which will show your help menu:
```
Rex Chat
Lyr-7D1h <lyr-7d1h@pm.me>

Usage:
    rchat <(client|c)|(server|s)>
```