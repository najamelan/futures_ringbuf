# futures_ringbuf

> A fake network stream for testing and examples without having to do tcp.

The futures_ringbuf type implements:
- `std::io::Read`
- `std::io::Write`

with the "futures" feature enabled:
- `futures::sink::Sink<u8>`
- `futures::sink::Sink<AsRef<[u8]>>`
- `futures::sink::Stream<u8>`
- `futures::sink::Stream<Vec<[u8]>>`
- `futures::io::AsyncRead`
- `futures::io::AsyncWrite`

It acts like both ends of a network connection. You can think of it like the network connection to an echo server. You can frame it with futures_codec, split it to get both halves separately and send and receive messages on it. You can use this in testing and examples to avoid having to make actual tcp connections.

Data in transit is held in an internal BytesMut.


## Usage

```rust
// This will re-export all required traits
//
use futures_ringbuf::*;

// Use with_capacity to enable back pressure
//
let mock          = ByteStream::with_capacity( 1024 );
let data: Vec<u8> = vec![ 1, 2, 3 ];

// From futures AsyncWrite
//
mock.send( &data );
mock.close();

assert_eq!( &data, mock.next().unwrap().unwrap() );
assert_eq!( None, mock.next() );
```
