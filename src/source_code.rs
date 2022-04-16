use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub struct SourceCode {
    pub code: String,
    pub source_mapping: Option<SourceMapping>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SourceMapping {
    pub file: String,
    pub mappings: String,
    pub names: Vec<String>,
    pub sources: Vec<String>,
    pub sources_content: Vec<String>,
    pub version: i32,
}

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

        let mapping_start = "//# sourceMappingURL=";
        let base64_coding = "data:application/json;charset=utf-8;base64";
        let base64_mapping_start = format!("{}{},", mapping_start, base64_coding);

        if source_mapping.starts_with(&base64_mapping_start) {
            let base64_input = source_mapping.replace(&base64_mapping_start, "");
            let maybe_decoded_mapping = try_decode_mapping(&base64_input)
                .and_then(|x| serde_json::from_str::<SourceMapping>(&x).ok());

            return SourceCode {
                code,
                source_mapping: maybe_decoded_mapping,
            };
        } else if source_mapping.starts_with(mapping_start) {
            return SourceCode {
                code,
                source_mapping: Some(source_mapping.to_string())
                    .and_then(|x| serde_json::from_str::<SourceMapping>(&x).ok()),
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
