use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub colname: String,
    pub dtype: String,
    pub default: Option<String>,
    pub options: Vec<String>,
}
#[derive(Debug, Clone)]
pub enum Constraint {
    PrimaryKey(PrimaryKey),
    ForeignKey(ForeignKey),
    Unique(Unique),
}
#[derive(Debug, Clone)]
pub struct Unique {
    pub columns: Vec<String>,
}
#[derive(Debug, Clone)]
pub struct PrimaryKey {
    pub columns: Vec<String>,
}
#[derive(Debug, Clone)]
pub struct ForeignKey {
    pub source_columns: Vec<String>,
    pub target_table: String,
    pub target_columns: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
    pub constraints: Vec<Constraint>,
}

#[derive(Debug, Clone)]
pub struct SelectColumn {
    pub name: String,
    pub alias: Option<String>,
    pub table_name: Option<String>,
}
#[derive(Debug, Clone)]
pub struct JoinKey {
    pub table_name: String,
    pub column_name: String,
}

#[derive(Debug)]
pub struct JoinSegment {
    pub left_key: JoinKey,
    pub right_key: JoinKey,
}

#[derive(Clone, Debug)]
pub struct Join {
    pub identifier: AliasedTableIdentifier,
    pub join_segments: Arc<Vec<JoinSegment>>,
}
#[derive(Clone, Debug)]
pub struct TableIdentifier {
    pub name: String,
    pub schema_name: Option<String>,
}

#[derive(Clone, Debug)]
pub struct AliasedTableIdentifier {
    pub alias: Option<String>,
    pub identifier: TableIdentifier,
}

#[derive(Debug)]
pub struct ColumnIdentifier {
    pub table_name: Option<String>,
    pub name: String,
}

#[derive(Debug)]
pub struct Select {
    pub table_identifier: AliasedTableIdentifier,
    pub select_columns: Vec<SelectColumn>,
    pub joins: Vec<Join>,
    pub statement: String,
}

#[derive(Debug)]
pub struct View {
    pub select: Select,
    pub name: String
}
impl Table {
    pub fn primary_keys(&self) -> Vec<Column> {
        let primary_keys: Vec<String> = self
            .constraints
            .iter()
            .filter_map(|item| {
                if let Constraint::PrimaryKey(a) = item {
                    Some(a)
                } else {
                    None
                }
            })
            .flat_map(|x| x.columns.clone())
            .collect();

        self.columns
            .clone()
            .into_iter()
            .filter(|col| primary_keys.contains(&col.name))
            .collect::<Vec<Column>>()
    }

    pub fn from_view(view: &View, table_hierarchy: HashMap<String, Table> ) -> Self {
        return Self::from_select(&view.select, view.name.clone(), table_hierarchy)
    }
    pub fn from_select(
        select: &Select,
        name: String,
        table_hierarchy: HashMap<String, Table>,
    ) -> Self {
        let columns_and_constraints = select
            .select_columns
            .iter()
            .map(|x| {
                let table_name = x.table_name.clone().unwrap_or_else(|| {
                    panic!(
                        "Expected a table name for the select column {} but found none",
                        x.name
                    )
                });
                let source_table = select
                    .joins
                    .iter()
                    .find(move |z| {
                        let source_alias = z.identifier.alias.clone().unwrap_or_else(|| {
                            panic!(
                                "Expected a table name for the join colum {:?} but found none",
                                z
                            )
                        });
                        source_alias == table_name
                    });

                let tname = x.table_name.clone().unwrap_or_else(|| {
                    panic!(
                        "Expected a table name for the select column {} but found none",
                        x.name
                    )
                });

                let table_identifier: String = match source_table {
                   Some(t) =>  t.identifier.identifier.name.clone(),
                    None => {
                         match select.table_identifier.alias.clone().unwrap() == tname{
                            true => select.table_identifier.identifier.name.clone(),
                            false => panic!("Did not find a corresponding table for the column {} ", x.name)
                            
                        }
                    }
            };
                let table = table_hierarchy.get(&table_identifier).expect(
                    "Did not find a matching table for the select statment in the table hierarchy.",
                );
                let column = table
                    .columns
                    .iter()
                    .find(move |j| j.name == x.name)
                    .unwrap_or_else(|| {
                        panic!(
                            "Did not find a column matching column {} in the select statement",
                            x.name
                        )
                    });

                let colname = match x.alias.clone() {
                    Some(a) => a,
                    None => column.name.clone()
                };
                let return_column = Column{
                    name: colname,
                    colname: column.colname.clone(),
                    dtype: column.dtype.clone(),
                    default: column.default.clone(),
                    options: column.options.clone() 
                };

                return_column
            })
            .collect();
        let constraints = Vec::new();

        Table {
            name,
            constraints,
            columns: columns_and_constraints,
        }
    }
}
#[derive(Debug, Clone)]
pub enum ConstraintOrColumn {
    Constraint(Constraint),
    Column(Column),
}

