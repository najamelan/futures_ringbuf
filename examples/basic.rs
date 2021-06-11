//! Frame a RingBuf with futures_codec. This example shows how the sending task will block when the buffer is full.
//! When a reader consumes the buffer, the sender is woken up.
//!
//! Run with `cargo run --example basic`.
//
use
{
	futures_ringbuf    :: { *                        } ,
	futures            :: { SinkExt, StreamExt, join } ,
	asynchronous_codec :: { Framed, LinesCodec       } ,
};


#[async_std::main]
//
async fn main()
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
		// If we would return here, the second line will never get sent because the buffer is full.
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
}


