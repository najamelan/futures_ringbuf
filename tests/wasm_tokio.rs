#![ cfg(all( target_arch="wasm32", feature="tokio" )) ]


// Verify basic functionality on wasm
//
// Tested:
//
// - ✔ the code from the basic example
//
use
{
	wasm_bindgen_test    :: { *                                 } ,
	wasm_bindgen_futures :: { spawn_local                       } ,
	futures_ringbuf      :: { *                                 } ,
	futures              :: { SinkExt, StreamExt, future::ready } ,
	tokio_util::codec    :: { Framed, LinesCodec                } ,
};

wasm_bindgen_test_configure!(run_in_browser);



#[wasm_bindgen_test]
//
fn basic_example_tokio()
{
	let mock = RingBuffer::new( 13 );
	let (mut writer, reader) = Framed::new( mock, LinesCodec::new() ).split();


	let send_task = async move
	{
		writer.send( "Hello World.".to_string() ).await.expect( "send" );
		writer.send( "Second line.".to_string() ).await.expect( "send" );
		writer.close().await.expect( "close sender" );
	};


	let receive_task = async move
	{
		let count = reader.fold( 0, |count, _| ready( count + 1 ) ).await;

		assert_eq!( count, 2 );
	};

	spawn_local( send_task    );
	spawn_local( receive_task );
}


