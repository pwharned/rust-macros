use parse::{
    build_table_hierarchy, joins_parser,
    alter_table_add_column_parser, cascade,select_column_parser, parse_ddl_file, column, column_list, column_name_table_name, comma_sep,
    constraint, create_table_parser, select_column_with_alias_with_cast_parser, default, foreign_key, function, match_char, match_string, SelectColumn,  join_list_parser,  name, create_view_parser, schema_name_table_name, with_whitespace, ForeignKey, ParseError, Parser, join_parser, select_parser
};
use std::collections::HashMap;
mod tests {
    use super::*;

    use std::fs;

    fn process_reult<'a, A>(res: Result<A, ParseError>)
    where
        A: 'a + std::fmt::Debug,
    {
        match res {
            Ok(a) => println!("{:?}", a),
            Err(e) => panic!("{:?}", e),
        }
    }
    fn print_result<'a, A>(res: Result<A, ParseError>)
    where
        A: 'a + std::fmt::Debug,
    {
        match res {
            Ok(a) => println!("{:?}", a),
            Err(e) => println!("{:?}", e),
        }
    }

    #[test]
    fn test_alter_column() {
        let res = alter_table_add_column_parser().parse(
            " ALTER TABLE public.parent ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public.parent_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);",
        );
        process_reult(res);
    }

    use std::path::PathBuf;
    #[test]
    fn testSelectParser() {
        let select_parser = with_whitespace(match_string("SELECT"))
            .and_then(|_| with_whitespace(match_string("*")));
        let result = select_parser.parse(" SELECT   * ");

        process_reult(result);
    }

    #[test]
    fn test_constraint_parser() {
        let constraints = "CONSTRAINT asset_types_pkey PRIMARY KEY (type_id) ";
        process_reult(constraint().or(column()).parse(constraints))
    }
    #[test]
    fn testOrParser() {
        let or = match_string("HELLO")
            .or(match_string("GOODBYE"))
            .or(match_string("FOO"));
    }

    #[test]
    fn test_comma() {
        let commavals = "FOREIGN KEY (HELLO,GOODBYE ) REFERENCES TABLE(TEST, TEST)";
        let column_parser = with_whitespace(match_char('('))
            .and_then(|_| comma_sep(with_whitespace(parse::name())))
            .and_then({ move |defs| match_char(')').map({ move |_| defs.clone() }) });

        let references_parser = with_whitespace(match_string("REFERENCES"));

        let result = foreign_key().parse(commavals);
    }

