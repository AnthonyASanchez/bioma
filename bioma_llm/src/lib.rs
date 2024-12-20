pub mod chat;
pub mod embeddings;
pub mod indexer;
pub mod markitdown;
pub mod pdf_analyzer;
pub mod rerank;
pub mod retriever;

pub mod prelude {
    pub use crate::chat::{self, Chat, ChatError, ChatMessages};
    pub use crate::embeddings::{
        self, EmbeddingContent, Embeddings, EmbeddingsError, GenerateEmbeddings, GeneratedEmbeddings, StoreEmbeddings, ImageData
    };
    pub use crate::indexer::{self, DeleteSource, DeletedSource, IndexGlobs, Indexer, IndexerError};
    pub use crate::rerank::{self, RankTexts, RankedText, RankedTexts, Rerank, RerankError};
    pub use crate::retriever::{self, RetrieveContext, RetrieveQuery, Retriever, RetrieverError};
    pub use ollama_rs::generation::{
        chat::{ChatMessage, ChatMessageResponse, MessageRole},
        images::Image,
    };
}
