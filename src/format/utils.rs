use percent_encoding::{percent_decode, utf8_percent_encode, NON_ALPHANUMERIC};
use regex::Regex;
use zalgo::{Generator, GeneratorArgs, ZalgoSize};

// Telegram allows only alphanumeric characters and underscores in bot commands,
// but we often have spaces, apostrophes and so on.
// Here we are utilizing url encode to replace such symbols with %\d\d (e.g., %20),
// but since % is itself forbidden character we are wrapping the resulting digit in underscores.
pub fn tg_encode(msg: &str) -> String {
	lazy_static! {
		static ref CHAR_REGEX: Regex = Regex::new(r"%([\dA-F]{2})").unwrap();
	}
	let msg = utf8_percent_encode(msg, NON_ALPHANUMERIC).to_string();
	CHAR_REGEX.replace_all(&msg, "_${1}_").into()
}

pub fn tg_decode(msg: &str) -> String {
	lazy_static! {
		static ref CHAR_REGEX: Regex = Regex::new(r"_([\dA-F]{2})_").unwrap();
	}
	let url_encoded = CHAR_REGEX.replace_all(msg, "%${1}");
	percent_decode(url_encoded.as_bytes())
		.decode_utf8_lossy()
		.to_string()
}

pub fn zalgofy(text: &mut String) {
	lazy_static! {
		static ref OPTIONS: GeneratorArgs = GeneratorArgs::new(true, true, true, ZalgoSize::None);
	}
	let mut zalgo = Generator::new();
	let mut buf = String::new();
	zalgo.gen(text.clone(), &mut buf, &OPTIONS);
	*text = buf;
}
