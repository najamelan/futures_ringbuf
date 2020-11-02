use crate::{ import::*, RingBuffer };


impl AsyncRead for RingBuffer<u8>
{
	/// Will return Poll::Pending when the buffer is empty. Will be woken up by the AsyncWrite impl when new
	/// data is written or the writer is closed.
	///
	/// When the buffer (for network simulation) is closed and empty, or if you pass in a 0 byte buffer,
	/// this will return `Poll::Ready( Ok(0) )`.
	///
	/// This method is infallible.
	//
	fn poll_read( mut self: Pin<&mut Self>, cx: &mut Context<'_>, dst: &mut [u8] ) -> Poll< Result<usize, io::Error> >
	{
		if dst.len() == 0
		{
			return Poll::Ready( Ok(0) );
		}


		let read = self.consumer.pop_slice( dst );

		if  read != 0
		{
			// If a writer is waiting for place in the buffer, wake them.
			//
			if let Some(waker) = self.write_waker.take()
			{
				waker.wake();
			}

			Poll::Ready( Ok(read) )
		}

		else
		{
			if self.closed
			{
				// Signals end of stream.
				//
				Ok(0).into()
			}

			else
			{
				// Store this waker so that the writer can wake us up after they wrote something.
				//
				self.read_waker.replace( cx.waker().clone() );

				Poll::Pending
			}
		}
	}
}




#[cfg(test)]
//
mod tests
{
	// What's tested:
	//
	// ✔ reading from full
	// ✔ reading from half full
	// ✔ reading from empty buffer
	// ✔ setting the waker
	// ✔ the waker being woken up by a write
	// ✔ reading again after a write on the empty buffer
	// ✔ reading from a closed buffer
	// ✔ reading from a closed empty buffer
	//
	use crate::{ import::{ *, assert_eq }, RingBuffer };

	#[test]
	//
	fn async_read() { block_on( async
	{
		let mut ring = RingBuffer::<u8>::new(2);

		// create a full buffer
		//
		ring.producer.push( b'a' ).expect( "write" );
		ring.producer.push( b'b' ).expect( "write" );


		// read 1
		//
		let mut read_buf = [0u8;1];

		AsyncReadExt::read( &mut ring, &mut read_buf ).await.unwrap();

		assert!( !ring.is_empty() );
		assert!( !ring.is_full()  );

		assert_eq!( ring.len()      , 1 );
		assert_eq!( ring.remaining(), 1 );

		assert!( ring.read_waker .is_none() );
		assert!( ring.write_waker.is_none() );

		assert_eq!( b'a', read_buf[0] );


		// read 2
		//
		AsyncReadExt::read( &mut ring, &mut read_buf ).await.unwrap();

		assert!(  ring.is_empty() );
		assert!( !ring.is_full()  );

		assert_eq!( ring.len()      , 0 );
		assert_eq!( ring.remaining(), 2 );

		assert!( ring.read_waker .is_none() );
		assert!( ring.write_waker.is_none() );

		assert_eq!( b'b', read_buf[0] );


		// read 3
		//
		let (waker, count) = new_count_waker();
		let mut cx = Context::from_waker( &waker );

		assert!( AsyncRead::poll_read( Pin::new( &mut ring ), &mut cx, &mut read_buf ).is_pending() );

		assert!(  ring.is_empty() );
		assert!( !ring.is_full()  );

		assert_eq!( ring.len()      , 0 );
		assert_eq!( ring.remaining(), 2 );

		assert!( ring.read_waker .is_some() );
		assert!( ring.write_waker.is_none() );

		// Write one back, verify read_waker get's woken up and we can read again
		//
		let arr = [ b'c' ];

		AsyncWriteExt::write( &mut ring, &arr ).await.expect( "write" );

		assert!( !ring.is_empty() );
		assert!( !ring.is_full()  );

		assert_eq!( ring.len()      , 1 );
		assert_eq!( ring.remaining(), 1 );

		assert!( ring.read_waker.is_none() );
		assert_eq!( count, 1 );

		assert_eq!( 1, AsyncReadExt::read( &mut ring, &mut read_buf ).await.unwrap() );

		assert_eq!( b'c', read_buf[0] );

		assert!(  ring.is_empty() );
		assert!( !ring.is_full()  );

		assert_eq!( ring.len()      , 0 );
		assert_eq!( ring.remaining(), 2 );
	})}


	#[test]
	//
	fn closed_read() { block_on( async
	{
		let mut ring     = RingBuffer::<u8>::new(2) ;
		let mut read_buf = [0u8;1]                  ;
		let     arr      = [ b'a' ]                 ;

		AsyncWriteExt::write( &mut ring, &arr ).await.expect( "write" );
		ring.close().await.unwrap();

		AsyncReadExt::read( &mut ring, &mut read_buf ).await.unwrap();

		assert_eq!( b'a', read_buf[0] );
		assert_eq!( AsyncReadExt::read( &mut ring, &mut read_buf ).await.unwrap(), 0 );

		// try read again, just in case
		//
		assert_eq!( AsyncReadExt::read( &mut ring, &mut read_buf ).await.unwrap(), 0 );

	})}
}
