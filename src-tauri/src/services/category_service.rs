use diesel::prelude::*;

use crate::db::schema::{categories, category_rules, transactions};
use crate::error::BudgyError;
use crate::models::category::{Category, CreateCategoryInput, NewCategory, UpdateCategory};

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

pub fn get_by_id(
    conn: &mut SqliteConnection,
    id: i32,
) -> Result<Option<Category>, BudgyError> {
    categories::table
        .find(id)
        .first::<Category>(conn)
        .optional()
        .map_err(BudgyError::from)
}

pub fn create(
    conn: &mut SqliteConnection,
    input: &CreateCategoryInput,
) -> Result<Category, BudgyError> {
    let new = NewCategory {
        name: &input.name,
        parent_id: input.parent_id,
        icon: input.icon.as_deref(),
        color: input.color.as_deref(),
    };

    diesel::insert_into(categories::table)
        .values(&new)
        .execute(conn)?;

    categories::table
        .order(categories::id.desc())
        .first::<Category>(conn)
        .map_err(BudgyError::from)
}

pub fn update(
    conn: &mut SqliteConnection,
    id: i32,
    changes: &UpdateCategory,
) -> Result<Category, BudgyError> {
    diesel::update(categories::table.find(id))
        .set(changes)
        .execute(conn)?;

    categories::table
        .find(id)
        .first::<Category>(conn)
        .map_err(BudgyError::from)
}

