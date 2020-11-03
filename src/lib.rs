#![ cfg_attr( nightly, feature( external_doc, doc_cfg    ) ) ]
#![ cfg_attr( nightly, doc    ( include = "../README.md" ) ) ]
#![ doc = "" ] // empty doc line to handle missing doc warning when the feature is missing.

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

mod async_read;
mod async_write;
mod endpoint;

pub use async_read ::*;
pub use async_write::*;
pub use endpoint   ::*;

#[ cfg( feature = "sketchy" ) ] mod dictator;
#[ cfg( feature = "sketchy" ) ] mod sketchy;
#[ cfg( feature = "sketchy" ) ] pub use dictator::*;
#[ cfg( feature = "sketchy" ) ] pub use sketchy::*;


// External dependencies
//
mod import
{
	pub(crate) use
	{
		std         :: { fmt, task::Waker                                 } ,
		ringbuf     :: { RingBuffer as SyncRingBuffer, Producer, Consumer } ,
		futures     :: { AsyncRead, AsyncWrite, AsyncReadExt              } ,
		futures::io :: { ReadHalf, WriteHalf                              } ,
		futures     :: { task::noop_waker                                 } ,
		std         :: { io, pin::Pin, task::{ Context, Poll }            } ,
	};


	#[ cfg(test) ]
	//
	pub(crate) use
	{
		futures           :: { AsyncWriteExt             } ,
		pretty_assertions :: { assert_eq                 } ,
		futures           :: { executor::block_on        } ,
		futures_test      :: { task::{ new_count_waker } } ,
	};
}


