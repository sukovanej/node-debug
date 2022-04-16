use crate::{source_code::SourceCode, cdt::models::DebuggerPausedCallFrame};

pub fn show_source_code(source_code: &SourceCode, call_frame: &DebuggerPausedCallFrame) -> String {
    let source_mapping = source_code.source_mapping.as_ref().unwrap();
    let source_view = source_mapping.get_source_view(0).unwrap();

    println!("{}", source_mapping.get_file().unwrap());

    let token = source_mapping
        .lookup_token(
            call_frame.location.line_number,
            call_frame.location.column_number,
        )
        .unwrap();
    let line = token.get_src_line();

    source_view
        .source()
        .lines()
        .skip(line as usize - 4)
        .take(9)
        .collect::<Vec<&str>>()
        .join("\n")
}