#[derive(Debug, Clone)]
pub struct ParseError {
    message: String,
    position: usize,
}

impl ParseError {
    fn new(message: String, position: usize) -> Self {
        ParseError { message, position }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "parsing failed at position {}: {}",
            self.position, self.message
        )
    }
}

pub type Parse<'a, Output> =
    Arc<dyn Fn(&'a str, usize) -> Result<(Output, &'a str, usize), ParseError> + 'a + Send + Sync>;
#[derive(Clone)]
pub struct Parser<'a, Output> {
    parser: Arc<
        dyn Fn(&'a str, usize) -> Result<(Output, &'a str, usize), ParseError> + 'a + Send + Sync,
    >,
}

impl<'a, Output: 'a> Parser<'a, Output> {
    // ... existing methods

    fn one_or_more(self) -> Parser<'a, Vec<Output>>
    where
        Output: Clone + 'a,
    {
        Parser::new(move |mut input: &'a str, mut position: usize| {
            let mut results = Vec::new();

            // Parse the first occurrence to ensure at least one match
            match self.parse_with_position(input, position) {
                Ok((first_result, remaining_input, new_position)) => {
                    results.push(first_result);
                    input = remaining_input;
                    position = new_position;
                }
                Err(err) => return Err(err),
            }

            // Continue parsing while there are more matches
            while let Ok((result, remaining_input, new_position)) =
                self.parse_with_position(input, position)
            {
                results.push(result);
                input = remaining_input;
                position = new_position;
            }

            Ok((results, input, position))
        })
    }

    pub fn zero_or_more(self) -> Parser<'a, Vec<Output>>
    where
        Output: Clone + 'a,
    {
        Parser::new(move |mut input: &'a str, mut position: usize| {
            let mut results = Vec::new();

            while let Ok((result, remaining_input, new_position)) =
                self.parse_with_position(input, position)
            {
                results.push(result);
                input = remaining_input;
                position = new_position;
            }

            Ok((results, input, position))
        })
    }
}

impl<'a, Output: 'a> Parser<'a, Output> {
    pub fn new<F>(parser: F) -> Self
    where
        F: 'a
            + Fn(&'a str, usize) -> Result<(Output, &'a str, usize), ParseError>
            + 'a
            + Send
            + Sync,
    {
        Self {
            parser: Arc::new(parser),
        }
    }

    pub fn parse(&self, input: &'a str) -> Result<(Output, &'a str, usize), ParseError> {
        self.parse_with_position(input, 0)
    }
    fn parse_with_position(
        &self,
        input: &'a str,
        position: usize,
    ) -> Result<(Output, &'a str, usize), ParseError> {
        (self.parser)(input, position).map_err(|e| ParseError::new(e.message, position))
    }

    pub fn map<B: 'a, F>(self, f: F) -> Parser<'a, B>
    where
        F: 'a + Send + Sync + Fn(Output) -> B,
    {
        Parser::new(move |input, position| {
            self.parse_with_position(input, position).map(
                |(output, remaining_input, new_position)| {
                    (f(output), remaining_input, new_position)
                },
            )
        })
    }

    pub fn and_then<B: 'a, F>(self, f: F) -> Parser<'a, B>
    where
        F: 'a + Send + Sync + Fn(Output) -> Parser<'a, B>,
    {
        Parser::new(
            move |input, position| match self.parse_with_position(input, position) {
                Ok((output1, remaining_input, new_position)) => {
                    f(output1).parse_with_position(remaining_input, new_position)
                }
                Err(e) => Err(ParseError::new(e.message, position)),
            },
        )
    }

    pub fn or(self, other: Parser<'a, Output>) -> Parser<'a, Output> {
        Parser::new(move |input, position| {
            self.parse_with_position(input, position)
                .or_else(|_| other.parse_with_position(input, position))
        })
    }
}

