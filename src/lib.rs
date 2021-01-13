use std::borrow::Cow;
use std::collections::LinkedList;
use std::str::FromStr;
use regex::Regex;

// language metadata and parsing patterns
pub struct LanguageConfig {
    language: Language,
    matchers: Vec<Matcher>,
}

// token type and regex pattern for identification
struct Matcher {
    token_type: TokenType,
    patterns: Vec<Regex>,
}

pub enum Language {
    Rust,
    Unknown,
}
use Language::*;

impl FromStr for Language {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        Ok(match s.as_str() {
            "rs" | "rust" => Rust,
            _ => Unknown,
        })
    }
}

impl From<Language> for LanguageConfig {
    fn from(language: Language) -> Self {
        match language {
            Rust => {
                LanguageConfig {
                    language,
                    matchers: vec![],
                }
            }
            Unknown => {
                LanguageConfig {
                    language,
                    matchers: vec![],
                }
            }
        }
    }
}

pub enum Token<'a> {
    Text(Cow<'a, &'a str>),
    Token {
        r#type: TokenType,
        content: &'a str,
    }
}

pub enum TokenType {
    Punctuation,
    Keyword,
    Number,
    String,
    Comment,
    Class,
    Other(&'static str), // custom type name
}

pub struct Tokenizer<'a> {
    source: &'a str,
    language_config: LanguageConfig,
    token_list: LinkedList<Token<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(language: Language, source: &'a str) -> Self {
        let language_config: LanguageConfig = language.into();
        Self {
            source,
            language_config,
            token_list: LinkedList::new(),
        }
    }

    pub fn into_token_vec(self) -> Vec<Token<'a>> {
        self.token_list.into_iter().collect::<Vec<_>>()
    }

    pub fn token_list(&self) -> &LinkedList<Token<'a>> {
        &self.token_list
    }

    pub fn tokenize(&mut self) {
        todo!();

    }
}
