use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Sections {
    pub sections: Vec<Section>,
}

#[derive(Deserialize, Debug)]
pub struct Section {
    pub issues: Vec<Issue>,
}

#[derive(Deserialize, Debug)]
pub struct Issue {
    pub key: String,
    #[serde(rename = "summaryText")]
    pub summary_text: String,
}

#[cfg(test)]
mod test {
    use super::Sections;

    #[test]
    fn test() {
        let json = r#"
            {
                "sections": [
                    {
                        "label": "History Search",
                        "sub": "Showing 1 of 1 matching issues",
                        "id": "hs",
                        "issues": [
                            {
                                "id": 10000,
                                "key": "TEST-1",
                                "summary": "<b>Test</b>",
                                "summaryText": "Test"
                            }
                        ]
                    }
                ]
            }
        "#;

        let o = serde_json::from_str::<Sections>(json).unwrap();
        println!("{:#?}", o);
    }
}
