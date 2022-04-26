use sqlparser::ast::{ColumnOption, DataType, Statement};

use crate::error::{Result, SQLRiteError};

#[derive(PartialEq, Debug)]
pub struct ParsedColumn {
    pub name: String,
    pub datatype: String,
    pub is_pk: bool,
    pub is_nullable: bool,
}

#[derive(Debug)]
pub struct CreateQuery {
    pub table_name: String,
    pub columns: Vec<ParsedColumn>,
}

impl CreateQuery {
    pub fn new(statement: &Statement) -> Result<Self> {
        match statement {
            Statement::CreateTable {
                name,
                columns,
                constraints,
                ..
            } => {
                let table_name = name;
                let mut parsed_columns: Vec<ParsedColumn> = vec![];
                for col in columns {
                    let name = col.name.to_string();
                    let datatype = match &col.data_type {
                        DataType::SmallInt(_val) => "int",
                        DataType::Int(_val) => "int",
                        DataType::BigInt(_val) => "int",
                        DataType::Boolean => "bool",
                        DataType::Text => "string",
                        DataType::Varchar(_bytes) => "string",
                        DataType::Float(_precision) => "float",
                        DataType::Double => "float",
                        DataType::Decimal(_precision1, _precision2) => "float",
                        _ => {
                            println!("not matched on custom type");
                            "invalid"
                        }
                    };

                    let mut is_pk: bool = false;
                    for column_option in &col.options {
                        is_pk = match column_option.option {
                            ColumnOption::Unique { is_primary } => is_primary,
                            _ => false,
                        };
                    }
                    parsed_columns.push(ParsedColumn {
                        name,
                        datatype: datatype.to_string(),
                        is_pk,
                        is_nullable: false,
                    });
                }

                // TODO: Handle constraints
                // Unique, Primary key, Nullable, Default value and others.
                for constraint in constraints {
                    println!("{:?}", constraint);
                }
                return Ok(CreateQuery {
                    table_name: table_name.to_string(),
                    columns: parsed_columns,
                });
            }
            _ => return Err(SQLRiteError::Internal("Error parsing query".to_string())),
        }
    }
}
