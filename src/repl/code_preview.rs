use crate::cdt::models::DebuggerPausedCallFrame;

use super::source_code::SourceCode;

pub fn create_code_preview(
    source_code: &SourceCode,
    call_frame: &DebuggerPausedCallFrame,
) -> Result<(String, Vec<(usize, String)>), Box<dyn std::error::Error>> {
    let source_mapping = source_code
        .source_mapping
        .as_ref()
        .ok_or("source_mapping not set")?;

    let source_view = source_mapping
        .get_source_view(0)
        .ok_or("source view not found")?;

    let file_name = source_mapping
        .get_file()
        .ok_or("file cannot be obtained from the source map")?
        .to_string();

    let token = source_mapping
        .lookup_token(
            call_frame.location.line_number,
            call_frame.location.column_number,
        )
        .unwrap();

    let line = token.get_src_line();
    let code_preview_lines = source_view
        .source()
        .lines()
        .map(|s| s.to_owned())
        .enumerate()
        .map(|(i, s)| (i + 1, s))
        .skip(line as usize - 4)
        .take(9)
        .collect();

    Ok((file_name, code_preview_lines))
}
