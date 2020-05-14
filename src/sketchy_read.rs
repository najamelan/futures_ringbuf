use crate::{ import::*, BenevolentDictator };

/// A wrapper for an AsyncRead that will randomly read less data then is available and would fit into the buffer,
/// as well as randomly returning Pending and waking up the task a few ms later.
///
/// The randomness is based on a seed, so that you can reproduce failing builds.
//
#[ derive( Debug ) ]
//
pub struct SketchyRead<T>
{
	inner: T                  ,
	bd   : BenevolentDictator ,
}


impl<T> SketchyRead<T>
{
	/// Create a new wrapper with random behavior based on seed.
	//
	pub fn new( inner: T, seed: u64 ) -> Self
	{
		Self { inner, bd: BenevolentDictator::new( seed ) }
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

		if self.bd.please( "return Partial?", 0.5 )
		{
			let size = self.bd.pick( "buffer size", 0..buf.len() );

			return Pin::new( &mut self.inner ).poll_read( cx, &mut buf[0..size] )
		}

		Pin::new( &mut self.inner ).poll_read( cx, buf )
	}
}
