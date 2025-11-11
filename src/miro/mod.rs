pub mod types;

#[cfg(feature = "stdio-mcp")]
pub mod builders;
#[cfg(feature = "stdio-mcp")]
pub mod client;

pub use types::{Board, BoardsResponse, CreateBoardRequest, CreateBoardResponse};

#[cfg(feature = "stdio-mcp")]
pub use builders::{ConnectorBuilder, ShapeBuilder, StickyNoteBuilder, TextBuilder};
#[cfg(feature = "stdio-mcp")]
pub use client::{MiroClient, MiroError};
