//! This example demonstrates how you can use tokio_util::compat to use the endpoint with
//! code that uses the tokio AsyncRead and AsyncWrite as interface.
//!
use
{
	futures_ringbuf :: { *                                     } ,
	tokio::io       :: { AsyncWriteExt, AsyncReadExt           } ,
	tokio_util      :: { compat::{ FuturesAsyncReadCompatExt } } ,
	futures         :: { executor::block_on                    } ,
};


fn main() { block_on( async
{
	// Buffer of 10 bytes in each direction.
	// When it's full it will return pending on writing and when it's empty it returns
	// pending on reading.
	//
	let (server, client) = Endpoint::pair( 10, 10 );


	// This does all the magic.
	//
	let mut server = server.compat();
	let mut client = client.compat();

	let     data = vec![ 1,2,3 ];
	let mut read = [0u8;3];

	server.write( &data ).await.expect( "write" );

	let n = client.read( &mut read ).await.expect( "read" );
	assert_eq!( n   , 3                 );
	assert_eq!( read, vec![ 1,2,3 ][..] );
})}
