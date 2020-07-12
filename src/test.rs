
#[cfg(test)]
mod tests {
    use diesel::mysql::MysqlConnection;
    use diesel::sql_query;
   use  diesel::query_builder::SqlQuery;
    use diesel::query_dsl::LoadQuery;
    use diesel::sql_types::{Integer, Text};
    use diesel::prelude::*;
    #[derive(QueryableByName,Debug,PartialEq)]
    struct Entry {
        #[sql_type = "Text"]
        gua_code: String,
        #[sql_type = "Text"]
        gua_name: String,
    }
    #[test]
    #[ignore]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn test_sql_query_function(){
        let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let connection = MysqlConnection::establish(&database_url).unwrap();
        println!("test");
        let query = "select gua_code,gua_name from six_four_hexagrams";
        let results = sql_query(query).load::<Entry>(&connection);
        println!("{:#?}",results);
    }

}