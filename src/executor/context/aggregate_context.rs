use std::prelude::v1::*;

use {
    super::BlendContext,
    crate::{ast::Aggregate, data::Value},
    im_rc::HashMap,
    std::{fmt::Debug, rc::Rc},
};

#[derive(Debug)]
pub struct AggregateContext<'a> {
    pub aggregated: Option<HashMap<&'a Aggregate, Value>>,
    pub next: Rc<BlendContext<'a>>,
}
