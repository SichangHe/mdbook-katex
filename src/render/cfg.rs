//! Extra configurations for pre-rendering KaTeX.
use katex::{macros::MacroDefinition, OutputFormat, Settings, TrustSetting};

use super::*;

impl KatexConfig {
    /// Configured output type.
    /// Defaults to `Html`, can also be `Mathml` or `HtmlAndMathml`.
    pub fn output_type(&self) -> OutputFormat {
        match self.output.as_str() {
            "html" => OutputFormat::Html,
            "mathml" => OutputFormat::Mathml,
            "htmlAndMathml" => OutputFormat::HtmlAndMathml,
            other => {
                error!(
"[preprocessor.katex]: `{other}` is not a valid choice for `output`! Please check your `book.toml`.
Defaulting to `html`. Other valid choices for output are `mathml` and `htmlAndMathml`."
                );
                OutputFormat::Html
            }
        }
    }

    /// From `root`, load macros as a `HashMap`.
    pub fn load_macros<P>(&self, root: P) -> HashMap<String, String>
    where
        P: AsRef<Path>,
    {
        load_macros(root, &self.macros)
    }

    /// Given `macros`, generate `(inline_opts, display_opts)`.
    /// `Settings` is not `Sync`, so each thread needs its own.
    pub fn build_opts_from_macros(&self, macros: &HashMap<String, String>) -> (Settings, Settings) {
        let build = |display_mode| {
            Settings::builder()
                .display_mode(display_mode)
                .output(self.output_type())
                .leqno(self.leqno)
                .fleqn(self.fleqn)
                .throw_on_error(self.throw_on_error)
                .error_color(self.error_color.clone())
                .macros(
                    macros
                        .iter()
                        .map(|(name, body)| (name.clone(), MacroDefinition::String(body.clone())))
                        .collect(),
                )
                .min_rule_thickness(self.min_rule_thickness)
                .max_size(self.max_size)
                .max_expand(self.max_expand)
                .trust(TrustSetting::Bool(self.trust))
                .build()
        };
        (build(false), build(true))
    }
}

/// Load macros from `root`/`macros_path` into a `HashMap`.
fn load_macros<P>(root: P, macros_path: &Option<String>) -> HashMap<String, String>
where
    P: AsRef<Path>,
{
    // load macros as a HashMap
    let mut map = HashMap::new();
    if let Some(path) = get_macro_path(root, macros_path) {
        let macro_str = load_as_string(&path);
        for couple in macro_str.split('\n') {
            // only consider lines starting with a backslash
            if let Some('\\') = couple.chars().next() {
                let couple: Vec<&str> = couple.splitn(2, ':').collect();
                map.insert(String::from(couple[0]), String::from(couple[1]));
            }
        }
    }
    map
}

/// Absolute path of the macro file.
pub fn get_macro_path<P>(root: P, macros_path: &Option<String>) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    macros_path
        .as_ref()
        .map(|path| root.as_ref().join(PathBuf::from(path)))
}

/// Read file at `path`.
pub fn load_as_string(path: &Path) -> String {
    let display = path.display();

    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {display}: {why}"),
        Ok(file) => file,
    };

    let mut string = String::new();
    if let Err(why) = file.read_to_string(&mut string) {
        panic!("couldn't read {display}: {why}")
    };
    string
}