    #[test]
    fn test_column() {
        process_reult(column().parse("action_id uuid DEFAULT gen_random_uuid() NOT NULL"));
        process_reult(column().parse("action_user text NOT NULL"));
        process_reult(column().parse("created_at timestamp with time zone DEFAULT now() NOT NULL"));
        process_reult(column().parse("related_asset uuid NOT NULL"));
        process_reult(column().parse("action_type text NOT NULL"));

        process_reult(comma_sep(column() ).parse("action_id uuid DEFAULT gen_random_uuid() NOT NULL,action_user text NOT NULL,created_at timestamp with time zone DEFAULT now() NOT NULL,related_asset uuid NOT NULL,action_type text NOT NULL
"));
    }

    #[test]
    fn test_comma_seperated_columns() {
        let result = comma_sep(column().or(constraint())).parse(
            " action_id uuid DEFAULT gen_random_uuid() NOT NULL,
    action_user text NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    related_asset uuid NOT NULL,
    action_type text NOT NULL
",
        );
    }

    #[test]
    fn test_column_in_parentheses() {
        process_reult(
            match_char('(')
                .and_then(move |_| comma_sep(column().or(constraint())))
                .parse(
                    "(action_id uuid DEFAULT gen_random_uuid() NOT NULL,
    action_user text NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    related_asset uuid NOT NULL,
    action_type text NOT NULL)
",
                ),
        );
    }

    #[test]
    fn function_parser() {
        let input = "get_random_uuid()";

        let result = function().parse(input);
        process_reult(result);
    }

    #[test]
    fn name_parser() {
        let input = "\\\"hello\"";
        name().parse(input);
    }

    #[test]

    fn default_test() {
        process_reult(default().parse("DEFAULT gen_random_uuid()"));
    }
    #[test]
    fn test_cascade() {
        process_reult(cascade().parse("ON DELETE CASCADE"));
        process_reult(cascade().parse("ON DELETE CASCADE ON UPDATE CASCADE"));
        process_reult(cascade().parse("ON UPDATE CASCADE"));
        process_reult(cascade().parse(""));
    }
    #[test]
    fn test_create_table_parser() {
        let relative_path = "../server/schema.sql";

        // Construct the absolute path
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
        let file_path = PathBuf::from(manifest_dir).join(relative_path);
        // Read the JSON file
        let file_content = fs::read_to_string(file_path).expect("Unable to read file");
        let sql = file_content.split(";");

        for stmt in sql {
            let clean = stmt.to_string();

            if clean.trim_start().is_empty() {
                continue;
            }

            let parsed = create_table_parser().parse(&clean);
            //print_result(parsed);
        }
        // Generate structs based on the JSON data
    }

    #[test]
    fn test_select_parser() {
        let relative_path = "../server/schema.sql";

        // Construct the absolute path
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
        let file_path = PathBuf::from(manifest_dir).join(relative_path);
        // Read the JSON file
        let file_content = fs::read_to_string(file_path).expect("Unable to read file");
        let sql = "select a.asset_id, at.name, av.value from public.entityattributes e join public.assets a on e.eid = a.asset_id join public.attributevalues av on av.aid = e.aid join public.attributes at on at.id = av.aid";



            let result = select_column_parser().parse("AV.ID AS ID2");

        match result {
            Ok(a) => assert!(a.0.name == "ID"),
            _ => panic!("Parsing failed"),
        }


        let column_alias_list = comma_sep(select_column_parser());

        let result  = column_alias_list.parse("AV.ID, AV.ID as ID1");

        match result {
            Ok(a) => {
                    assert!(a.0[0].name == "ID"); 
                    assert!(a.0[1].name == "ID");
                    assert!(a.0[1].table_name == Some("AV".to_string()));
            },
            _ => panic!("Parsing failed"),
        }



        let res = join_list_parser().parse("on ev.id = ac.id2 and ev.id2 = ac.id");

        match res {
            Ok(a) => {
                assert!(a.0[0].right_key.table_name == "ac")
            },
            _ => panic!("Parsing failed")
        }





        let res = select_parser().parse("select a.id, b.id as id2 FROM public.test a join public.test b on a.id = b.id and a.test = b.test join public.test c on c.id = a.id");

        match res {
            Ok(a) => {
                assert!(a.0.table_identifier.alias == Some("a".to_string()) );
                assert!(a.0.table_identifier.identifier.name == "test" );
                print!("{:?}", a.0);
                assert!(a.0.joins.len()==2 );
            },
            Err(e) => panic!("{}", e)
        }

    }

#[test]
    pub fn test_tree() {

        let table_vec  = parse_ddl_file("../server/schema.sql".to_string());
        let sql = "select a.asset_id, at.name, av.value as value FROM public.entityattributes e join public.assets a on e.eid = a.asset_id join public.attributevalues av on av.aid = e.aid join public.attributes at on at.id = av.aid";
        let select = match select_parser().parse(sql){
            Ok(a) => { println!("{:?}", a.0); a.0},
            Err(e) => panic!("{}", e)
        };

        

        let mut table_hash = HashMap::new();
        for table in table_vec{
            table_hash.insert(table.0.name.clone(), table.0.clone());
        }
        let mut hierachy = build_table_hierarchy(&select, &table_hash);

        hierachy.print();
    }

}
#[test]
    pub fn test_select_with_cast_view() {


    let result = select_column_with_alias_with_cast_parser().parse("cast(a as text) as a");
    match result{
        Ok(a) => println!("{:?}", a),
        Err(e) => panic!("{}",e)
    }

    match select_column_with_alias_with_cast_parser().parse("casta as text) as a"){
        Ok(a) => panic!("Invalid parse {:?}",a),
        Err(e) => println!("{:?}", e)
    }
        


}
#[test]
    pub fn test_create_view() {

        let table_vec  = parse_ddl_file("../server/schema.sql".to_string());
        let sql = "create view ttest select cast(a.asset_id as text) as asset_id, at.name, av.value as value FROM public.entityattributes e join public.assets a on e.eid = a.asset_id join public.attributevalues av on av.aid = e.aid join public.attributes at on at.id = av.aid";
        let select = match create_view_parser().parse(sql){
            Ok(a) => { println!("{:?}", a.0); a.0},
            Err(e) => panic!("{}", e)
        };

        


}
