
/// Identifies the synchronisation of a pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Synchronisation {
    /// Synchronous pipeline.
    Sync,
    /// Asynchronous pipeline.
    Async,
}

/// Pipeline
///
/// A pipeline is a set of stages that are executed in sequence.
///
/// Stages are composed such that the pipeline passes the output
/// of a given stage as the input into another. To avoid tightly
/// coupling stages and restricting 
/// 
/// 

