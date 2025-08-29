#![feature(associated_type_defaults, generic_associated_types)]
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait ProcessTrait {
    type Input<'input>;
    type Output<'output>;
    type Error: std::error::Error;

    async fn process<'input, 'output>(
        input: Self::Input<'input>,
    ) -> Result<Self::Output<'output>, Self::Error>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frontmatter {
    pub title: Option<String>,
    pub authors: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub aliases: Option<Vec<String>>,
    pub date: Option<String>,
}

#[allow(clippy::type_complexity)]
struct Processor<Input, Output, Error> {
    last_transformation: Option<(
        std::marker::PhantomData<Input>,
        std::marker::PhantomData<Output>,
        std::marker::PhantomData<Error>,
    )>,
    transformations: Vec<Box<,
}

impl Processor<(), (), ()> {
    pub fn new() -> Self {
        Self {
            transformations: Vec::new(),
        }
    }

    pub fn apply<F, Input, Output, Error>(&mut self, f: F) -> Processor<Input, Output, Error>
    where
        F: for<'input, 'output> ProcessTrait<
            Input<'input> = Input,
            Output<'output> = Output,
            Error = Error,
        >,
    {
        Processor {
            transformations: self.transformations,
        }
    }
}
