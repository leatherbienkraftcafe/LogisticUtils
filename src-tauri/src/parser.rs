use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct ParsedRow {
    pub key: String,
    pub value: String,
}

fn is_delimiter(c: char) -> bool {
    c == ':' || c.is_whitespace()
}

/// Amazon/Yahoo/Excel などからコピーした「キー 値」形式のテキストを行ごとに分解する。
/// 区切り文字は `:` / タブ / 連続する空白のいずれか。
pub fn parse_lines(payload: &str) -> Vec<ParsedRow> {
    payload
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let idx = line.find(is_delimiter)?;
            let key = line[..idx].trim();
            let value = line[idx..].trim_start_matches(is_delimiter).trim();
            if key.is_empty() || value.is_empty() {
                return None;
            }
            Some(ParsedRow {
                key: key.to_string(),
                value: value.to_string(),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_colon_separated_lines() {
        let input = "名前: 山田太郎\n住所: 東京都千代田区1-1";
        let rows = parse_lines(input);
        assert_eq!(
            rows,
            vec![
                ParsedRow { key: "名前".into(), value: "山田太郎".into() },
                ParsedRow { key: "住所".into(), value: "東京都千代田区1-1".into() },
            ]
        );
    }

    #[test]
    fn parses_whitespace_separated_lines() {
        let input = "郵便番号 100-0001\n電話番号\t03-1234-5678";
        let rows = parse_lines(input);
        assert_eq!(
            rows,
            vec![
                ParsedRow { key: "郵便番号".into(), value: "100-0001".into() },
                ParsedRow { key: "電話番号".into(), value: "03-1234-5678".into() },
            ]
        );
    }

    #[test]
    fn skips_blank_and_malformed_lines() {
        let input = "\n   \n単独キー\nキー: 値\n";
        let rows = parse_lines(input);
        assert_eq!(
            rows,
            vec![ParsedRow { key: "キー".into(), value: "値".into() }]
        );
    }

    #[test]
    fn trims_surrounding_whitespace() {
        let input = "  キー  :   値  ";
        let rows = parse_lines(input);
        assert_eq!(rows, vec![ParsedRow { key: "キー".into(), value: "値".into() }]);
    }
}
