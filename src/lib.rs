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
pub use self::ring_buffer ::* ;

#[ cfg( feature = "tokio" ) ] mod async_read_tokio;
#[ cfg( feature = "tokio" ) ] mod async_write_tokio;
#[ cfg( feature = "tokio" ) ] mod endpoint_tokio;

#[ cfg( feature = "tokio" ) ] pub use async_read_tokio::*;
#[ cfg( feature = "tokio" ) ] pub use async_write_tokio::*;
#[ cfg( feature = "tokio" ) ] pub use endpoint_tokio::*;

#[ cfg( feature = "futures_io" ) ] mod async_read;
#[ cfg( feature = "futures_io" ) ] mod async_write;
#[ cfg( feature = "futures_io" ) ] mod endpoint;

#[ cfg( feature = "futures_io" ) ] pub use async_read ::*;
#[ cfg( feature = "futures_io" ) ] pub use async_write::*;
#[ cfg( feature = "futures_io" ) ] pub use endpoint   ::*;



// External dependencies
//
mod import
{
	pub(crate) use
	{
		std         :: { fmt, task::Waker                                  } ,
		ringbuf     :: { RingBuffer as SyncRingBuffer, Producer, Consumer  } ,
	};


	#[ cfg(all( test, any( feature="futures_io", feature="tokio" ) )) ]
	//
	pub(crate) use
	{
		pretty_assertions :: { assert_eq                 } ,
		futures           :: { executor::block_on        } ,
		futures_test      :: { task::{ new_count_waker } } ,
	};


	#[ cfg(any( feature="futures_io", feature="tokio" )) ]
	//
	pub(crate) use
	{
		futures     :: { task::noop_waker                      } ,
		std         :: { io, pin::Pin, task::{ Context, Poll } } ,
	};



	#[ cfg( feature = "tokio" ) ]
	//
	pub(crate) use
	{
		tokio:: { io::{ AsyncRead as TokioAsyncR, AsyncWrite as TokioAsyncW, ReadHalf as TokioReadHalf, WriteHalf as TokioWriteHalf } } ,
	};


	#[ cfg(all( test, feature = "tokio" )) ]
	//
	pub(crate) use
	{
		tokio:: { io::{ AsyncReadExt as TokioARExt, AsyncWriteExt as TokioAWExt } } ,
	};


	#[ cfg( feature = "futures_io" ) ]
	//
	pub(crate) use
	{
		futures     :: { AsyncRead as FutAsyncR, AsyncWrite as FutAsyncW, AsyncReadExt as FutARExt } ,
		futures::io :: { ReadHalf, WriteHalf                                                       } ,
	};


	#[ cfg(all( test, feature = "futures_io" )) ]
	//
	pub(crate) use
	{
		futures:: { AsyncWriteExt as FutAWExt } ,
	};



}


