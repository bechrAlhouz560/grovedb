pub mod cli;
pub mod parser;

use sqlparser::{
    ast::Statement,
    dialect::{AnsiDialect, GenericDialect},
    parser::Parser,
};

fn main() {
    let dialect = AnsiDialect {}; // or AnsiDialect

    let sql = "create table if not exists user (name string , password string);";

    let ast = Parser::parse_sql(&dialect, sql).unwrap();

    for state in ast {
        match state {
            Statement::CreateTable {
                or_replace,
                temporary,
                external,
                global,
                if_not_exists,
                transient,
                name,
                columns,
                constraints,
                hive_distribution,
                hive_formats,
                table_properties,
                with_options,
                file_format,
                location,
                query,
                without_rowid,
                like,
                clone,
                engine,
                comment,
                auto_increment_offset,
                default_charset,
                collation,
                on_commit,
                on_cluster,
                order_by,
                strict,
            } => {
                println!("name = {} {:#?}", name, columns)
            }

            (_) => {}
        }
    }
}
