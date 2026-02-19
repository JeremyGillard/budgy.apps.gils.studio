use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::db::schema::categories;

#[derive(Queryable, Selectable, Identifiable, Debug, Serialize, Deserialize)]
#[diesel(table_name = categories)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub parent_id: Option<i32>,
    pub icon: Option<String>,
    pub color: Option<String>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = categories)]
pub struct NewCategory<'a> {
    pub name: &'a str,
    pub parent_id: Option<i32>,
    pub icon: Option<&'a str>,
    pub color: Option<&'a str>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCategoryInput {
    pub name: String,
    pub parent_id: Option<i32>,
    pub icon: Option<String>,
    pub color: Option<String>,
}

#[derive(AsChangeset, Debug, Deserialize)]
#[diesel(table_name = categories)]
pub struct UpdateCategory {
    pub name: String,
    pub parent_id: Option<i32>,
    pub icon: Option<String>,
    pub color: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::connection::establish_test_connection;
    

    #[test]
    fn test_seed_categories_exist() {
        let conn = &mut establish_test_connection();

        let results: Vec<Category> = categories::table
            .select(Category::as_select())
            .load(conn)
            .expect("Failed to load categories");

        assert!(results.len() >= 11, "Seed categories should be present");

        let names: Vec<&str> = results.iter().map(|c| c.name.as_str()).collect();
        assert!(names.contains(&"Food & Groceries"));
        assert!(names.contains(&"Income"));
    }

    #[test]
    fn test_parent_child_relationship() {
        let conn = &mut establish_test_connection();

        let food = categories::table
            .filter(categories::name.eq("Food & Groceries"))
            .first::<Category>(conn)
            .expect("Food category should exist");

        let child = NewCategory {
            name: "Restaurants",
            parent_id: Some(food.id),
            icon: None,
            color: None,
        };

        diesel::insert_into(categories::table)
            .values(&child)
            .execute(conn)
            .expect("Failed to insert child category");

        let restaurant: Category = categories::table
            .filter(categories::name.eq("Restaurants"))
            .first(conn)
            .expect("Restaurant category should exist");

        assert_eq!(restaurant.parent_id, Some(food.id));
    }

    #[test]
    fn test_name_uniqueness() {
        let conn = &mut establish_test_connection();

        let dup = NewCategory {
            name: "Income",
            parent_id: None,
            icon: None,
            color: None,
        };

        let result = diesel::insert_into(categories::table)
            .values(&dup)
            .execute(conn);

        assert!(result.is_err(), "Duplicate category name should fail");
    }
}
