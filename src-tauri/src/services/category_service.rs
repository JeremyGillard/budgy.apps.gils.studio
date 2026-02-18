use diesel::prelude::*;

use crate::db::schema::categories;
use crate::error::BudgyError;
use crate::models::category::Category;

pub fn list_all(conn: &mut SqliteConnection) -> Result<Vec<Category>, BudgyError> {
    categories::table
        .select(Category::as_select())
        .load(conn)
        .map_err(BudgyError::from)
}

pub fn get_by_name(
    conn: &mut SqliteConnection,
    name: &str,
) -> Result<Option<Category>, BudgyError> {
    categories::table
        .filter(categories::name.eq(name))
        .first::<Category>(conn)
        .optional()
        .map_err(BudgyError::from)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::connection::establish_test_connection;

    #[test]
    fn test_list_all_seeded() {
        let conn = &mut establish_test_connection();
        let all = list_all(conn).unwrap();
        assert!(all.len() >= 11);
    }

    #[test]
    fn test_get_by_name() {
        let conn = &mut establish_test_connection();
        let cat = get_by_name(conn, "Income").unwrap();
        assert!(cat.is_some());
        assert_eq!(cat.unwrap().name, "Income");
    }

    #[test]
    fn test_get_by_name_not_found() {
        let conn = &mut establish_test_connection();
        let cat = get_by_name(conn, "Nonexistent").unwrap();
        assert!(cat.is_none());
    }
}
