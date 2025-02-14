#![allow(dead_code)]
pub(crate) fn is_valid_hash(content: &str) -> bool {
    matches!(content.len(), 32 | 40) && content.chars().all(|c| c.is_digit(16))
}
pub(crate) fn is_valid_sha1_hex(content: &str) -> bool {
    content.len() == 40 && content.chars().all(|c| c.is_digit(16))
}
pub(crate) fn is_valid_hex(content: &str) -> bool {
    content.chars().all(|c| c.is_digit(16))
}

pub(crate) fn is_valid_sha1_line(content: &str) -> bool {
    if content.starts_with("115://") && (content.matches('|').count() > 3) {
        let res: Vec<&str> = content.split('|').collect();
        return res[1].chars().all(|x| x.is_digit(10))
            && is_valid_sha1_hex(res[2])
            && is_valid_sha1_hex(res[3]);
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn valid_hash_test() {
        //all lower cap
        assert!(is_valid_hash("3e63c6d6e7a1015bfddd23768e1af38fae3bc203"));
        //all upper cap
        assert!(is_valid_hash("3E63C6D6E7A1015BFDDD23768E1AF38FAE3BC203"));
        //mix cap
        assert!(is_valid_hash("3e63C6d6e7a1015bfdDd23768e1af38fae3bC203"));

        // 32 len
        assert!(is_valid_hash("3e63C6d6e7a1015bfdDd23768e1af312"));

        // magnet:?xt=urn:btih:3e63c6d6e7a1015bfddd23768e1af38fae3bc203&dn=%E3%81%AA%E3%81%BE%E3%81%84%E3%81%8D%E3%81%96%E3%81%8B%E3%82%8A%E3%80%82%20%E7%AC%AC01-22%E5%B7%BB%20%5B
    }

    #[test]
    fn invalid_hash_test() {
        assert!(!is_valid_hash(""));
        assert!(!is_valid_hash("chan"));
        assert!(!is_valid_hash("123456"));

        // include g
        assert!(!is_valid_hash("3g63C6d6e7a1015bfdDd23768e1af38fae3bC203"),);
        //31
        assert!(!is_valid_hash("1234567890123456789012345678901"));
        //33
        assert!(!is_valid_hash("123456789012345678901234567890123"));

        //39
        assert!(!is_valid_hash("123456789012345678901234567890123456789"));
        //41
        assert!(!is_valid_hash("12345678901234567890123456789012345678901"));

        assert!(!is_valid_hash(
            "this is more than 40fjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjj"
        ));
    }

    #[test]
    fn sha1_link_without_protocol_test() {
        let line_without_protocol = "[座头鲸 Humpback Whales 2015][3D+2D][无中字][18.52GB].iso|19880869888|702C4E22BE8F3D856C496178C488E86B606D9912|13F48115A678499823003C8331E9C0AD0243F089";
        assert!(!is_valid_sha1_line(line_without_protocol))
    }
}
