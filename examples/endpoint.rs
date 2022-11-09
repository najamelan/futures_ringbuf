use
{
	futures_ringbuf :: { *                           } ,
	futures         :: { AsyncWriteExt, AsyncReadExt } ,
};


#[async_std::main]
//
async fn main()
{
	// Buffer of 10 bytes in each direction.
	// When it's full it will return pending on writing and when it's empty it returns
	// pending on reading.
	//
	let (mut server, mut client) = Endpoint::pair( 10, 10 );

	let     data = vec![ 1,2,3 ];
	let mut read = [0u8;3];

	server.write_all( &data ).await.expect( "write" );

	let n = client.read( &mut read ).await.expect( "read" );
	assert_eq!( n   , 3                 );
	assert_eq!( read, vec![ 1,2,3 ][..] );
}
