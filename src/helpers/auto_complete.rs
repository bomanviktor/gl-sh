use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Helper};

pub struct CustomHelper {
    pub filename_completer: FilenameCompleter,
}

impl Completer for CustomHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Self::Candidate>), ReadlineError> {
        self.filename_completer.complete(line, pos, ctx)
    }
}

impl Helper for CustomHelper {}
impl Hinter for CustomHelper {
    type Hint = String;

    fn hint(&self, _line: &str, _pos: usize, _ctx: &Context<'_>) -> Option<String> {
        // No hints are provided
        None
    }
}

impl Highlighter for CustomHelper {}

impl Validator for CustomHelper {}
