use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::db::schema::category_rules;

#[derive(Queryable, Selectable, Identifiable, Debug, Serialize, Deserialize)]
#[diesel(table_name = category_rules)]
pub struct CategoryRule {
    pub id: i32,
    pub category_id: i32,
    pub match_field: String,
    pub match_pattern: String,
    pub priority: i32,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = category_rules)]
pub struct NewCategoryRule<'a> {
    pub category_id: i32,
    pub match_field: &'a str,
    pub match_pattern: &'a str,
    pub priority: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::connection::establish_test_connection;
    use crate::db::schema::categories;
    use crate::models::category::Category;
    

    #[test]
    fn test_insert_and_query_category_rule() {
        let conn = &mut establish_test_connection();

        let food: Category = categories::table
            .filter(categories::name.eq("Food & Groceries"))
            .first(conn)
            .expect("Food category should exist from seeds");

        let new = NewCategoryRule {
            category_id: food.id,
            match_field: "counterparty_name",
            match_pattern: "COLRUYT",
            priority: 10,
        };

        diesel::insert_into(category_rules::table)
            .values(&new)
            .execute(conn)
            .expect("Failed to insert category rule");

        let results: Vec<CategoryRule> = category_rules::table
            .select(CategoryRule::as_select())
            .load(conn)
            .expect("Failed to load rules");

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].match_pattern, "COLRUYT");
        assert_eq!(results[0].category_id, food.id);
    }
}
