use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[clap(
    visible_aliases = ["machines", "m"], 
    verbatim_doc_comment,
)]
/// Manage Dedale Machines. 
/// Dedale Machines are super-slow, or at least not lighting fast VMs that can be created, and then "quickly" started and stopped as needed with dedalectl commands or with the Machines REST dedale.
pub enum Command {
    /// Create, but don't start, a machine
    Create,
    /// List Dedale machines
    List,
    /// Destroy Dedale machines
    Destroy,
    /// Start one or more Dedale machines
    Start,
    /// Stop one or more Dedale machines
    Stop,
}