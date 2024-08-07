use std::collections::HashMap;

use crate::UPOS;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenID {
    /// The standard, single index.
    Single(usize),
    /// A range of tokens that form an ID. Denoted by a hyphen
    /// in CoNLL-U format (e.g. 1-3).
    Range(usize, usize),
    /// To represent ellipses, ConLL-U allows to create sub-indices of the preceding
    /// regular node (or 0 if it is a the beginning of a sentence). They are separated
    /// by a decimal point and represent an "empty" node.
    Empty(usize, usize),
}

type Features = HashMap<String, String>;

/// A `Token` is the basic unit of what is defined on a (non-comment) line in CoNLL-U format.
/// The ConLL-U specification uses the terms "word", "node" and "multi-word token" while this crate
/// decided to use the general notion of "Token" to subsume all of the above.
///
/// The fields of a `Token` are the ten fields that are defined in the CoNLL-U specification.
/// The only mandatory fields are [Token::id] and [Token::form]. The remaining ones are optional (absence denoted
/// by an underscore in the text format) and represented as [Option] types.
///
/// A [TokenBuilder] type is available for more convenient creation of [Token] structs.
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
    /// Return a new [TokenBuilder]
    pub fn builder(id: TokenID, form: String) -> TokenBuilder {
        TokenBuilder::new(id, form)
    }
}

/// A builder for Tokens to allow for more convenient manual creation if necessary.
///
/// ```rust
/// use rs_conllu::{Token, TokenID};
///
/// // Get a new builder from Token
/// let token = Token::builder(TokenID::Single(1), "Hello".to_string())
///     .lemma("Hello".to_string())
///     .build();
///
/// ```
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

    /// Set the lemma of the token.
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
