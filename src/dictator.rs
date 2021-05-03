use
{
	rand        :: { Rng, RngCore, thread_rng, SeedableRng, distributions::uniform::SampleUniform } ,
	rand_chacha :: { ChaCha8Rng                                                                   } ,
	std         :: { ops::Range, fmt                                                              } ,
	log         :: { *                                                                            } ,
};


/// Dictator that makes random decisions based on a seed. That is the decisions are
/// reproducible. For reproducible decisions, your use of the dictator must be deterministic.
//
#[ derive( Debug ) ]
//
pub struct Dictator
{
	seed: u64,
	rng : ChaCha8Rng,
}


impl Dictator
{
	/// Birth place of all dictators. This method will log the seed with log::trace.
	/// Make sure you turn on logging for the futures_ringbuf crate so you can reproduce failing tests.
	//
	pub fn new( seed: u64 ) -> Self
	{
		trace!( "Creating new dictator with seed {}", seed );

		Self
		{
			seed,
			rng: ChaCha8Rng::seed_from_u64( seed ),
		}
	}

	/// Ask the dictator permission to do something.
	//
	pub fn please( &mut self, question: &str, prob: f64 ) -> bool
	{
		let answer = self.rng.gen_bool( prob );

		trace!( "dictator please {}, answer: {}", question, answer );

		answer
	}

	/// Ask the dictator to pick from a range of values.
	//
	pub fn pick<Idx: SampleUniform + fmt::Debug + Copy + PartialOrd>( &mut self, what: &str, range: Range<Idx> ) -> Idx
	{
		let pick = self.rng.gen_range( range.clone() );

		trace!( "dictator pick {} from {:?}, answer: {:?}", what, range, pick );

		pick
	}


	/// Create a new random seed from entropy.
	//
	pub fn seed() -> u64
	{
		thread_rng().next_u64()
	}
}


#[ cfg(test) ]
//
mod tests
{
	use super::*;

	#[test]
	//
	fn predictable()
	{
		let mut bd = Dictator::new( 265468510 );

		assert!( !bd.please( "one"  , 0.4 ) );
		assert!( !bd.please( "two"  , 0.6 ) );
		assert!( !bd.please( "three", 0.5 ) );
		assert!( !bd.please( "four" , 0.3 ) );
		assert!(  bd.please( "five" , 0.9 ) );
		assert!( !bd.please( "six"  , 0.2 ) );
		assert!( !bd.please( "seven", 0.5 ) );
		assert!(  bd.please( "eight", 0.4 ) );

		assert_eq!( 1 , bd.pick( "nine"  , 0..100 ) );
		assert_eq!( 94, bd.pick( "ten"   , 0..100 ) );
		assert_eq!( 46, bd.pick( "eleven", 0..100 ) );

		assert!(  bd.please( "twelve"  , 0.6 ) );
		assert!(  bd.please( "thirteen", 0.5 ) );
		assert!(  bd.please( "fourteen", 0.5 ) );
		assert!(  bd.please( "fifteen" , 0.5 ) );
		assert!( !bd.please( "sixteen" , 0.5 ) );
	}
}
