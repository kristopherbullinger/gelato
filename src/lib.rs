use std::borrow::Cow;
use std::collections::LinkedList;
use std::str::FromStr;
use regex::Regex;

// language metadata and parsing patterns (compiled regexes)
pub struct LanguageConfig {
    language: Language,
    token_matchers: Vec<Matcher>,
}

// token type and regex pattern for identification
struct Matcher {
    token_type: TokenType,
    patterns: Vec<Regex>,
}

// language metadata and parsing patterns ('static strs)
struct RawLanguageConfig<'a> {
    language: Language,
    token_matchers: &'a [RawMatcher<'a>],
}

#[derive(Clone, Copy)]
struct RawMatcher<'a> {
    token_type: TokenType,
    patterns: &'a [&'a str],
}

impl<'a> From<RawMatcher<'a>> for Matcher {
    fn from(raw: RawMatcher) -> Self {
        let patterns = raw.patterns.iter()
            .copied()
            .map(|pattern| Regex::new(pattern).unwrap())
            .collect::<Vec<_>>();
        Self {
            token_type: raw.token_type,
            patterns,
        }
    }
}

impl<'a> From<RawLanguageConfig<'a>> for LanguageConfig {
    fn from(raw: RawLanguageConfig) -> Self {
        let token_matchers = raw.token_matchers.iter()
            .copied()
            .map(|matcher| Matcher::from(matcher))
            .collect::<Vec<_>>();
        Self {
            language: raw.language,
            token_matchers,
        }
    }
} 

#[derive(Clone, Copy, PartialEq, Eq)]
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
                    token_matchers: vec![],
                }
            }
            Unknown => {
                LanguageConfig {
                    language,
                    token_matchers: vec![],
                }
            }
        }
    }
}

pub enum Token<'a> {
    Text(Cow<'a, &'a str>),
    Token {
        token_type: TokenType,
        content: &'a str,
    }
}

#[derive(Copy, Clone)]
pub enum TokenType {
    Punctuation,
    Identifier,
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
    pub token_list: LinkedList<Token<'a>>,
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

    pub fn tokenize(&mut self) {
        todo!();

    }
}
