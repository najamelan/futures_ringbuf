use crate::import::*;


/// A RingBuffer that implements `AsyncRead` and `AsyncWrite` from the futures library.
//
pub struct RingBuffer<T: Sized + Copy>
{
	pub(crate) producer   : Producer<T>   ,
	pub(crate) consumer   : Consumer<T>   ,
	pub(crate) read_waker : Option<Waker> ,
	pub(crate) write_waker: Option<Waker> ,
	pub(crate) closed     : bool          ,
}


impl<T: Sized + Copy> RingBuffer<T>
{
	/// Create a new RingBuffer<T> with a defined capacity. Note that `capacity != length`, similar
	/// to Vec.
	//
	pub fn new( size: usize ) -> Self
	{
		let (producer, consumer) = SyncRingBuffer::new( size ).split();

		Self
		{
			producer            ,
			consumer            ,
			read_waker  : None  ,
			write_waker : None  ,
			closed      : false ,
		}
	}


	/// The total capacity of the buffer
	//
	pub fn capacity( &self ) -> usize
	{
		self.producer.capacity()
	}


	/// Whether there is no data at all in the buffer.
	//
	pub fn is_empty( &self ) -> bool
	{
		self.producer.is_empty()
	}


	/// Whether the buffer is completely full.
	//
	pub fn is_full(&self) -> bool
	{
		self.producer.is_full()
	}


	/// The length of the data in the buffer.
	//
	pub fn len(&self) -> usize
	{
		self.producer.len()
	}


	/// How much free space there is left in the container. On empty, `remaining == capacity`
	//
	pub fn remaining(&self) -> usize
	{
		self.producer.remaining()
	}
}


/// The compiler cannot verify that the producer/consumer are from the same `RingBuffer` object.
/// Obviously if you abuse this things won't work as expected.
///
/// I added this so you can seed a buffer before passing it to futures_ringbuf.
//
impl<T: Sized + Copy> From< (Producer<T>, Consumer<T>) > for RingBuffer<T>
{
	fn from( buffer: (Producer<T>, Consumer<T>) ) -> Self
	{
		let (producer, consumer) = (buffer.0, buffer.1);

		Self
		{
			producer            ,
			consumer            ,
			read_waker  : None  ,
			write_waker : None  ,
			closed      : false ,
		}
	}
}


impl<T: Sized + Copy> From< SyncRingBuffer<T> > for RingBuffer<T>
{
	fn from( buffer: SyncRingBuffer<T> ) -> Self
	{
		let (producer, consumer) = buffer.split();

		Self
		{
			producer            ,
			consumer            ,
			read_waker  : None  ,
			write_waker : None  ,
			closed      : false ,
		}
	}
}


impl<T: Sized + Copy> fmt::Debug for RingBuffer<T>
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		write!( f, "RingBuffer with capacity: {}", self.capacity() )
	}
}