// Additional helper functions

pub fn whitespace<'a>() -> Parser<'a, ()> {
    Parser::new(|input: &'a str, position: usize| {
        let trimmed = input.trim_start();
        let len = input.len() - trimmed.len();
        if len > 0 {
            Ok(((), &input[len..], position + len))
        } else {
            Ok(((), input, position))
        }
    })
}

pub fn with_whitespace<'a, Output: 'a + Sync + Send>(
    parser: Parser<'a, Output>,
) -> Parser<'a, Output> {
    let parser: Parse<Output> = Arc::clone(&parser.parser);
    whitespace().and_then(move |_| {
        let parser = Arc::clone(&parser);
        Parser::new(move |input, position| {
            parser(input, position).and_then(move |(result, remaining_input, new_position)| {
                whitespace()
                    .parse_with_position(remaining_input, new_position)
                    .map(|(_, remaining_input, final_position)| {
                        (result, remaining_input, final_position)
                    })
            })
        })
    })
}

pub fn match_char<'a>(expected: char) -> Parser<'a, char> {
    Parser::new(move |input: &'a str, position: usize| {
        let mut chars = input.chars();
        if let Some(first_char) = chars.next() {
            if first_char == expected {
                return Ok((first_char, chars.as_str(), position + first_char.len_utf8()));
            }
        }
        Err(ParseError::new(
            format!(
                "Found invalid character sequence while examining input: '{}', expected {}",
                input, expected
            ),
            position,
        ))
    })
}

pub fn match_string<'a>(expected: &'a str) -> Parser<'a, &'a str> {
    Parser::new(move |input: &'a str, position: usize| {
        if input.to_lowercase().starts_with(&expected.to_lowercase()) {
            return Ok((
                expected,
                &input[expected.len()..],
                position + expected.len(),
            ));
        }
        Err(ParseError::new(
            format!(
                "Found invalid character sequence while examining input '{}', expected {} ",
                input, expected
            ),
            position,
        ))
    })
}

pub fn number<'a>() -> Parser<'a, &'a str> {
    Parser::new(|input: &'a str, position: usize| {
        let chars = input.chars();
        let mut end = 0;
        for c in chars {
            if c.is_numeric() || c == '.' {
                end += c.len_utf8();
            } else {
                break;
            }
        }
        if end > 0 {
            Ok((&input[..end], &input[end..], position + end))
        } else {
            Err(ParseError::new(
                format!("Found invalid input while looking for number: {}", input),
                position,
            ))
        }
    })
}

pub fn name<'a>() -> Parser<'a, &'a str> {
    Parser::new(|input: &'a str, position: usize| {
        let chars = input.chars();
        let mut end = 0;
        for c in chars {
            if c.is_alphanumeric() || c == '_' || c == '"' {
                end += c.len_utf8();
            } else {
                break;
            }
        }
        if end > 0 {
            Ok((&input[..end], &input[end..], position + end))
        } else {
            Err(ParseError::new(
                format!("Found invalid input while looking for name: {}", input),
                position,
            ))
        }
    })
}
pub fn function<'a>() -> Parser<'a, &'a str> {
    Parser::new(|input: &'a str, position: usize| {
        let chars = input.chars();
        let mut end = 0;
        for c in chars {
            if c.is_alphanumeric() || c == '_' || c == '(' || c == ')' || c == '"' {
                end += c.len_utf8();
            } else {
                break;
            }
        }
        if end > 0 {
            Ok((&input[..end], &input[end..], position + end))
        } else {
            Err(ParseError::new(
                format!("Found invalid input while looking for name: {}", input),
                position,
            ))
        }
    })
}

fn until<'a>() -> Parser<'a, &'a str> {
    Parser::new(|input: &'a str, position: usize| {
        let chars = input.chars();
        let mut end = 0;
        for c in chars {
            if c != ',' {
                end += c.len_utf8();
            } else {
                break;
            }
        }
        if end > 0 {
            Ok((&input[..end], &input[end..], position + end))
        } else {
            Err(ParseError::new(
                format!("Found invalid input: {}", input),
                position,
            ))
        }
    })
}
pub fn list<'a, Output: 'a>(
    parser: Parser<'a, Output>,
    string: &'a str,
) -> Parser<'a, Arc<Vec<Output>>> {
    Parser::new(move |input: &'a str, mut position: usize| {
        let mut result = Vec::new();
        let mut remaining_input = input;
        while let Ok((item, rest, new_position)) =
            parser.parse_with_position(remaining_input, position)
        {
            result.push(item);
            remaining_input = rest;
            position = new_position;
            if let Ok((_, rest, new_position)) =
                with_whitespace(match_string(string)).parse_with_position(remaining_input, position)
            {
                remaining_input = rest;
                position = new_position;
            } else {
                break;
            }
        }
        Ok((Arc::new(result), remaining_input, position))
    })
}

