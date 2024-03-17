use wasm_bindgen::prelude::*;

mod katana;

fn unescape_html(html_string: &str) -> String {
    let replacements = [
        ("&nbsp;", " "),
        ("&amp;", "&"),
        ("&lt;", "<"),
        ("&gt;", ">"),
        ("&quot;", "\""),
        ("&#39;", "'"),
        ("&apos;", "'"),
        ("&cent;", "¢"),
        ("&pound;", "£"),
        ("&yen;", "¥"),
        ("&euro;", "€"),
        ("&copy;", "©"),
        ("&reg;", "®"),
        ("&sect;", "§"),
        ("&uml;", "¨"),
        ("&ordf;", "ª"),
        ("&laquo;", "«"),
        ("&not;", "¬"),
        ("&shy;", "­"),
        ("&macr;", "¯"),
        ("&deg;", "°"),
        ("&plusmn;", "±"),
        ("&sup2;", "²"),
        ("&sup3;", "³"),
        ("&acute;", "´"),
        ("&micro;", "µ"),
        ("&para;", "¶"),
        ("&middot;", "·"),
        ("&cedil;", "¸"),
        ("&sup1;", "¹"),
        ("&ordm;", "º"),
        ("&raquo;", "»"),
        ("&frac14;", "¼"),
        ("&frac12;", "½"),
        ("&frac34;", "¾"),
        ("&iquest;", "¿"),
        ("&Agrave;", "À"),
        ("&Aacute;", "Á"),
        ("&Acirc;", "Â"),
        ("&Atilde;", "Ã"),
        ("&Auml;", "Ä"),
        ("&Aring;", "Å"),
        ("&AElig;", "Æ"),
        ("&Ccedil;", "Ç"),
        ("&Egrave;", "È"),
        ("&Eacute;", "É"),
        ("&Ecirc;", "Ê"),
        ("&Euml;", "Ë"),
        ("&Igrave;", "Ì"),
        ("&Iacute;", "Í"),
        ("&Icirc;", "Î"),
        ("&Iuml;", "Ï"),
        ("&ETH;", "Ð"),
        ("&Ntilde;", "Ñ"),
        ("&Ograve;", "Ò"),
        ("&Oacute;", "Ó"),
        ("&Ocirc;", "Ô"),
        ("&Otilde;", "Õ"),
        ("&Ouml;", "Ö"),
        ("&times;", "×"),
        ("&Oslash;", "Ø"),
        ("&Ugrave;", "Ù"),
        ("&Uacute;", "Ú"),
        ("&Ucirc;", "Û"),
        ("&Uuml;", "Ü"),
        ("&Yacute;", "Ý"),
        ("&THORN;", "Þ"),
        ("&szlig;", "ß"),
        ("&agrave;", "à"),
        ("&aacute;", "á"),
        ("&acirc;", "â"),
        ("&atilde;", "ã"),
        ("&auml;", "ä"),
        ("&aring;", "å"),
        ("&aelig;", "æ"),
        ("&ccedil;", "ç"),
        ("&egrave;", "è"),
        ("&eacute;", "é"),
        ("&ecirc;", "ê"),
        ("&euml;", "ë"),
        ("&igrave;", "ì"),
        ("&iacute;", "í"),
        ("&icirc;", "î"),
        ("&iuml;", "ï"),
        ("&eth;", "ð"),
        ("&ntilde;", "ñ"),
        ("&ograve;", "ò"),
        ("&oacute;", "ó"),
        ("&ocirc;", "ô"),
        ("&otilde;", "õ"),
        ("&ouml;", "ö"),
        ("&divide;", "÷"),
        ("&oslash;", "ø"),
        ("&ugrave;", "ù"),
        ("&uacute;", "ú"),
        ("&ucirc;", "û"),
        ("&uuml;", "ü"),
        ("&yacute;", "ý"),
        ("&thorn;", "þ"),
        ("&yuml;", "ÿ"),
    ];
    replacements
        .iter()
        .fold(html_string.to_string(), |acc, &(entity, char)| {
            acc.replace(entity, char)
        })
}

fn replace_abbreviations(text: &str) -> String {
    let abbreviations = [
        ("i.e.", "ie"),
        ("e.g.", "eg"),
        ("etc.", "etc"),
        ("mr.", "mr"),
        ("mrs.", "mrs"),
        ("vs.", "vs"),
        ("dr.", "dr"),
        ("prof.", "prof"),
        ("sr.", "sr"),
        ("jr.", "jr"),
        ("st.", "st"),
        ("jan.", "jan"),
        ("feb.", "feb"),
        ("mar.", "mar"),
        ("apr.", "apr"),
        ("jun.", "jun"),
        ("jul.", "jul"),
        ("aug.", "aug"),
        ("sept.", "sept"),
        ("oct.", "oct"),
        ("nov.", "nov"),
        ("dec.", "dec"),
        ("a.m.", "am"),
        ("p.m.", "pm"),
        ("u.s.", "us"),
        ("u.k.", "uk"),
    ];
    let regex_set = regex::RegexSet::new(abbreviations.iter().map(|&(abbr, _)| abbr)).unwrap();
    regex_set
        .matches(text)
        .iter()
        .fold(text.to_string(), |acc, m| {
            let (from, to) = abbreviations[m];
            acc.replace(from, to)
        })
}

fn remove_html_tags(html_string: &str) -> String {
    let text = regex::Regex::new(r"(?s)<!--(.*?)-->")
        .unwrap()
        .replace_all(html_string, "")
        .into_owned();

    let text = regex::Regex::new(r"(?s)<h[1-6]>(.*?)</h[1-6]>")
        .unwrap()
        .replace_all(&text, "$1\n\n")
        .into_owned();

    let text = unescape_html(&text);
    let text = regex::Regex::new(r"<(.*?)>")
        .unwrap()
        .replace_all(&text, " ")
        .into_owned();
    let text = regex::Regex::new(r"  ")
        .unwrap()
        .replace_all(&text, " ")
        .into_owned();
    let text = replace_abbreviations(&text);
    let text = regex::Regex::new(r"\n\s*?\n")
        .unwrap()
        .replace_all(&text, "\n\n")
        .into_owned();
    let text = regex::Regex::new(r"\s?\[[0-9]+\]\s?")
        .unwrap()
        .replace_all(&text, "")
        .into_owned();
    let text = text
        .split("\n")
        .map(|line| line.trim())
        .filter(|line| !line.starts_with("^  "))
        .collect::<Vec<&str>>()
        .join("\n");
    // remove all sequences of 3 or more newlines with two newlines
    let text = regex::Regex::new(r"\n{3,}")
        .unwrap()
        .replace_all(&text, "\n\n")
        .into_owned();
    text
}

#[wasm_bindgen]
pub fn prepare_text(text: &str) -> String {
    let text = text
        .split("\n")
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>()
        .join(" ");

    let text = html2md::parse_html(&text);

    let text = remove_html_tags(&text);

    let paragraphs = katana::cut(&text);

    paragraphs
        .iter()
        .map(|p| p.as_slice().join(" "))
        .collect::<Vec<String>>()
        .join("\n\n")
}
