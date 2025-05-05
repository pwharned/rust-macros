
pub mod select_generator {
    //Generates routes and implementations based on complicated select statements which include
    //joins.
    //
use parse;
    pub fn select_route_generator(path: String) {
        let tables = parse::parse_ddl_file(path);
    }
}