pub fn delete(
    conn: &mut SqliteConnection,
    id: i32,
    reassign_to_id: i32,
) -> Result<(), BudgyError> {
    let uncategorized = get_by_name(conn, "Uncategorized")?.ok_or_else(|| {
        BudgyError::General("Uncategorized category not found".to_string())
    })?;

    if id == uncategorized.id {
        return Err(BudgyError::General(
            "Cannot delete the Uncategorized category".to_string(),
        ));
    }

    if reassign_to_id == id {
        return Err(BudgyError::General(
            "Cannot reassign transactions to the category being deleted".to_string(),
        ));
    }

    // Verify the target category exists
    get_by_id(conn, reassign_to_id)?.ok_or_else(|| {
        BudgyError::General("Target reassignment category does not exist".to_string())
    })?;

    // Reassign transactions to chosen category
    diesel::update(transactions::table.filter(transactions::category_id.eq(id)))
        .set(transactions::category_id.eq(reassign_to_id))
        .execute(conn)?;

    // Detach child categories
    diesel::update(categories::table.filter(categories::parent_id.eq(id)))
        .set(categories::parent_id.eq(None::<i32>))
        .execute(conn)?;

    // Delete category rules
    diesel::delete(category_rules::table.filter(category_rules::category_id.eq(id)))
        .execute(conn)?;

    // Delete the category
    diesel::delete(categories::table.find(id)).execute(conn)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::connection::establish_test_connection;
    use crate::models::category_rule::NewCategoryRule;

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

    #[test]
    fn test_get_by_id_found() {
        let conn = &mut establish_test_connection();
        let all = list_all(conn).unwrap();
        let first = &all[0];
        let found = get_by_id(conn, first.id).unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, first.name);
    }

    #[test]
    fn test_get_by_id_not_found() {
        let conn = &mut establish_test_connection();
        let found = get_by_id(conn, 99999).unwrap();
        assert!(found.is_none());
    }

    #[test]
    fn test_create_basic() {
        let conn = &mut establish_test_connection();
        let input = CreateCategoryInput {
            name: "Test Category".to_string(),
            parent_id: None,
            icon: None,
            color: Some("#ff0000".to_string()),
        };
        let cat = create(conn, &input).unwrap();
        assert_eq!(cat.name, "Test Category");
        assert_eq!(cat.color, Some("#ff0000".to_string()));
        assert_eq!(cat.parent_id, None);
    }

    #[test]
    fn test_create_with_parent() {
        let conn = &mut establish_test_connection();
        let parent = get_by_name(conn, "Food & Groceries").unwrap().unwrap();
        let input = CreateCategoryInput {
            name: "Restaurants".to_string(),
            parent_id: Some(parent.id),
            icon: None,
            color: None,
        };
        let cat = create(conn, &input).unwrap();
        assert_eq!(cat.name, "Restaurants");
        assert_eq!(cat.parent_id, Some(parent.id));
    }

    #[test]
    fn test_create_duplicate_name_fails() {
        let conn = &mut establish_test_connection();
        let input = CreateCategoryInput {
            name: "Income".to_string(),
            parent_id: None,
            icon: None,
            color: None,
        };
        let result = create(conn, &input);
        assert!(result.is_err());
    }

    #[test]
    fn test_update_name_and_color() {
        let conn = &mut establish_test_connection();
        let input = CreateCategoryInput {
            name: "Updatable".to_string(),
            parent_id: None,
            icon: None,
            color: Some("#000000".to_string()),
        };
        let cat = create(conn, &input).unwrap();

        let changes = UpdateCategory {
            name: "Updated Name".to_string(),
            parent_id: None,
            icon: None,
            color: Some("#ffffff".to_string()),
        };
        let updated = update(conn, cat.id, &changes).unwrap();
        assert_eq!(updated.name, "Updated Name");
        assert_eq!(updated.color, Some("#ffffff".to_string()));
    }

    #[test]
    fn test_update_duplicate_name_fails() {
        let conn = &mut establish_test_connection();
        let input = CreateCategoryInput {
            name: "UniqueForUpdate".to_string(),
            parent_id: None,
            icon: None,
            color: None,
        };
        let cat = create(conn, &input).unwrap();

        let changes = UpdateCategory {
            name: "Income".to_string(),
            parent_id: None,
            icon: None,
            color: None,
        };
        let result = update(conn, cat.id, &changes);
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_reassigns_transactions() {
        let conn = &mut establish_test_connection();

        // Create a category to delete and a target category
        let input = CreateCategoryInput {
            name: "ToDelete".to_string(),
            parent_id: None,
            icon: None,
            color: None,
        };
        let cat = create(conn, &input).unwrap();

        let target_input = CreateCategoryInput {
            name: "ReassignTarget".to_string(),
            parent_id: None,
            icon: None,
            color: None,
        };
        let target = create(conn, &target_input).unwrap();

        // Create a transaction assigned to this category (need account + type first)
        use crate::db::schema::{accounts, transaction_types};
        use crate::models::account::NewAccount;
        use crate::models::transaction::NewTransaction;

        diesel::insert_into(accounts::table)
            .values(&NewAccount {
                iban: "BE00000000000000",
                label: None,
                currency: "EUR",
            })
            .execute(conn)
            .unwrap();
        let acct: crate::models::account::Account = accounts::table
            .order(accounts::id.desc())
            .first(conn)
            .unwrap();

        let tx_type: crate::models::transaction_type::TransactionType =
            transaction_types::table.first(conn).unwrap();

        let new_tx = NewTransaction {
            account_id: acct.id,
            counterparty_id: None,
            category_id: Some(cat.id),
            transaction_type_id: tx_type.id,
            accounting_date: "2024-01-01",
            value_date: "2024-01-01",
            statement_number: None,
            transaction_number: None,
            amount_cents: -1000,
            currency: "EUR",
            description: "Test transaction",
            communication: None,
            import_hash: "delete_test_hash_001",
        };

        diesel::insert_into(transactions::table)
            .values(&new_tx)
            .execute(conn)
            .unwrap();

        delete(conn, cat.id, target.id).unwrap();

        // Transaction should now point to the target category
        let tx: crate::models::transaction::Transaction = transactions::table
            .filter(transactions::import_hash.eq("delete_test_hash_001"))
            .first(conn)
            .unwrap();
        assert_eq!(tx.category_id, Some(target.id));

        // Category should be gone
        assert!(get_by_id(conn, cat.id).unwrap().is_none());
    }

    #[test]
    fn test_delete_uncategorized_rejected() {
        let conn = &mut establish_test_connection();
        let uncategorized = get_by_name(conn, "Uncategorized").unwrap().unwrap();
        let income = get_by_name(conn, "Income").unwrap().unwrap();
        let result = delete(conn, uncategorized.id, income.id);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Cannot delete"));
    }

    #[test]
    fn test_delete_detaches_children() {
        let conn = &mut establish_test_connection();

        let uncategorized = get_by_name(conn, "Uncategorized").unwrap().unwrap();

        let parent_input = CreateCategoryInput {
            name: "ParentToDelete".to_string(),
            parent_id: None,
            icon: None,
            color: None,
        };
        let parent = create(conn, &parent_input).unwrap();

        let child_input = CreateCategoryInput {
            name: "ChildOfParent".to_string(),
            parent_id: Some(parent.id),
            icon: None,
            color: None,
        };
        let child = create(conn, &child_input).unwrap();
        assert_eq!(child.parent_id, Some(parent.id));

        delete(conn, parent.id, uncategorized.id).unwrap();

        let child_after = get_by_id(conn, child.id).unwrap().unwrap();
        assert_eq!(child_after.parent_id, None);
    }

    #[test]
    fn test_delete_removes_rules() {
        let conn = &mut establish_test_connection();

        let uncategorized = get_by_name(conn, "Uncategorized").unwrap().unwrap();

        let input = CreateCategoryInput {
            name: "RuleTarget".to_string(),
            parent_id: None,
            icon: None,
            color: None,
        };
        let cat = create(conn, &input).unwrap();

        let rule = NewCategoryRule {
            category_id: cat.id,
            match_field: "description",
            match_pattern: "TEST",
            priority: 1,
        };
        diesel::insert_into(category_rules::table)
            .values(&rule)
            .execute(conn)
            .unwrap();

        let rule_count: i64 = category_rules::table
            .filter(category_rules::category_id.eq(cat.id))
            .count()
            .get_result(conn)
            .unwrap();
        assert_eq!(rule_count, 1);

        delete(conn, cat.id, uncategorized.id).unwrap();

        let rule_count_after: i64 = category_rules::table
            .filter(category_rules::category_id.eq(cat.id))
            .count()
            .get_result(conn)
            .unwrap();
        assert_eq!(rule_count_after, 0);
    }

    #[test]
    fn test_delete_rejects_self_reassignment() {
        let conn = &mut establish_test_connection();

        let input = CreateCategoryInput {
            name: "SelfReassign".to_string(),
            parent_id: None,
            icon: None,
            color: None,
        };
        let cat = create(conn, &input).unwrap();

        let result = delete(conn, cat.id, cat.id);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Cannot reassign transactions to the category being deleted"));
    }

    #[test]
    fn test_delete_rejects_invalid_reassignment() {
        let conn = &mut establish_test_connection();

        let input = CreateCategoryInput {
            name: "InvalidTarget".to_string(),
            parent_id: None,
            icon: None,
            color: None,
        };
        let cat = create(conn, &input).unwrap();

        let result = delete(conn, cat.id, 99999);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Target reassignment category does not exist"));
    }
}
