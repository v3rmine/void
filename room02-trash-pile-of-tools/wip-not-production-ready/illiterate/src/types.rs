use std::collections::HashMap;

use crate::parse::{
    IlliterateParserBlock, IlliterateParserCodeRef,
    IlliterateParserSourceFile,
};

#[derive(Debug, Clone)]
pub struct IlliterateSourceFile {
    pub file: String,
    pub code_blocks: Vec<IlliterateBlock>,
}

#[derive(Debug, Clone)]
pub enum IlliterateBlock {
    Named {
        lang: String,
        name: String,
        code_content: String,
        refs_in_code: Vec<IlliterateRef>,
        params: HashMap<String, String>,
    },
    File {
        lang: String,
        path: String,
        code_content: String,
        refs_in_code: Vec<IlliterateRef>,
        params: HashMap<String, String>,
    },
    Plain {
        lang: String,
        code_content: String,
        refs_in_code: Vec<IlliterateRef>,
        params: HashMap<String, String>,
    },
}

#[derive(Debug, Clone)]
pub struct IlliterateRef {
    pub base_indent_match: String,
    pub is_inline: bool,
    pub name: String,
    pub ref_text: String,
}

#[derive(Debug, Clone)]
pub struct IlliterateCodeWithRefs {
    pub code_content: String,
    pub refs_in_code: Vec<IlliterateRef>,
}

#[derive(Debug, Clone)]
pub struct IlliterateResolvedResult {
    pub resolved: HashMap<String, String>,
    pub cyclic: Vec<String>,
    pub missing: Vec<(String, String)>,
}

impl From<IlliterateParserSourceFile>
    for IlliterateSourceFile
{
    fn from(value: IlliterateParserSourceFile) -> Self {
        let mut source_file = IlliterateSourceFile {
            file: value.borrow_file().clone(),
            code_blocks: Vec::new(),
        };

        value.with_code_blocks(|code_blocks| {
            source_file.code_blocks = code_blocks
                .iter()
                .map(IlliterateBlock::from)
                .collect::<Vec<_>>();
        });

        source_file
    }
}

impl<'a> From<&IlliterateParserBlock<'a>>
    for IlliterateBlock
{
    fn from(value: &IlliterateParserBlock<'a>) -> Self {
        match value {
            IlliterateParserBlock::File {
                lang,
                path,
                code_content,
                refs_in_code,
                params,
                ..
            } => IlliterateBlock::File {
                lang: lang.content.to_string(),
                path: path.content.to_string(),
                code_content: code_content
                    .content
                    .to_string(),
                params: params
                    .iter()
                    .map(|(key, value)| {
                        (
                            key.to_string(),
                            value.content.to_string(),
                        )
                    })
                    .collect::<HashMap<_, _>>(),
                refs_in_code: refs_in_code
                    .iter()
                    .map(IlliterateRef::from)
                    .collect::<Vec<_>>(),
            },
            IlliterateParserBlock::Named {
                lang,
                name,
                code_content,
                refs_in_code,
                params,
                ..
            } => IlliterateBlock::Named {
                lang: lang.content.to_string(),
                name: name.content.to_string(),
                code_content: code_content
                    .content
                    .to_string(),
                params: params
                    .iter()
                    .map(|(key, value)| {
                        (
                            key.to_string(),
                            value.content.to_string(),
                        )
                    })
                    .collect::<HashMap<_, _>>(),
                refs_in_code: refs_in_code
                    .iter()
                    .map(IlliterateRef::from)
                    .collect::<Vec<_>>(),
            },
            IlliterateParserBlock::Plain {
                lang,
                refs_in_code,
                params,
                code_content,
                ..
            } => IlliterateBlock::Plain {
                lang: lang.content.to_string(),
                code_content: code_content
                    .content
                    .to_string(),
                params: params
                    .iter()
                    .map(|(key, value)| {
                        (
                            key.to_string(),
                            value.content.to_string(),
                        )
                    })
                    .collect::<HashMap<_, _>>(),
                refs_in_code: refs_in_code
                    .iter()
                    .map(IlliterateRef::from)
                    .collect::<Vec<_>>(),
            },
        }
    }
}

impl<'a> From<&IlliterateParserCodeRef<'a>>
    for IlliterateRef
{
    fn from(value: &IlliterateParserCodeRef<'a>) -> Self {
        Self {
            base_indent_match: value
                .base_indent_match
                .to_string(),
            is_inline: value.is_inline,
            name: value.regex_match.content.to_string(),
            ref_text: value
                .full_ref_match
                .content
                .to_string(),
        }
    }
}
