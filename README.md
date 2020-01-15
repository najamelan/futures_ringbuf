# futures_ringbuf

[![standard-readme compliant](https://img.shields.io/badge/readme%20style-standard-brightgreen.svg?style=flat-square)](https://github.com/RichardLitt/standard-readme)
[![Build Status](https://api.travis-ci.org/najamelan/futures_ringbuf.svg?branch=master)](https://travis-ci.org/najamelan/futures_ringbuf)
[![Docs](https://docs.rs/futures_ringbuf/badge.svg)](https://docs.rs/futures_ringbuf)
[![crates.io](https://img.shields.io/crates/v/futures_ringbuf.svg)](https://crates.io/crates/futures_ringbuf)


> A ringbuffer that implements AsyncRead/AsyncWrite.

It can be used for testing async network crates cross platform without having to make TCP connections. The crate provides a type `Endpoint` which allows creating both ends of a fake network stream with a ringbuffer in each direction.
It facilitates testing more complex situations like back pressure.

It can also be used as an in memory buffer for communicating between async tasks. I haven't done benchmarks yet.

There are currently 2 versions of the AsyncRead/Write traits. The _futures-rs_ version and the _tokio_ version. This crate implements both behind feature flags: `futures_io` and `tokio`. `futures_io` is enabled by default.

Data in transit is held in an internal RingBuffer from the [ringbuf crate](https://crates.io/crates/ringbuf).

## Table of Contents

- [Install](#install)
   - [Upgrade](#upgrade)
   - [Dependencies](#dependencies)
   - [Security](#security)
- [Usage](#usage)
   - [WASM](#wasm)
   - [Basic Example](#basic-example)
   - [Endpoint Example](#endpoint-example)
- [API](#api)
- [Contributing](#contributing)
   - [Code of Conduct](#code-of-conduct)
- [License](#license)


## Install
With [cargo add](https://github.com/killercup/cargo-edit):
`cargo add futures_ringbuf`

With [cargo yaml](https://gitlab.com/storedbox/cargo-yaml):
```yaml
dependencies:

   futures_ringbuf: ^0.1
```

With raw Cargo.toml
```toml
[dependencies]

    futures_ringbuf = "^0.1"
```

### Upgrade

Please check out the [changelog](https://github.com/najamelan/futures_ringbuf/blob/master/CHANGELOG.md) when upgrading.


### Dependencies

This crate has few dependencies. Cargo will automatically handle it's dependencies for you.

There are no optional features.


### Security

This crate uses `#![ forbid( unsafe_code ) ]`, but it's dependencies use quite some unsafe. On first sight the unsafe usage in `ringbuf` looks sound, but I haven't scrutinized every detail of it and it's not documented.
A lot of unsafe code is present in the futures library, which I haven't reviewed.


## Usage

The crate provides a `RingBuffer<T>` struct which implements `AsyncRead`/`AsyncWrite` from the futures library
when `T` is u8. You can now call `split` provided by `AsyncRead` and treat them as both ends of a network connection.

The reader will return `Poll::Pending` when the buffer is empty, and the writer when the buffer is full. They will
wake each other up when new data/space is available.

If you want to play with `std::io::Read`/`std::io::Write`, check out the `ringbuf` crate directly, as it's `Producer` and
`Consumer` types implement these traits, so I didn't include them here.

I haven't yet included `Stream<T>`, `Sink<T>`, because on `u8` that doesn't make much sense, but if there is demand,
it can definitely be added.

The requirements on `T` are `T: Sized + Copy`.

If you want to seed the buffer before using it with futures_ringbuf, you can use the `Producer` and `Consumer` types of ringbuf. `futures_ringbuf::RingBuffer` implements `From< (Producer<T>, Consumer<T>) >`.


### WASM

This crate works on WASM. See the [integration test](https://github.com/najamelan/futures_ringbuf/tree/master/test/wasm.rs) for WASM for some code.


### Basic example

```rust
//! Frame a RingBuf with futures_codec. This example shows how the sending task will
//! block when the buffer is full. When a reader consumes the buffer, the sender is woken up.
//!
//! Run with `cargo run --example basic`.
//
use
{
   futures_ringbuf :: { *                                            } ,
   futures         :: { SinkExt, StreamExt, executor::block_on, join } ,
   futures_codec   :: { Framed, LinesCodec                           } ,
};


fn main()
{
   let program = async
   {
         let mock = RingBuffer::new( 13 );
         let (mut writer, mut reader) = Framed::new( mock, LinesCodec{} ).split();


         let send_task = async move
         {
            writer.send( "Hello World\n".to_string() ).await.expect( "send" );
            println!( "sent first line" );

            writer.send( "Second line\n".to_string() ).await.expect( "send" );
            println!( "sent second line" );

            writer.close().await.expect( "close sender" );
            println!( "sink closed" );
         };


         let receive_task = async move
         {
            // If we would return here, the second line will never get sent
            // because the buffer is full.
            //
            // return;

            while let Some(msg) = reader.next().await.transpose().expect( "receive message" )
            {
               println!( "Received: {:#?}", msg );
            }
         };


         // Poll them in concurrently
         //
         join!( send_task, receive_task );
   };

   block_on( program );
}
```


### Endpoint

When using one ringbuffer, we get both ends of one connection. If we want a more realistic duplex connection, we
need two ringbuffers, with one endpoint reading from the ringbuffer the other endpoint is writing to. Tasks need
to be woken up correctly when new data or space becomes available... To facilitate this, an `Endpoint` type is provided which will take care of this setup for you.

Note: it wasn't convenient to implement a tokio version in the same type, so a new type is introduced, [TokioEndpoint].


### Endpoint example

```rust
use
{
   futures_ringbuf :: { *                                               } ,
   futures         :: { AsyncWriteExt, AsyncReadExt, executor::block_on } ,
};


fn main() { block_on( async
{
   // Buffer of 10 bytes in each direction. The buffer size always refers to the writing side, so here
   // the first 10 means the server can write 10 bytes before it's buffer is full.
   // When it's full it will return pending on writing and when it's empty it returns
   // pending on reading.
   //
   let (mut server, mut client) = Endpoint::pair( 10, 10 );

   let     data = vec![ 1,2,3 ];
   let mut read = [0u8;3];

   server.write( &data ).await.expect( "write" );

   let n = client.read( &mut read ).await.expect( "read" );

   assert_eq!( n   , 3                 );
   assert_eq!( read, vec![ 1,2,3 ][..] );
})}
```


## API

API documentation can be found on [docs.rs](https://docs.rs/futures_ringbuf).


## Contributing

This repository accepts contributions. Ideas, questions, feature requests and bug reports can be filed through Github issues.

Pull Requests are welcome on Github. By committing pull requests, you accept that your code might be modified and reformatted to fit the project coding style or to improve the implementation. Please discuss what you want to see modified before filing a pull request if you don't want to be doing work that might be rejected.

Please file PR's against the `dev` branch, don't forget to update the changelog and the documentation.

### Testing

`cargo test`

On WASM, after [installing wasm-pack](https://rustwasm.github.io/wasm-pack/):

`wasm-pack test --firefox --headless`

or

`wasm-pack test --chrome --headless`

### Code of conduct

Any of the behaviors described in [point 4 "Unacceptable Behavior" of the Citizens Code of Conduct](http://citizencodeofconduct.org/#unacceptable-behavior) are not welcome here and might get you banned. If anyone including maintainers and moderators of the project fail to respect these/your limits, you are entitled to call them out.

## License

[Unlicence](https://unlicense.org/)

