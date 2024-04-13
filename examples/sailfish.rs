use std::fs::File;
use std::io::Write;

use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "db.stpl")]
struct DatabaseTemplate {
    name: String,
    tables: Vec<TableTemplate>
}

struct TableTemplate {
    name: String,
    columns: Vec<ColumnTemplate>
}

struct ColumnTemplate {
    name: String, consolidated_type: String, is_nullable: bool, is_identity: bool,
    parent: Option<(String, String)>
}

fn main() {
    let ctx = DatabaseTemplate {
        name: "students_db".to_owned(),
        tables: vec![TableTemplate {
            name: "city".to_owned(),
            columns: vec![ColumnTemplate {
                name: "id".to_owned(), consolidated_type: "integer".to_owned(),
                is_nullable: false, is_identity: true, parent: None
            }, ColumnTemplate {
                name: "name".to_owned(), consolidated_type: "text".to_owned(),
                is_nullable: false, is_identity: false, parent: None
            }]
        }, TableTemplate {
            name: "student".to_owned(),
            columns: vec![ColumnTemplate {
                name: "id".to_owned(), consolidated_type: "integer".to_owned(),
                is_nullable: false, is_identity: true, parent: None
            }, ColumnTemplate {
                name: "name".to_owned(), consolidated_type: "text".to_owned(),
                is_nullable: false, is_identity: false, parent: None
            }, ColumnTemplate {
                name: "city_id".to_owned(), consolidated_type: "integer".to_owned(),
                is_nullable: false, is_identity: false, parent: Some(("city".to_owned(), "id".to_owned()))
            }]
        }],
    };

    let mut w = File::create("res.sql").unwrap();
    let res = ctx.render_once().unwrap();

    write!(&mut w, "{}", res).unwrap();
    print!("{}", res);
}