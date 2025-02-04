//! This module contains conversion of intermediate AST to provable AST and a non-provable component if necessary.
mod error;
mod where_expr_builder_tests;
pub use error::ConversionError;
pub(crate) use error::ConversionResult;

#[cfg(all(test, feature = "blitzar"))]
mod query_expr_tests;

mod query_expr;
pub use query_expr::QueryExpr;

mod result_expr_builder;
pub(crate) use result_expr_builder::ResultExprBuilder;

mod filter_expr_builder;
pub(crate) use filter_expr_builder::FilterExprBuilder;

pub(crate) mod query_context;
pub(crate) use query_context::QueryContext;

mod query_context_builder;
pub(crate) use query_context_builder::{type_check_binary_operation, QueryContextBuilder};

mod where_expr_builder;
pub(crate) use where_expr_builder::WhereExprBuilder;
