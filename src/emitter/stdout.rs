use std::io::Write;

use super::*;

#[derive(Debug)]
pub struct StdoutEmitter {}

impl Default for StdoutEmitter {
    fn default() -> Self {
        Self::new()
    }
}

impl StdoutEmitter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Emitter for StdoutEmitter {
    fn emit_check(
        &mut self,
        _: FormattedFile<'_>,
    ) -> Result<EmitterResult, io::Error> {
        Ok(EmitterResult::default())
    }

    fn emit_formatted_file(
        &mut self,
        FormattedFile { formatted_text, .. }: FormattedFile<'_>,
    ) -> Result<EmitterResult, io::Error> {
        match io::stdout().write_all(formatted_text.as_bytes()) {
            Err(e) => Err(e),
            Ok(_) => Ok(EmitterResult::default()),
        }
    }
}
