// See: https://github.com/rust-lang/rust/issues/44732#issuecomment-488766871
//
#![cfg_attr( feature = "external_doc", feature(external_doc) )]
#![cfg_attr( feature = "external_doc", doc(include = "../README.md"))]
//!


#![ doc    ( html_root_url = "https://docs.rs/futures_ringbuf" ) ]
#![ deny   ( missing_docs                                      ) ]
#![ forbid ( unsafe_code                                       ) ]
#![ allow  ( clippy::suspicious_else_formatting                ) ]

#![ warn
(
	missing_debug_implementations ,
	nonstandard_style             ,
	rust_2018_idioms              ,
	trivial_casts                 ,
	trivial_numeric_casts         ,
	unused_extern_crates          ,
	unused_qualifications         ,
	single_use_lifetimes          ,
	unreachable_pub               ,
	variant_size_differences      ,
)]


mod ring_buffer ;
mod async_read  ;
mod async_write ;
mod endpoint    ;

pub use self::ring_buffer ::* ;
pub use async_read        ::* ;
pub use async_write       ::* ;
pub use endpoint          ::* ;



// External dependencies
//
mod import
{
	pub(crate) use
	{
		std         :: { io, fmt, pin::Pin, task::{ Context, Poll, Waker } } ,
		ringbuf     :: { RingBuffer as SyncRingBuffer, Producer, Consumer  } ,
		futures     :: { AsyncRead, AsyncWrite, executor::block_on         } ,
		futures::io :: { ReadHalf, WriteHalf, AsyncReadExt, AsyncWriteExt  } ,
	};


	#[ cfg( test ) ]
	//
	pub(crate) use
	{
		pretty_assertions :: { assert_eq                 } ,
		futures_test      :: { task::{ new_count_waker } } ,
	};
}


