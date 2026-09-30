pub struct WalkOptions {
    pub max_depth: Option<usize>,
    pub show_hidden: bool,
    pub dirs_only: bool,
    pub files_only: bool,
    pub extensions: Vec<String>,
    pub ignore: Vec<String>,
    pub use_gitignore: bool,
}
