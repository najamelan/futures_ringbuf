use crate::{ import::*, RingBuffer };



impl AsyncWrite for RingBuffer<u8>
{
	/// Will return Poll::Pending when the buffer is full. AsyncRead impl will wake up this task
	/// when new place is made.
	/// This method returns a `io::ErrorKind::NotConnected` error if called after `poll_close`.
	//
	fn poll_write( mut self: Pin<&mut Self>, cx: &mut Context<'_>, src: &[u8] ) -> Poll< Result<usize, io::Error> >
	{
		if self.closed { return Err( io::ErrorKind::NotConnected.into() ).into() }

		let wrote = self.producer.push_slice( src );

		if wrote != 0
		{
			// If a reader is waiting for data, now that we wrote, wake them up.
			//
			if let Some(waker) = self.read_waker.take()
			{
				waker.wake();
			}

			Ok(wrote).into()
		}


		else
		{
			// If the buffer is full, store our waker so readers can wake us up when they have consumed some data.
			//
			self.write_waker.replace( cx.waker().clone() );

			Poll::Pending
		}
	}


	/// We are always flushed, this is a noop.
	/// This method is infallible.
	//
	fn poll_flush( self: Pin<&mut Self>, _cx: &mut Context<'_> ) -> Poll< Result<(), io::Error> >
	{
		Ok(()).into()
	}


	/// Closes the stream. After this no more data can be send into it.
	/// This method is infallible.
	//
	fn poll_close( mut self: Pin<&mut Self>, _cx: &mut Context<'_> ) -> Poll< Result<(), io::Error> >
	{
		self.closed = true;

		// If a reader is waiting for data, now that we wrote, wake them up.
		//
		if let Some(waker) = self.read_waker.take()
		{
			waker.wake();
		}

		Ok(()).into()
	}
}


#[cfg(test)]
//
mod tests
{
	// What's tested:
	//
	// ✔ writing to empty buffer
	// ✔ writing to half full
	// ✔ writing to full
	// ✔ setting the waker
	// ✔ the waker being woken up by a read
	// ✔ the waker from a reader is woken up when closing the writer
	// ✔ writing again after a read on the full buffer
	// ✔ writing to a closed buffer
	//
	use crate::{ import::{ *, assert_eq }, RingBuffer };

	#[test]
	//
	fn async_write() { block_on( async
	{
		let mut ring = RingBuffer::<u8>::new(2);

		assert!(  ring.is_empty() );
		assert!( !ring.is_full()  );

		assert_eq!( ring.len()      , 0 );
		assert_eq!( ring.remaining(), 2 );

		assert!( ring.read_waker .is_none() );
		assert!( ring.write_waker.is_none() );


		// write 1
		//
		let arr = [ b'a' ];
		AsyncWriteExt::write( &mut ring, &arr ).await.expect( "write" );

		assert!( !ring.is_empty() );
		assert!( !ring.is_full()  );

		assert_eq!( ring.len()      , 1 );
		assert_eq!( ring.remaining(), 1 );

		assert!( ring.read_waker .is_none() );
		assert!( ring.write_waker.is_none() );

		assert_eq!( b'a', ring.consumer.pop().unwrap() );


		// write 2
		//
		let arr = [ b'b' ];
		AsyncWriteExt::write( &mut ring, &arr ).await.expect( "write" );

		let arr = [ b'c' ];
		AsyncWriteExt::write( &mut ring, &arr ).await.expect( "write" );

		assert!( !ring.is_empty() );
		assert!(  ring.is_full()  );

		assert_eq!( ring.len()      , 2 );
		assert_eq!( ring.remaining(), 0 );

		assert!( ring.read_waker .is_none() );
		assert!( ring.write_waker.is_none() );

		assert_eq!( b'b', ring.consumer.pop().unwrap() );
		assert_eq!( b'c', ring.consumer.pop().unwrap() );


		// write 3
		//
		let arr = [ b'd' ];
		AsyncWriteExt::write( &mut ring, &arr ).await.expect( "write" );

		let arr = [ b'e' ];
		AsyncWriteExt::write( &mut ring, &arr ).await.expect( "write" );


		let (waker, count) = new_count_waker();
		let mut cx = Context::from_waker( &waker );

		let arr = [ b'f' ];
		assert!( AsyncWrite::poll_write( Pin::new( &mut ring ), &mut cx, &arr ).is_pending() );

		assert!( !ring.is_empty() );
		assert!(  ring.is_full()  );

		assert_eq!( ring.len()      , 2 );
		assert_eq!( ring.remaining(), 0 );

		assert!( ring.write_waker.is_some() );

		// Pop 1 and try writing again
		//
		let mut read_buf = [0u8;1];
		assert_eq!( 1, AsyncReadExt::read( &mut ring, &mut read_buf ).await.unwrap() );

		assert_eq!( b'd', read_buf[0] );

		assert!( ring.write_waker.is_none() );
		assert_eq!( count, 1 );

		assert!( !ring.is_empty() );
		assert!( !ring.is_full()  );

		assert_eq!( ring.len()      , 1 );
		assert_eq!( ring.remaining(), 1 );


		AsyncWriteExt::write( &mut ring, &arr ).await.expect( "write" );

		assert!( !ring.is_empty() );
		assert!(  ring.is_full()  );

		assert_eq!( ring.len()      , 2 );
		assert_eq!( ring.remaining(), 0 );

	})}



	#[test]
	//
	fn close_wake_reader() { block_on( async
	{
		let mut ring = RingBuffer::<u8>::new(2);

		let (waker, _count) = new_count_waker();
		let mut cx = Context::from_waker( &waker );

		let mut read_buf = [0u8;1];

		assert!( AsyncRead::poll_read( Pin::new( &mut ring ), &mut cx, &mut read_buf ).is_pending() );

		assert!( ring.read_waker .is_some() );
		assert!( ring.write_waker.is_none() );

		ring.close().await.expect( "close" );

		assert!( ring.read_waker.is_none() );
	})}



	#[test]
	//
	fn closed_write() { block_on( async
	{
		let mut ring = RingBuffer::<u8>::new(2);

		ring.close().await.unwrap();

		let arr = [ b'a' ];
		assert_eq!( AsyncWriteExt::write( &mut ring, &arr ).await.unwrap_err().kind(), io::ErrorKind::NotConnected );

		// Should be the same
		//
		assert_eq!( AsyncWriteExt::write( &mut ring, &arr ).await.unwrap_err().kind(), io::ErrorKind::NotConnected );
	})}
}
