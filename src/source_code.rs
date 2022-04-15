#[derive(Debug)]
pub struct SourceCode {
    pub code: String,
    pub source_mapping: Option<String>,
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

        if source_mapping.starts_with(mapping_start) {
            if source_mapping.starts_with(&base64_mapping_start) {
                let base64_input = input.replace(&base64_mapping_start, "");
                println!("{}", base64_input);

                let source_mapping = match base64::decode_config(base64_input, base64::STANDARD) {
                    Ok(decoded) => match std::str::from_utf8(&decoded) {
                        Ok(decoded) => decoded.to_string(),
                        Err(_) => source_mapping.to_string(),
                    },
                    Err(e) => {
                        println!("kokot3 {:?}", e);
                        source_mapping.to_string()
                    }
                };
                return SourceCode {
                    code,
                    source_mapping: Some(source_mapping),
                };
            } else {
                return SourceCode {
                    code,
                    source_mapping: Some(source_mapping.to_string()),
                };
            }
        }

        SourceCode {
            code: input.to_owned(),
            source_mapping: None,
        }
    }
}