pub fn comma_sep<'a, Output: 'a>(parser: Parser<'a, Output>) -> Parser<'a, Arc<Vec<Output>>> {
    Parser::new(move |input: &'a str, mut position: usize| {
        let mut result = Vec::new();
        let mut remaining_input = input;
        while let Ok((item, rest, new_position)) =
            parser.parse_with_position(remaining_input, position)
        {
            result.push(item);
            remaining_input = rest;
            position = new_position;
            if let Ok((_, rest, new_position)) =
                with_whitespace(match_char(',')).parse_with_position(remaining_input, position)
            {
                remaining_input = rest;
                position = new_position;
            } else {
                break;
            }
        }
        Ok((Arc::new(result), remaining_input, position))
    })
}

pub fn primary_key<'a>() -> Parser<'a, ConstraintOrColumn> {
    with_whitespace(match_string("PRIMARY KEY")).and_then(move |_| {
        with_whitespace(match_char('('))
            .and_then(|_| comma_sep(with_whitespace(name())))
            .and_then({
                move |defs| {
                    match_char(')').map(move |_| {
                        ConstraintOrColumn::Constraint(Constraint::PrimaryKey(PrimaryKey {
                            columns: defs.to_vec().into_iter().map(|s| s.to_string()).collect(),
                        }))
                    })
                }
            })
    })
}
pub fn unique<'a>() -> Parser<'a, ConstraintOrColumn> {
    with_whitespace(match_string("UNIQUE")).and_then(move |_| {
        with_whitespace(match_char('('))
            .and_then(|_| comma_sep(with_whitespace(name())))
            .and_then({
                move |defs| {
                    match_char(')').map(move |_| {
                        ConstraintOrColumn::Constraint(Constraint::Unique(Unique {
                            columns: defs.to_vec().into_iter().map(|s| s.to_string()).collect(),
                        }))
                    })
                }
            })
    })
}

pub fn table_name_identifier<'a>() -> Parser<'a, TableIdentifier> {
    with_whitespace(name())
        .and_then(move |schema_name| {
            with_whitespace(match_char('.'))
                .and_then(|_| name())
                .map(|x| TableIdentifier {
                    name: x.to_string(),
                    schema_name: Some(schema_name.to_string()),
                })
        })
        .or(with_whitespace(name()).map(|x| TableIdentifier {
            name: x.to_string(),
            schema_name: None,
        }))
}

pub fn aliased_table_name_identifier<'a>() -> Parser<'a, AliasedTableIdentifier> {
    table_name_identifier().and_then({
        move |t| {
            with_whitespace(name())
                .or(Parser::new(|input, position| Ok(("", input, position))))
                .map(move |alias| AliasedTableIdentifier {
                    alias: Some(alias.to_string()),
                    identifier: t.clone(),
                })
        }
    })
}

pub fn column_name_table_name<'a>() -> Parser<'a, ColumnIdentifier> {
    with_whitespace(name())
        .and_then(move |table_name| {
            with_whitespace(match_char('.'))
                .and_then(|_| name())
                .map(|x| ColumnIdentifier {
                    table_name: Some(table_name.to_string()),
                    name: x.to_string(),
                })
        })
        .or(with_whitespace(name()).map(|x| ColumnIdentifier {
            table_name: None,
            name: x.to_string(),
        }))
}

pub fn schema_name_table_name<'a>() -> Parser<'a, &'a str> {
    with_whitespace(name())
        .and_then(move |_| with_whitespace(match_char('.')).and_then(|_| name()))
        .or(with_whitespace(name()))
}

pub fn join_key_parser<'a>() -> Parser<'a, JoinKey> {
    name().and_then(move |table_name| {
        match_char('.').and_then(move |_| {
            name().map(move |column_name| JoinKey {
                table_name: table_name.to_string(),
                column_name: column_name.to_string(),
            })
        })
    })
}
pub fn join_segment_parser<'a>() -> Parser<'a, JoinSegment> {
    join_key_parser().and_then(move |l| {
        with_whitespace(match_char('=')).and_then({
            move |_| {
                join_key_parser().map({
                    let value = l.clone();
                    move |r| JoinSegment {
                        left_key: value.clone(),
                        right_key: r,
                    }
                })
            }
        })
    })
}

