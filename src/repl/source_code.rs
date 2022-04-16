use sourcemap::SourceMap;

#[derive(Debug)]
pub struct SourceCode {
    pub code: String,
    pub source_mapping: Option<SourceMap>,
}

static MAPPING_START: &'static str = "//# sourceMappingURL=";
static BASE64_CODING: &'static str = "data:application/json;charset=utf-8;base64";

impl SourceCode {
    pub fn from_str(input: &str) -> SourceCode {
        let lines = input.lines().collect::<Vec<&str>>();
        let lines = lines.split_last();

        if lines.is_none() {
            return SourceCode {
                code: input.to_owned(),
                source_mapping: None,
            };
        }

        let (source_mapping, code) = lines.unwrap();
        let code = code.join("\n");
        let base64_mapping_start = format!("{}{},", MAPPING_START, BASE64_CODING);

        if source_mapping.starts_with(&base64_mapping_start) {
            let base64_input = source_mapping.replace(&base64_mapping_start, "");
            let maybe_decoded_mapping = try_decode_mapping(&base64_input)
                .and_then(|x| SourceMap::from_slice(x.as_bytes()).ok());

            return SourceCode {
                code,
                source_mapping: maybe_decoded_mapping,
            };
        } else if source_mapping.starts_with(MAPPING_START) {
            return SourceCode {
                code,
                source_mapping: Some(source_mapping.to_string())
                    .and_then(|x| SourceMap::from_slice(x.as_bytes()).ok()),
            };
        }

        SourceCode {
            code: input.to_owned(),
            source_mapping: None,
        }
    }
}

fn try_decode_mapping(input: &str) -> Option<String> {
    let source_mapping = base64::decode(input);

    match source_mapping {
        Ok(decoded) => match std::str::from_utf8(&decoded) {
            Ok(decoded) => Some(decoded.to_string()),
            Err(_) => None,
        },
        Err(_) => None,
    }
}
