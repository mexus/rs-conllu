//! A library for parsing the CoNNL-U format.
//!
//! ## Basic Usage
//!
//! Parse a sentence in CoNNL-U format and iterate over the
//! containing [`Token`] elements.
//! Example taken from [CoNLL-U format description](https://universaldependencies.org/format.html).
//!
//! ```
//! use rs_conllu::{parse_sentence, TokenID};
//!
//! let s = "# sent_id = 1
//! ## text = They buy and sell books.
//! 1	They	they	PRON	PRP	Case=Nom|Number=Plur	2	nsubj	2:nsubj|4:nsubj	_
//! 2	buy	buy	VERB	VBP	Number=Plur|Person=3|Tense=Pres	0	root	0:root	_
//! 3	and	and	CCONJ	CC	_	4	cc	4:cc	_
//! 4	sell	sell	VERB	VBP	Number=Plur|Person=4|Tense=Pres	2	conj	0:root|2:conj	_
//! 6	books	book	NOUN	NNS	Number=Plur	2	obj	2:obj|4:obj	SpaceAfter=No
//! 7	.	.	PUNCT	.	_	2	punct	2:punct	_
//! ";
//!
//! let sentence = parse_sentence(s).unwrap();
//! let mut token_iter = sentence.into_iter();
//!
//! assert_eq!(token_iter.next().unwrap().id, TokenID::Single(1));
//! assert_eq!(token_iter.next().unwrap().form, "buy".to_owned());
//!
//! ```

#![allow(clippy::tabs_in_doc_comments)]

use std::{collections::HashMap, error::Error, fmt, str::FromStr};

pub mod cli;
pub mod parsers;

pub use crate::parsers::{parse_file, parse_sentence, parse_token};

pub struct Feature<'a>(pub &'a str, pub &'a str);

#[derive(Debug, PartialEq, Eq)]
pub struct ParseUposError;

impl fmt::Display for ParseUposError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error while parsing UPOS.")
    }
}

impl Error for ParseUposError {}

/// The set of Universal POS tags according
/// to [UD version 2](https://universaldependencies.org/u/pos/index.html).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UPOS {
    ADJ,
    ADP,
    ADV,
    AUX,
    CCONJ,
    DET,
    INTJ,
    NOUN,
    NUM,
    PART,
    PRON,
    PROPN,
    PUNCT,
    SCONJ,
    SYM,
    VERB,
    X,
}

impl FromStr for UPOS {
    type Err = ParseUposError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        use UPOS::*;
        match value {
            "ADJ" => Ok(ADJ),
            "ADP" => Ok(ADP),
            "ADV" => Ok(ADV),
            "AUX" => Ok(AUX),
            "CCONJ" => Ok(CCONJ),
            "DET" => Ok(DET),
            "INTJ" => Ok(INTJ),
            "NOUN" => Ok(NOUN),
            "NUM" => Ok(NUM),
            "PART" => Ok(PART),
            "PRON" => Ok(PRON),
            "PROPN" => Ok(PROPN),
            "PUNCT" => Ok(PUNCT),
            "SCONJ" => Ok(SCONJ),
            "SYM" => Ok(SYM),
            "VERB" => Ok(VERB),
            "X" => Ok(X),
            _ => Err(ParseUposError),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenID {
    Single(usize),
    Range(usize, usize),
    Subordinate { major: usize, minor: usize },
}

type Features = HashMap<String, String>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub id: TokenID,
    pub form: String,
    pub lemma: Option<String>,
    pub upos: Option<UPOS>,
    pub xpos: Option<String>,
    pub features: Option<Features>,
    pub head: Option<TokenID>,
    pub deprel: Option<String>,
    pub dep: Option<Vec<Dep>>,
    pub misc: Option<String>,
}

impl Token {
    pub fn builder(id: TokenID, form: String) -> TokenBuilder {
        TokenBuilder::new(id, form)
    }
}

pub struct TokenBuilder {
    id: TokenID,
    form: String,
    lemma: Option<String>,
    upos: Option<UPOS>,
    xpos: Option<String>,
    features: Option<Features>,
    head: Option<TokenID>,
    deprel: Option<String>,
    dep: Option<Vec<Dep>>,
    misc: Option<String>,
}

impl TokenBuilder {
    pub fn new(id: TokenID, form: String) -> TokenBuilder {
        TokenBuilder {
            id,
            form,
            lemma: None,
            upos: None,
            xpos: None,
            features: None,
            head: None,
            deprel: None,
            dep: None,
            misc: None,
        }
    }

    pub fn lemma(mut self, lemma: String) -> TokenBuilder {
        self.lemma = Some(lemma);
        self
    }

    pub fn upos(mut self, upos: UPOS) -> TokenBuilder {
        self.upos = Some(upos);
        self
    }

    pub fn xpos(mut self, xpos: String) -> TokenBuilder {
        self.xpos = Some(xpos);
        self
    }

    pub fn features(mut self, features: Features) -> TokenBuilder {
        self.features = Some(features);
        self
    }

    pub fn head(mut self, head: TokenID) -> TokenBuilder {
        self.head = Some(head);
        self
    }

    pub fn deprel(mut self, deprel: String) -> TokenBuilder {
        self.deprel = Some(deprel);
        self
    }

    pub fn dep(mut self, dep: Vec<Dep>) -> TokenBuilder {
        self.dep = Some(dep);
        self
    }

    pub fn misc(mut self, misc: String) -> TokenBuilder {
        self.misc = Some(misc);
        self
    }

    pub fn build(self) -> Token {
        Token {
            id: self.id,
            form: self.form,
            lemma: self.lemma,
            upos: self.upos,
            xpos: self.xpos,
            features: self.features,
            head: self.head,
            deprel: self.deprel,
            dep: self.dep,
            misc: self.misc,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dep {
    pub head: TokenID,
    pub rel: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sentence {
    pub meta: Vec<String>,
    pub tokens: Vec<Token>,
}

impl IntoIterator for Sentence {
    type Item = Token;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.tokens.into_iter()
    }
}