pub fn join_parser<'a>() -> Parser<'a, Join> {
    aliased_table_name_identifier().and_then(move |at| {
        join_list_parser().map(move |segments| Join {
            identifier: at.clone(),
            join_segments: segments,
        })
    })
}

pub fn joins_parser<'a>() -> Parser<'a, Arc<Vec<Join>>> {
    with_whitespace(match_string("join")).and_then(move |_| list(join_parser(), "join"))
}

pub fn join_list_parser<'a>() -> Parser<'a, std::sync::Arc<Vec<JoinSegment>>> {
    with_whitespace(match_string("on")).and_then(move |_| list(join_segment_parser(), "and"))
}

pub fn select_column_list_parser<'a>() -> Parser<'a, std::sync::Arc<Vec<SelectColumn>>> {
    comma_sep(select_column_parser())
}

pub fn select_parser<'a>() -> Parser<'a, Select> {
    with_whitespace(match_string("SELECT")).and_then({
        move |select| {
            select_column_list_parser().and_then({
                move |cols| {
                    with_whitespace(match_string("FROM")).and_then({
                        move |_| {
                            aliased_table_name_identifier().and_then({
                                let cols = cols.clone();
                                move |id| {
                                    joins_parser().map({
                                        let cols = cols.clone();

                                        move |joins| Select {
                                            table_identifier: id.clone(),
                                            select_columns: cols.clone().to_vec(),
                                            joins: joins.to_vec(),
                                            statement: format!("SELECT {}", &select),
                                        }
                                    })
                                }
                            })
                        }
                    })
                }
            })
        }
    })
}

pub fn foreign_key<'a>() -> Parser<'a, ConstraintOrColumn> {
    with_whitespace(match_string("FOREIGN KEY")).and_then(move |_| {
        with_whitespace(match_char('('))
            .and_then(|_| comma_sep(with_whitespace(name())))
            .and_then({
                move |defs| {
                    match_char(')').and_then(move |_| {
                        with_whitespace(match_string("REFERENCES")).and_then({
                            let defs2 = defs.clone();
                            move |_| {
                                with_whitespace(schema_name_table_name()).and_then({
                                    let def3 = defs2.clone();
                                    move |tablename| {
                                        with_whitespace(match_char('(')).and_then({
                                            let def4 = def3.clone();
                                            move |_| {
                                                comma_sep(with_whitespace(name())).and_then({
                                                    let def5 = def4.clone();
                                                    move |columnnames| {
                                                        match_char(')').and_then({
                                                            let columnnames2 = columnnames.clone();
                                                                let def6 = def5.clone();

                                                            move |_| cascade()  .map({
                                                            let def6 = def6.clone();
                                                            let columnnames3 = columnnames2.clone();

                                                                move |_| {
                                                               ConstraintOrColumn::Constraint(Constraint::ForeignKey(ForeignKey {
                                                                    source_columns: def6
                                                                        .to_vec()
                                                                        .into_iter()
                                                                        .map(|s| s.to_string())
                                                                        .collect::<Vec<String>>(),
                                                                    target_table: tablename
                                                                       .to_string(),
                                                                    target_columns: columnnames3
                                                                        .to_vec()
                                                                        .into_iter()
                                                                        .map(|s| s.to_string())
                                                                        .collect::<Vec<String>>(),
                                                                }))
                                                            }
                                                        })})
                                                    }
                                                })
                                            }
                                        })
                                    }
                                })
                            }
                        })
                    })
                }
            })
    })
}
pub fn constraint<'a>() -> Parser<'a, ConstraintOrColumn> {
    with_whitespace(match_string("CONSTRAINT")).and_then({
        move |_| with_whitespace(name()).and_then(|_| foreign_key().or(primary_key()).or(unique()))
    })
}
pub fn add_constraint<'a>() -> Parser<'a, ConstraintOrColumn> {
    with_whitespace(match_string("ADD CONSTRAINT")).and_then({
        move |_| with_whitespace(name()).and_then(|_| foreign_key().or(primary_key()).or(unique()))
    })
}

