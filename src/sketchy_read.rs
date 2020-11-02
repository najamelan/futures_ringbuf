use crate::{ import::*, Dictator };

/// A wrapper for an AsyncRead that will randomly read less data then is available and would fit into the buffer,
/// as well as randomly returning Pending and waking up the task a few ms later.
///
/// The randomness is based on a seed, so that you can reproduce failing tests. In order be reproducible,
/// your test should be deterministic. In general avoid spawning and executor schedulers, prefer `join!`
/// from the futures library to run parts of your test concurrently.
//
#[ derive( Debug ) ]
//
pub struct SketchyRead<T>
{
	inner: T        ,
	bd   : Dictator ,
}


impl<T> SketchyRead<T>
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


impl<T> FutAsyncR for SketchyRead<T>

	where T: FutAsyncR + Unpin
{
	fn poll_read( mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut [u8] ) -> Poll< Result<usize, io::Error> >
	{
		if self.bd.please( "return Pending?", 0.3 )
		{
			cx.waker().wake_by_ref();
			return Poll::Pending;
		}

		// Buffer 0 is an error from the caller and buffer 1 means we are not allowed to make it 0,
		// so no point in running this part.
		//
		if buf.len() > 1 && self.bd.please( "return Partial?", 0.5 )
		{
			// It's important we don't allow zero here, since that usually means that the stream has ended.
			//
			let size = self.bd.pick( "buffer size", 1..buf.len() );

			return Pin::new( &mut self.inner ).poll_read( cx, &mut buf[0..size] )
		}

		Pin::new( &mut self.inner ).poll_read( cx, buf )
	}
}



impl<T> FutAsyncW for SketchyRead<T> where T: FutAsyncW + Unpin
{
	fn poll_write( mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8] ) -> Poll< io::Result<usize> >
	{
		Pin::new( &mut self.inner ).poll_write( cx, buf )
	}


	fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll< io::Result<()> >
	{
		Pin::new( &mut self.inner ).poll_flush( cx )

	}


	fn poll_close( mut self: Pin<&mut Self>, cx: &mut Context<'_> ) -> Poll< io::Result<()> >
	{
		Pin::new( &mut self.inner ).poll_close( cx )
	}
}
