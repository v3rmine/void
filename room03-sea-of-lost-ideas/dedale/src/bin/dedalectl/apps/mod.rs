use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[clap(
    visible_alias = "app", 
    long_about,
    verbatim_doc_comment,
)]
/// The APPS commands focus on managing your Dedale applications. Start with the CREATE command to register your application.
/// The LIST command will list all currently registered applications.
pub enum Command {
    /// Create a new application
    Create,
    /// Permanently destroys an app
    Destroy,
    /// List applications
    List,
}