pub fn cascade<'a>() -> Parser<'a, &'a str> {
    let on_delete_cascade = with_whitespace(match_string("ON DELETE CASCADE"));
    let on_update_cascade = with_whitespace(match_string("ON UPDATE CASCADE"));

    // Match sequence: "ON DELETE CASCADE" followed by "ON UPDATE CASCADE"
    let sequence = on_delete_cascade.clone().and_then({
        let on_update_cascade = on_update_cascade.clone();
        move |_| on_update_cascade.clone()
    });

    // Combine all options, ensuring parsers are cloned appropriately
    sequence
        .or(on_delete_cascade)
        .or(on_update_cascade)
        .or(Parser::new(|input, position| Ok(("", input, position))))
}

pub fn varying<'a>() -> Parser<'a, &'a str> {
    with_whitespace(dtype_name()).and_then(move |dtype| {
        with_whitespace(match_string("VARYING")).and_then(move |_| {
            with_whitespace(match_char('(')).and_then(move |_| {
                with_whitespace(number())
                    .and_then(move |_| with_whitespace(match_char(')')).map(move |_| dtype))
            })
        })
    })
}

pub fn data_type<'a>() -> Parser<'a, &'a str> {
    varying().or(with_whitespace(dtype_name()).and_then(move |dtype| {
        with_whitespace(match_string("WITH TIME ZONE"))
            .or(with_whitespace(match_string("PRECISION")))
            .or(Parser::new(|input, position| Ok(("", input, position))))
            .map(move |_| dtype)
    }))
}
pub fn dtype_name<'a>() -> Parser<'a, &'a str> {
    Parser::new(|input: &'a str, position: usize| {
        let chars = input.chars();
        let mut end = 0;
        for c in chars {
            if c.is_alphanumeric() || c == '_' || c == '"' || c == '[' || c == ']' {
                end += c.len_utf8();
            } else {
                break;
            }
        }
        if end > 0 {
            Ok((&input[..end], &input[end..], position + end))
        } else {
            Err(ParseError::new(
                format!("Found invalid input while looking for name: {}", input),
                position,
            ))
        }
    })
}

pub fn default<'a>() -> Parser<'a, &'a str> {
    with_whitespace(match_string("DEFAULT")).and_then({
        move |_| with_whitespace(number().or(function() .or(Parser::new(|input, position| Ok(("", input, position))   )) )  )
    }).or(Parser::new(|input, position| Ok(("", input, position)  ) ))
}
pub fn select_column_with_alias_with_cast_parser<'a>() -> Parser<'a, SelectColumn> {
    with_whitespace(match_string("cast")).and_then( move |_|{
        match_char('(').and_then(move |_|
            column_name_table_name().and_then(move |col_name|
                {
                    with_whitespace(match_string("as")).and_then( move |_|{
                        let cname = col_name.name.clone();
                        let tname = col_name.table_name.clone(); 
                        data_type().and_then( move |_|{
                            let tname = tname.clone();
                            let cname = cname.clone();
                            match_char(')').and_then(move |_|{
                                let tname = tname.clone();
                                let cname = cname.clone();
                                with_whitespace(match_string("as"))
            .or(with_whitespace(match_string("")))
            .and_then(move |_| name())
            .map(move |alias| SelectColumn {
                name: cname.to_string(),
                table_name: tname.clone(),
                alias: Some(alias.to_string()),
            })

                            })
                        })
                    })
                }
            )
        )
    })

}


pub fn select_column_with_alias_parser<'a>() -> Parser<'a, SelectColumn> {
    column_name_table_name().and_then(move |col_name| {
        with_whitespace(match_string("as"))
            .or(with_whitespace(match_string("")))
            .and_then(move |_| name())
            .map(move |alias| SelectColumn {
                name: col_name.name.to_string(),
                table_name: col_name.table_name.clone(),
                alias: Some(alias.to_string()),
            })
    })
}

pub fn select_column_parser<'a>() -> Parser<'a, SelectColumn> {
   select_column_with_alias_with_cast_parser().or(  select_column_with_alias_parser().or(column_name_table_name().map(move |col_name| {
        SelectColumn {
            name: col_name.name.to_string(),
            table_name: col_name.table_name.clone(),
            alias: None,
        }
    }))
    )
}

