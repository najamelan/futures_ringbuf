// See: https://github.com/rust-lang/rust/issues/44732#issuecomment-488766871
//
#![cfg_attr( feature = "external_doc", feature(external_doc) )]
#![cfg_attr( feature = "external_doc", doc(include = "../README.md"))]
//!


#![ doc    ( html_root_url = "https://docs.rs/futures_ringbuf" ) ]
#![ feature( async_await                                       ) ]
#![ deny   ( missing_docs                                      ) ]
#![ forbid ( unsafe_code                                       ) ]
#![ allow  ( clippy::suspicious_else_formatting                ) ]


mod ring_buffer ;
mod async_read  ;
mod async_write ;

pub use self::ring_buffer ::* ;
pub use async_read        ::* ;
pub use async_write       ::* ;



// External dependencies
//
mod import
{
	pub(crate) use
	{
		std     :: { io, fmt, pin::Pin, task::{ Context, Poll, Waker } } ,
		ringbuf :: { RingBuffer as SyncRingBuffer, Producer, Consumer  } ,
		futures :: { AsyncRead, AsyncWrite                             } ,
	};


	#[ cfg( test ) ]
	//
	pub(crate) use
	{
		pretty_assertions :: { assert_eq                                       } ,
		futures           :: { AsyncReadExt, AsyncWriteExt, executor::block_on } ,
		futures_test      :: { task::{ new_count_waker }                       } ,
	};
}


