use crate::{ import::*, Dictator };

/// A wrapper for any type that implements `AsyncRead`/`AsyncWrite`, that will randomly return pending and
/// reschedule or only process partial buffers. This helps with testing consumers of these interfaces
/// on in memory objects like a mock network connection created with [Endpoint](crate::Endpoint) which
/// would otherwise always be ready. This simulates a more random behavior you might observe on a real
/// network connection.
///
/// The randomness is based on a seed, so that you can reproduce failing tests. In order be reproducible,
/// your test should be deterministic. In general avoid spawning and executor schedulers, prefer `join!`
/// from the futures library to run parts of your test concurrently.
///
/// # Example
///
/// ```
/// #[test]
/// //
/// fn my_test()
/// {
///    // Make sure to log for failing tests, so you can rerun with the same seed.
///    // futures-ringbuf will log any decisions made by `Sketchy` and will log the
///    // seed.
///    //
///    let _ = flexi_logger::Logger::with_str( "trace" ).start();
///
///    // Since we want to test a random combination of events (pending, partial buffer fills, normal behavior)
///    // let's run this several times.
///    //
///    for _ in 0..500
///    {
///       let seed = Dictator::seed();
///       let (server, client) = Endpoint::pair( 64, 64 );
///       let server = Sketchy::new( server, seed );
///       let client = Sketchy::new( client, seed );
///
///
///       // now use AsyncRead/AsyncWrite on server and client to test your code,
///       // eg. a codec implementation.
///    }
/// }
/// ```
//
#[ derive( Debug ) ]
//
pub struct Sketchy<T>
{
	inner: T        ,
	bd   : Dictator ,
}


impl<T> Sketchy<T>
{
	/// Create a new wrapper with random behavior based on seed.
	//
	pub fn new( inner: T, seed: u64 ) -> Self
	{
		Self
		{
			inner,
			bd: Dictator::new( seed )
		}
	}
}


impl<T> AsyncRead for Sketchy<T>

	where T: AsyncRead + Unpin
{
	/// About one third of the time, this will return pending and reschedule the waker, one third will
	/// only pass a partial buffer to the wrapped type and one third will just forward the call unmodified.
	//
	fn poll_read( mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut [u8] ) -> Poll< Result<usize, io::Error> >
	{
		if self.bd.please( "AsyncRead::poll_read - return Pending?", 0.3 )
		{
			cx.waker().wake_by_ref();
			return Poll::Pending;
		}

		// Buffer 0 is an error from the caller and buffer 1 means we are not allowed to make it 0,
		// so no point in running this part.
		//
		if buf.len() > 1 && self.bd.please( "AsyncRead::poll_read - return Partial?", 0.5 )
		{
			// It's important we don't allow zero here, since that usually means that the stream has ended.
			//
			let size = self.bd.pick( "AsyncRead::poll_read - buffer size", 1..buf.len() );

			return Pin::new( &mut self.inner ).poll_read( cx, &mut buf[0..size] )
		}

		Pin::new( &mut self.inner ).poll_read( cx, buf )
	}
}



impl<T> AsyncWrite for Sketchy<T> where T: AsyncWrite + Unpin
{
	fn poll_write( mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8] ) -> Poll< io::Result<usize> >
	{
		if self.bd.please( "AsyncWrite::poll_write - return Pending?", 0.3 )
		{
			cx.waker().wake_by_ref();
			return Poll::Pending;
		}

		// Buffer 0 is an error from the caller and buffer 1 means we are not allowed to make it 0,
		// so no point in running this part.
		//
		if buf.len() > 1 && self.bd.please( "AsyncWrite::poll_write - return Partial?", 0.5 )
		{
			// It's important we don't allow zero here, since that usually means that the stream has ended.
			//
			let size = self.bd.pick( "AsyncWrite::poll_write - buffer size", 1..buf.len() );

			return Pin::new( &mut self.inner ).poll_write( cx, &buf[0..size] )
		}


		Pin::new( &mut self.inner ).poll_write( cx, buf )
	}


	fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll< io::Result<()> >
	{
		if self.bd.please( "AsyncWrite::poll_flush - return Pending?", 0.5 )
		{
			cx.waker().wake_by_ref();
			return Poll::Pending;
		}

		Pin::new( &mut self.inner ).poll_flush( cx )

	}


	fn poll_close( mut self: Pin<&mut Self>, cx: &mut Context<'_> ) -> Poll< io::Result<()> >
	{
		if self.bd.please( "AsyncWrite::poll_close - return Pending?", 0.5 )
		{
			cx.waker().wake_by_ref();
			return Poll::Pending;
		}

		Pin::new( &mut self.inner ).poll_close( cx )
	}
}
