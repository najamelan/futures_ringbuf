use crate::{ import::*, RingBuffer };

/// Represents a network endpoint. This is for duplex connection mocking. Each direction has a separate
/// ringbuffer and one buffer's readhalf is connected to the other buffer's writehalf in order to simulate
/// a duplex connection.
///
/// The main way to create this is to call `Endpoint::pair` which returns a tuple of endpoints, and which
/// let's you specify the buffer size for each direction.
///
/// Endpoint implements AsyncRead/AsyncWrite so you can feed it to interfaces that need those combined in
/// a single type.
///
/// By setting the buffer size precisely, one can simulate back pressure. Endpoint will return Pending on writes
/// when full and on reads when empty.
///
/// When calling `shutdown` on an endpoint, any further writes on that endpoint will return [`std::io::ErrorKind::NotConnected`]
/// and any reads on the other endpoint will continue to empty the buffer and then return `Ok(0)`. `Ok(0)` means
/// no new data will ever appear, unless you passed in a zero sized buffer.
///
/// If the remote endpoint is pending on a read, the task will be woken up when calling `shutdown` or dropping
/// this endpoint.
//
#[ cfg_attr( nightly, doc(cfg( feature = "tokio" )) ) ]
//
#[ derive( Debug ) ]
//
pub struct TokioEndpoint
{
	writer: TokioWriteHalf< RingBuffer<u8> >,
	reader: TokioReadHalf < RingBuffer<u8> >,
}


impl TokioEndpoint
{
	/// Create a pair of endpoints, specifying the buffer size for each one. The buffer size corresponds
	/// to the buffer the respective endpoint writes to. The other will read from this one.
	///
	/// Tokio version.
	//
	pub fn pair( a_buf: usize, b_buf: usize ) -> (Self, Self)
	{
		let ab_buf = RingBuffer::<u8>::new( a_buf );
		let ba_buf = RingBuffer::<u8>::new( b_buf );

		let (ab_reader, ab_writer) = tokio::io::split( ab_buf );
		let (ba_reader, ba_writer) = tokio::io::split( ba_buf );

		(
			Self{ writer: ab_writer, reader: ba_reader },
			Self{ writer: ba_writer, reader: ab_reader },
		)
	}
}


impl TokioAsyncR for TokioEndpoint
{
	fn poll_read( mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut [u8] ) -> Poll< io::Result<usize> >
	{
		Pin::new( &mut self.reader ).poll_read( cx, buf )
	}
}


impl TokioAsyncW for TokioEndpoint
{
	fn poll_write( mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8] ) -> Poll< io::Result<usize> >
	{
		Pin::new( &mut self.writer ).poll_write( cx, buf )
	}


	fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll< io::Result<()> >
	{
		Pin::new( &mut self.writer ).poll_flush( cx )

	}


	fn poll_shutdown( mut self: Pin<&mut Self>, cx: &mut Context<'_> ) -> Poll< io::Result<()> >
	{
		Pin::new( &mut self.writer ).poll_shutdown( cx )
	}
}


/// Makes sure that the reader of the other end is notified (woken up if pending) of the connection closure.
//
impl Drop for TokioEndpoint
{
	fn drop( &mut self )
	{
		// closing the ringbuffer is not an async operation. It always returns immediately,
		// so we can fake an async operation. Note that block_on is not an option, since
		// that can't be nested and client code might use it as well.
		//
		let waker  = noop_waker();
		let mut cx = Context::from_waker( &waker );

		let _ = Pin::new( self ).poll_shutdown( &mut cx);
	}
}