pub fn column<'a>() -> Parser<'a, ConstraintOrColumn> {
    with_whitespace(name()).and_then(|colname| {
        with_whitespace(data_type()).and_then(move |dtype| {
            // Capture default value if present

            default().and_then(move |default_value| {
                // Capture NOT NULL constraint if present
                let not_null_parser = with_whitespace(match_string("NOT NULL"))
                    .or(with_whitespace(match_string("NULL")))
                    .or(Parser::new(|input, position| Ok(("", input, position)))); // Changed to return (bool, &str)

                not_null_parser.and_then({
                    let value = Some(default_value.to_string());
                    move |not_null| {
                        // Capture other constraints
                        let constraint_parser = with_whitespace(match_string("PRIMARY KEY"))
                            .or(with_whitespace(match_string("UNIQUE")))
                            .zero_or_more();

                        constraint_parser.map({
                            let value_final = match value.clone().unwrap().len() {
                                0 => None,
                                _ => value.clone(),
                            };
                            move |constraints| {
                                ConstraintOrColumn::Column(Column {
                                    name: colname.replace('"', ""),
                                    colname: colname.to_string(),
                                    dtype: dtype.to_string(),
                                    default: value_final.clone(), // No need to clone here
                                    options: constraints
                                        .into_iter()
                                        .map(|s| s.to_string())
                                        .collect(),
                                })
                            }
                        })
                    }
                })
            })
        })
    })
}

pub fn column_list<'a>() -> Parser<'a, Arc<Vec<ConstraintOrColumn>>> {
    with_whitespace(match_char('('))
        .and_then(|_| comma_sep(constraint().or(column())))
        .and_then(move |cols| with_whitespace(match_char(')')).map(move |_| Arc::clone(&cols)))
}
pub fn alter_table_add_column_parser<'a>() -> Parser<'a, (String, String, String)> {
    with_whitespace(match_string("ALTER TABLE"))
        .and_then(move |_| {
            with_whitespace(name())
                .and_then(|_| {
                    with_whitespace(match_char('.')).and_then(|_| with_whitespace(name()))
                })
                .or(with_whitespace(name()))
        })
        .and_then(move |table_name| {
            add_column().map(move |colname| {
                (
                    table_name.to_string(),
                    colname.to_string(),
                    String::from("GENERATED ALWAYS AS IDENTITY"),
                )
            })
        })
}
pub fn add_column<'a>() -> Parser<'a, &'a str> {
    with_whitespace(match_string("ALTER COLUMN")).and_then({
        move |_| {
            with_whitespace(name()).and_then(move |colname| {
                with_whitespace(match_string("ADD GENERATED ALWAYS AS IDENTITY"))
                    .map({ move |_| colname })
            })
        }
    })
}

pub fn alter_table_parser<'a>() -> Parser<'a, (String, ConstraintOrColumn)> {
    with_whitespace(match_string("ALTER TABLE ONLY"))
        .and_then(move |_| {
            with_whitespace(name())
                .and_then(|_| {
                    with_whitespace(match_char('.')).and_then(|_| with_whitespace(name()))
                })
                .or(with_whitespace(name()))
        })
        .and_then(move |table_name| {
            add_constraint().map(move |constraint| (table_name.to_string(), constraint))
        })
}
pub fn create_view_parser<'a>() -> Parser<'a, View> {
    with_whitespace(match_string("CREATE VIEW"))
        .and_then(move |_| {
            with_whitespace(name())
                .and_then(|_| {
                    with_whitespace(match_char('.')).and_then(|_| with_whitespace(name()))
                })
                .or(with_whitespace(name()))
        })
        .and_then(move |table_name| {

        select_parser().map(move |select|
        View 
        {
                select,
                name: table_name.to_string(),
            }
        )
        })
}



pub fn create_table_parser<'a>() -> Parser<'a, Table> {
    with_whitespace(match_string("CREATE TABLE"))
        .and_then(move |_| {
            with_whitespace(name())
                .and_then(|_| {
                    with_whitespace(match_char('.')).and_then(|_| with_whitespace(name()))
                })
                .or(with_whitespace(name()))
        })
        .and_then(move |table_name| {
            column_list().map(move |columns| {
                let mut column_defs = Vec::new();
                let mut constraints = Vec::new();
                for result in columns.iter().cloned() {
                    match result {
                        ConstraintOrColumn::Column(cd) => column_defs.push(cd),
                        ConstraintOrColumn::Constraint(c) => constraints.push(c),
                    }
                }
                Table {
                    name: table_name.to_string(),
                    columns: column_defs.to_vec(),
                    constraints: constraints.to_vec(),
                }
            })
        })
}

