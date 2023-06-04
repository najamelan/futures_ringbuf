// Tested:
//
// ✔ basic sending and receiving
// ✔ try to read after close
// ✔ try to write after close
// ✔ read remaining data after close
// ✔ wake up pending reader after call to close
// - wake up pending reader after drop (requires an async drop)
//
use
{
	futures_ringbuf   :: { *                                                                      } ,
	tokio_util        :: { codec::{ Framed, LinesCodec }, compat::{ FuturesAsyncReadCompatExt }   } ,
	tokio::io         :: { AsyncRead, AsyncWrite, AsyncWriteExt, AsyncReadExt, ReadBuf            } ,
	futures           :: { future::join, SinkExt, StreamExt, channel::oneshot, executor::block_on } ,
	futures_test      :: { task::noop_waker                                                       } ,
	assert_matches    :: { assert_matches                                                         } ,
	std               :: { task::{ Poll, Context }                                                } ,
	ergo_pin          :: { ergo_pin                                                               } ,
};



#[ test ]
//
fn basic_usage() { block_on( async
{
	let (server, client) = Endpoint::pair( 10, 10 );

	let mut server = server.compat();
	let mut client = client.compat();

	let     data = vec![ 1,2,3 ];
	let mut read = [0u8;3];

	server.write_all( &data ).await.expect( "write" );

	let n = client.read( &mut read ).await.expect( "read" );
	assert_eq!( n   , 3                 );
	assert_eq!( read, vec![ 1,2,3 ][..] );
})}


#[ test ] #[ ergo_pin ]
//
fn close_write()
{
	let (server, client) = Endpoint::pair( 10, 10 );

	let server = server.compat();
	let _client = client.compat();

	let     waker  = noop_waker();
	let mut cx     = Context::from_waker( &waker );
	let mut pserv  = pin!( server );

	let res = pserv.as_mut().poll_shutdown( &mut cx );

	assert_matches!( res, Poll::Ready( Ok(_) ) );

	let buf = vec![ 1,2,3 ];
	let res = pserv.poll_write( &mut cx, &buf );

	match res
	{
		Poll::Ready( Err(e) ) => assert_eq!( e.kind(), std::io::ErrorKind::NotConnected ) ,
		_                     => panic!( "poll_write should return error: {:?}", res ),
	}
}


#[ test ] #[ ergo_pin ]
//
fn close_read()
{
	// flexi_logger::Logger::with_str( "futures_ringbuf=trace" ).start().expect( "flexi_logger");

	let (server, client) = Endpoint::pair( 10, 10 );

	let server = server.compat();
	let client = client.compat();


	let mut pserv  = pin!( server );
	let     pcl    = pin!( client );

	let     waker  = noop_waker();
	let mut cx     = Context::from_waker( &waker );

	// let res = pserv.as_mut().poll_close( &mut cx );
	// assert_matches!( res, Poll::Ready( Ok(_) ) );

	block_on( pserv.as_mut().shutdown() ).expect( "close server" );

	let mut buf      = [0u8;10];
	let mut read_buf = ReadBuf::new( &mut buf );

	let res = pcl.poll_read( &mut cx, &mut read_buf );

	assert_eq!( read_buf.filled().len(), 0 );
	assert_matches!( res, Poll::Ready( Ok(()) ));
}



#[ test ]
//
fn close_read_remaining() { block_on( async
{
	let (server, client) = Endpoint::pair( 10, 10 );

	let mut server = server.compat();
	let mut client = client.compat();


	let     data  = vec![ 1,2,3 ];
	let mut read  = [0u8;3];
	let mut read2 = [0u8;3];

	server.write_all( &data ).await.expect( "write" );
	server.shutdown().await.expect( "close" );

	let n = client.read( &mut read ).await.expect( "read" );
	assert_eq!( n   , 3                 );
	assert_eq!( read, vec![ 1,2,3 ][..] );

	let n = client.read( &mut read2 ).await.expect( "read" );
	assert_eq!( n   , 0        );
	assert_eq!( read2, [0u8;3] );
})}



#[ test ]
//
fn close_wake_pending()
{
	let (server, client) = Endpoint::pair( 10, 10 );

	let server = server.compat();
	let client = client.compat();

	let (sender, receiver) = oneshot::channel::<()>();

	let svr = async move
	{
		let (mut sink, _stream) = Framed::new( server, LinesCodec::new() ).split::<String>();

		receiver.await.expect( "read channel" );

		sink.close().await.expect( "close" );
	};

	let clt = async move
	{
		let (_sink, mut stream) = Framed::new( client, LinesCodec::new() ).split::<String>();

		sender.send(()).expect( "write channel" );

		// This should not hang
		//
		assert!( stream.next().await.is_none() );
	};

	// WARNING: even though we synchronize with the oneshot channel, this test does not hang
	// when it should if the order of clt and svr are reversed here.
	//
	block_on( join( clt, svr ) );
}



#[ test ]
//
fn drop_wake_pending()
{
	let (server, client) = Endpoint::pair( 10, 10 );

	let server = server.compat();
	let mut client = client.compat();

	let (sender, receiver)   = oneshot::channel::<()>();

	let svr = async move
	{
		receiver.await.expect( "read channel" );

		drop( server );
	};

	let clt = async move
	{
		sender.send(()).expect( "write channel" );

		let mut read_buf = [0u8;1];

		// This should not hang
		//
		let result = client.read( &mut read_buf ).await.expect( "Ok(0)" );
		assert_eq!( result, 0 );
	};

	// WARNING: even though we synchronize with the oneshot channel, this test does not hang
	// when it should if the order of clt and svr are reversed here.
	//
	block_on( join( clt, svr ) );
}
