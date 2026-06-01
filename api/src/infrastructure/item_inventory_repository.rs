use crate::domain::item_inventory::ItemInventoryRepository;
use crate::domain::ItemInventoryCommand::{Create, Modify, Replace};
use crate::domain::Mutation::{Assign, Clear, Retain};
use crate::domain::{Item, ItemInventory, ItemInventoryCommand, Page, Pageable};
use crate::transaction::TransactionContext;
use async_trait::async_trait;
use sqlx::postgres::PgArguments;
use sqlx::{query, query_as, query_scalar, query_with, Arguments, AssertSqlSafe, FromRow, PgPool};
use uuid::Uuid;

pub struct PgItemInventoryRepository {
    pool: PgPool,
}

impl PgItemInventoryRepository {
    pub fn new(
        pool: PgPool
    ) -> Self {
        Self { pool }
    }
}

#[derive(Debug, FromRow)]
struct ItemInventoryRow {
    inventory_id: Uuid,
    inventory_quantity: i32,
    item_id: Uuid,
    item_code: i32,
    item_name: String,
    item_price: Option<i32>,
    item_sell_price: Option<i32>,
}

#[async_trait]
impl ItemInventoryRepository for PgItemInventoryRepository {
    async fn find_by_id(&self, id: &Uuid) -> anyhow::Result<Option<ItemInventory>> {
        let item_inventory = TransactionContext::execute_with(
            &self.pool, async |conn| {
                query_as::<_, ItemInventoryRow>(r#"
                    SELECT
                        inventories.id          AS inventory_id,
                        inventories.quantity    AS inventory_quantity,
                        items.id                AS item_id,
                        items.code              AS item_code,
                        items.name              AS item_name,
                        items.price             AS item_price,
                        items.sell_price        AS item_sell_price
                    FROM
                        item_inventories inventories
                    INNER JOIN
                        items items
                        ON inventories.item_id = items.id
                    WHERE
                        inventories.id = $1
                    "#)
                    .bind(id)
                    .fetch_optional(conn)
                    .await
            })
            .await?
            .map(|row| ItemInventory::of(
                row.inventory_id,
                Item::of(
                    row.item_id,
                    row.item_code,
                    row.item_name,
                    row.item_price,
                    row.item_sell_price,
                ),
                row.inventory_quantity,
            ));

        Ok(item_inventory)
    }

    async fn find_by_item_id(&self, item_id: &Uuid) -> anyhow::Result<Option<ItemInventory>> {
        let item_inventory = TransactionContext::execute_with(
            &self.pool, async |conn| {
                query_as::<_, ItemInventoryRow>(r#"
                    SELECT
                        inventories.id          AS inventory_id,
                        inventories.quantity    AS inventory_quantity,
                        items.id                AS item_id,
                        items.code              AS item_code,
                        items.name              AS item_name,
                        items.price             AS item_price,
                        items.sell_price        AS item_sell_price
                    FROM
                        item_inventories inventories
                    INNER JOIN
                        items items
                        ON inventories.item_id = items.id
                    WHERE
                        inventories.item_id = $1
                    "#)
                    .bind(item_id)
                    .fetch_optional(conn)
                    .await
            })
            .await?
            .map(|row| ItemInventory::of(
                    row.inventory_id,
                    Item::of(
                        row.item_id,
                        row.item_code,
                        row.item_name,
                        row.item_price,
                        row.item_sell_price,
                    ),
                    row.inventory_quantity,
                ));

        Ok(item_inventory)
    }

    async fn find_all(&self, pageable: &Pageable) -> anyhow::Result<Page<ItemInventory>> {
        let count = TransactionContext::execute_with(
            &self.pool, async |conn| {
                query_scalar(r#"
                    SELECT
                        COUNT(*)
                    FROM
                        item_inventories inventories
                    INNER JOIN
                        items items
                        ON inventories.item_id = items.id
                    "#)
                    .fetch_one(conn)
                    .await
            })
            .await?;

        let item_inventories = TransactionContext::execute_with(
            &self.pool, async |conn| {
                query_as::<_, ItemInventoryRow>(r#"
                    SELECT
                        inventories.id          AS inventory_id,
                        inventories.quantity    AS inventory_quantity,
                        items.id                AS item_id,
                        items.code              AS item_code,
                        items.name              AS item_name,
                        items.price             AS item_price,
                        items.sell_price        AS item_sell_price
                    FROM
                        item_inventories inventories
                    INNER JOIN
                        items items
                        ON inventories.item_id = items.id
                    LIMIT $1
                    OFFSET $2
                    "#)
                    .bind(pageable.size)
                    .bind(pageable.offset())
                    .fetch_all(conn)
                    .await
            })
            .await?
            .into_iter()
                .map(|row| ItemInventory::of(
                    row.inventory_id,
                    Item::of(
                        row.item_id,
                        row.item_code,
                        row.item_name,
                        row.item_price,
                        row.item_sell_price,
                    ),
                    row.inventory_quantity,
                ))
                .collect();

        Ok(Page::new(item_inventories, pageable, count))
    }

    async fn find_all_by_name_starting_with(&self, name: &String, pageable: &Pageable) -> anyhow::Result<Page<ItemInventory>> {
        let count = TransactionContext::execute_with(
            &self.pool, async |conn| {
                query_scalar(r#"
                    SELECT
                        COUNT(*)
                    FROM
                        item_inventories inventories
                    INNER JOIN
                        items items
                        ON inventories.item_id = items.id
                    WHERE
                        items.name LIKE $1 || '%'
                    "#)
                    .bind(name)
                    .fetch_one(conn)
                    .await
            })
            .await?;

        let item_inventories = TransactionContext::execute_with(
            &self.pool, async |conn| {
                query_as::<_, ItemInventoryRow>(r#"
                    SELECT
                        inventories.id          AS inventory_id,
                        inventories.quantity    AS inventory_quantity,
                        items.id                AS item_id,
                        items.code              AS item_code,
                        items.name              AS item_name,
                        items.price             AS item_price,
                        items.sell_price        AS item_sell_price
                    FROM
                        item_inventories inventories
                    INNER JOIN
                        items items
                        ON inventories.item_id = items.id
                    WHERE
                        items.name LIKE $1 || '%'
                    LIMIT $2
                    OFFSET $3
                    "#)
                    .bind(name)
                    .bind(pageable.size)
                    .bind(pageable.offset())
                    .fetch_all(conn)
                    .await
            })
            .await?
            .into_iter()
                .map(|row| ItemInventory::of(
                    row.inventory_id,
                    Item::of(
                        row.item_id,
                        row.item_code,
                        row.item_name,
                        row.item_price,
                        row.item_sell_price,
                    ),
                    row.inventory_quantity,
                ))
                .collect();

        Ok(Page::new(item_inventories, pageable, count))
    }

    async fn save(&self, item_inventory_command: &ItemInventoryCommand) -> anyhow::Result<ItemInventory> {
        let id = match item_inventory_command {
            Create(item_inventory) => {
                TransactionContext::execute_with(
                    &self.pool, async |conn| {
                        query(r#"
                            INSERT INTO item_inventories (
                                id,
                                item_id,
                                quantity
                            ) VALUES (
                                $1,
                                $2,
                                $3
                            )
                            "#)
                            .bind(item_inventory.id)
                            .bind(item_inventory.item.id)
                            .bind(item_inventory.quantity)
                            .execute(conn)
                            .await
                    })
                    .await?;

                item_inventory.id
            }
            Replace(item_inventory) => {
                TransactionContext::execute_with(
                    &self.pool, async |conn| {
                        query(r#"
                            UPDATE item_inventories
                            SET
                                item_id = $2,
                                quantity = $3
                            WHERE
                                id = $1
                            "#)
                            .bind(item_inventory.id)
                            .bind(item_inventory.item.id)
                            .bind(item_inventory.quantity)
                            .execute(conn)
                            .await
                    })
                    .await?;

                item_inventory.id
            }
            Modify(item_inventory) => {
                if item_inventory.is_all_retained() {
                    return self.find_by_id(&item_inventory.id)
                        .await?
                        .ok_or_else(|| anyhow::anyhow!("Not found"))
                }

                let mut args = PgArguments::default();
                let mut arg_queries = Vec::new();
                let mut arg_index = 2; // Use $1 in WHERE clause
                let _ = args.add(item_inventory.id);

                match &item_inventory.item {
                    Retain => {}
                    Clear => {
                        let _ = args.add(None::<Uuid>);
                        arg_queries.push(format!("item_id = ${arg_index}"));
                        arg_index += 1;
                    }
                    Assign(param) => {
                        let _ = args.add(param.id);
                        arg_queries.push(format!("item_id = ${arg_index}"));
                        arg_index += 1;
                    }
                }

                match &item_inventory.quantity {
                    Retain => {}
                    Clear => {
                        let _ = args.add(None::<Uuid>);
                        arg_queries.push(format!("quantity = ${arg_index}"));
                        //arg_index += 1;   // Not use
                    }
                    Assign(param) => {
                        let _ = args.add(param);
                        arg_queries.push(format!("quantity = ${arg_index}"));
                        //arg_index += 1;   // Not use
                    }
                }

                TransactionContext::execute_with(
                    &self.pool, async |conn| {
                        query_with(
                            AssertSqlSafe(format!(r#"
                                UPDATE item_inventories
                                SET
                                    {}
                                WHERE
                                    id = $1
                                "#, arg_queries.join(", ")
                            )),
                            args)
                            .execute(conn)
                            .await
                    })
                    .await?;

                item_inventory.id
            }
        };

        self.find_by_id(&id).await?
            .ok_or_else(|| anyhow::anyhow!("Not found"))
    }
}