pub fn parse_ddl_file(relative_path: String) -> Vec<(Table, String, usize)> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let file_path = PathBuf::from(manifest_dir).join(relative_path);
    let file_content = fs::read_to_string(file_path).expect("Unable to read file");
    let sql = file_content.split(";");

    let mut tables = Vec::new();
    let mut constraints = Vec::new();
    let mut alter_columns = Vec::new();
    for statement in sql.into_iter() {
        let content = statement.trim_start();
        if content.starts_with("CREATE TABLE") {
            let table = create_table_parser().parse(content);

            match table {
                Ok(a) => {
                    let b = (a.0.clone(), a.1.to_string(), a.2);
                    tables.push(b)
                }
                Err(e) => (), //println!("{}", e),
            }
        } else if content.starts_with("ALTER TABLE") {
            let constraint = alter_table_parser().parse(content);

            match constraint {
                Ok(a) => constraints.push(a),
                Err(_) => {
                    let alter_column = alter_table_add_column_parser().parse(content);
                    match alter_column {
                        Ok(b) => alter_columns.push(b),
                        Err(e) => (), //println!("{}", e),
                    }
                }
            }
        }
    }

    for constraint in constraints {
        for table in &mut tables {
            if constraint.0 .0 == table.0.name {
                match constraint.0 .1 {
                    ConstraintOrColumn::Constraint(ref a) => table.0.constraints.push(a.clone()),
                    _ => continue,
                }
            }
        }
    }

    for col in alter_columns {
        for table in &mut tables {
            if col.0 .0 == table.0.name {
                for tc in &mut table.0.columns {
                    if col.0 .1 == tc.colname {
                        tc.default = Some(col.0 .2.clone());
                    }
                }
            }
        }
    }

    tables
}

// Trie-like structure to represent the hierarchy of tables
#[derive(Clone)]
pub struct TableHierarchy {
    pub table: String,
    pub children: Vec<TableHierarchy>,
}

impl TableHierarchy {
    pub fn new(table: String) -> Self {
        TableHierarchy {
            table,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: TableHierarchy) {
        self.children.push(child);
    }

    pub fn print(&mut self) {
        println!("{}", self.table);
        for mut child in self.children.clone() {
            child.print();
        }
    }
}

// Function to build the table hierarchy
pub fn build_table_hierarchy(select: &Select, tables: &HashMap<String, Table>) -> TableHierarchy {
    let hierarchy = TableHierarchy::new(select.table_identifier.identifier.name.clone());
    let mut table_map = HashMap::new();

    // Initialize the table map with the main table
    table_map.insert(
        select.table_identifier.identifier.name.clone(),
        hierarchy.clone(),
    );

    // Process each join
    for join in &select.joins {
        let left_table = &join.identifier.identifier.name;
        let right_table = &join.identifier.identifier.name;

        // Check if the join key for the left table is a primary key
        let left_is_primary_key = is_primary_key(
            left_table,
            &join.join_segments[0].left_key.column_name,
            tables,
        );

        // Check if the join key for the right table is a primary key
        let right_is_primary_key = is_primary_key(
            right_table,
            &join.join_segments[0].right_key.column_name,
            tables,
        );

        if left_is_primary_key && !right_is_primary_key {
            // Add the right table as a child of the left table
            let child_hierarchy = TableHierarchy::new(right_table.clone());
            if let Some(parent) = table_map.get_mut(left_table) {
                parent.add_child(child_hierarchy.clone());
            }
            table_map.insert(right_table.clone(), child_hierarchy);
        }
    }

    // Rebuild the hierarchy from the table map
    let mut root = None;
    for (table, hierarchy) in table_map {
        if table == select.table_identifier.identifier.name {
            root = Some(hierarchy);
        }
    }

    root.unwrap_or_else(|| TableHierarchy::new(select.table_identifier.identifier.name.clone()))
}

// Function to check if a column is a primary key
fn is_primary_key(table_name: &str, column_name: &str, tables: &HashMap<String, Table>) -> bool {
    if let Some(table) = tables.get(table_name) {
        for constraint in &table.constraints {
            if let Constraint::PrimaryKey(pk) = constraint {
                if pk.columns.contains(&column_name.to_string()) {
                    return true;
                }
            }
        }
    }
    false
}
