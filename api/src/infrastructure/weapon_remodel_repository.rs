use crate::domain::{Item, Page, Pageable, Weapon, WeaponMaterial, WeaponRemodel, WeaponRemodelRepository};
use crate::transaction::TransactionContext;
use async_trait::async_trait;
use sqlx::{query_as, query_scalar, FromRow, PgPool};
use uuid::Uuid;

pub struct PgWeaponRemodelRepository {
    pool: PgPool,
}

impl PgWeaponRemodelRepository {
    pub fn new(
        pool: PgPool
    ) -> Self {
        Self { pool }
    }
}

#[derive(Debug, FromRow)]
struct WeaponRemodelRow {
    remodel_id: Uuid,
    remodel_price: i32,
    weapon_id: Uuid,
    weapon_name: String,
    weapon_str: i32,
    weapon_hit: i32,
}

#[derive(Debug, FromRow)]
struct WeaponMaterialRow {
    material_id: Uuid,
    material_quantity: i32,
    item_id: Uuid,
    item_code: i32,
    item_name: String,
    item_price: Option<i32>,
    item_sell_price: Option<i32>,
}

#[async_trait]
impl WeaponRemodelRepository for PgWeaponRemodelRepository {
    async fn find_by_id(&self, id: &Uuid) -> anyhow::Result<Option<WeaponRemodel>> {
        let remodel_row = TransactionContext::execute_with(
            &self.pool, async |conn| {
                query_as::<_, WeaponRemodelRow>(r#"
                    SELECT
                        remodels.id         AS remodel_id,
                        remodels.price      AS remodel_price,
                        weapons.id          AS weapon_id,
                        weapons.name        AS weapon_name,
                        weapons.str         AS weapon_str,
                        weapons.hit         AS weapon_hit
                    FROM
                        weapon_remodels remodels
                    INNER JOIN
                        weapons weapons
                        ON remodels.weapon_id = weapons.id
                    WHERE
                        remodels.id = $1
                    "#)
                    .bind(id)
                    .fetch_optional(conn)
                    .await
            })
            .await?;

        let remodel_row = match remodel_row {
            Some(row) => row,
            None => return Ok(None),
        };

        let material_rows = TransactionContext::execute_with(
            &self.pool, async |conn| {
                query_as::<_, WeaponMaterialRow>(r#"
                    SELECT
                        materials.id        AS material_id,
                        materials.quantity  AS material_quantity,
                        items.id            AS item_id,
                        items.code          AS item_code,
                        items.name          AS item_name,
                        items.price         AS item_price,
                        items.sell_price    AS item_sell_price
                    FROM
                        weapon_materials materials
                    INNER JOIN
                        items items
                        ON materials.item_id = items.id
                    WHERE
                        materials.weapon_id = $1
                    "#)
                    .bind(remodel_row.weapon_id)
                    .fetch_all(conn)
                    .await
            })
            .await?;

        Ok(Some(
            WeaponRemodel::of(
                remodel_row.remodel_id,
                Weapon::of(
                    remodel_row.weapon_id,
                    remodel_row.weapon_name,
                    remodel_row.weapon_str,
                    remodel_row.weapon_hit,
                ),
                remodel_row.remodel_price,
                material_rows.into_iter()
                    .map(|row| WeaponMaterial::of(
                        row.material_id,
                        Item::of(
                            row.item_id,
                            row.item_code,
                            row.item_name,
                            row.item_price,
                            row.item_sell_price,
                        ),
                        row.material_quantity,
                    ))
                    .collect(),
            )
        ))
    }

    async fn find_all(&self, pageable: &Pageable) -> anyhow::Result<Page<WeaponRemodel>> {
        let count = TransactionContext::execute_with(
            &self.pool, async |conn| {
                query_scalar(r#"
                    SELECT
                        COUNT(*)
                    FROM
                        weapon_remodels remodels
                    INNER JOIN
                        weapons weapons
                        ON remodels.weapon_id = weapons.id
                    "#)
                    .fetch_one(conn)
                    .await
            })
            .await?;

        let remodel_rows = TransactionContext::execute_with(
            &self.pool, async |conn| {
                query_as::<_, WeaponRemodelRow>(r#"
                    SELECT
                        remodels.id         AS remodel_id,
                        remodels.price      AS remodel_price,
                        weapons.id          AS weapon_id,
                        weapons.name        AS weapon_name,
                        weapons.str         AS weapon_str,
                        weapons.hit         AS weapon_hit
                    FROM
                        weapon_remodels remodels
                    INNER JOIN
                        weapons weapons
                        ON remodels.weapon_id = weapons.id
                    LIMIT $1
                    OFFSET $2
                    "#)
                    .bind(pageable.size)
                    .bind(pageable.offset())
                    .fetch_all(conn)
                    .await
            })
            .await?;

        let mut weapon_remodels = Vec::new();

        for row in remodel_rows {
            let materials = TransactionContext::execute_with(
                &self.pool, async |conn| {
                    query_as::<_, WeaponMaterialRow>(r#"
                        SELECT
                            materials.id       AS material_id,
                            materials.quantity AS material_quantity,
                            items.id           AS item_id,
                            items.code         AS item_code,
                            items.name         AS item_name,
                            items.price        AS item_price,
                            items.sell_price   AS item_sell_price
                        FROM
                            weapon_materials materials
                        INNER JOIN
                            items
                            ON materials.item_id = items.id
                        WHERE materials.weapon_id = $1
                        "#)
                        .bind(row.weapon_id)
                        .fetch_all(conn)
                        .await
                })
                .await?
                .into_iter()
                    .map(|row| WeaponMaterial::of(
                        row.material_id,
                        Item::of(
                            row.item_id,
                            row.item_code,
                            row.item_name,
                            row.item_price,
                            row.item_sell_price,
                        ),
                        row.material_quantity,
                    ))
                    .collect();

            weapon_remodels.push(
                WeaponRemodel::of(
                    row.remodel_id,
                    Weapon::of(
                        row.weapon_id,
                        row.weapon_name,
                        row.weapon_str,
                        row.weapon_hit,
                    ),
                    row.remodel_price,
                    materials,
                )
            );
        }

        Ok(Page::new(weapon_remodels, pageable, count))
    }

    async fn find_all_by_name_starting_with(&self, name: &String, pageable: &Pageable) -> anyhow::Result<Page<WeaponRemodel>> {
        let count = TransactionContext::execute_with(
            &self.pool, async |conn| {
                query_scalar(r#"
                    SELECT
                        COUNT(*)
                    FROM
                        weapon_remodels remodels
                    INNER JOIN
                        weapons weapons
                        ON remodels.weapon_id = weapons.id
                    WHERE
                        weapons.name LIKE $1 || '%'
                    "#)
                    .bind(name)
                    .fetch_one(conn)
                    .await
            })
            .await?;

        let remodel_rows = TransactionContext::execute_with(
            &self.pool, async |conn| {
                query_as::<_, WeaponRemodelRow>(r#"
                    SELECT
                        remodels.id         AS remodel_id,
                        remodels.price      AS remodel_price,
                        weapons.id          AS weapon_id,
                        weapons.name        AS weapon_name,
                        weapons.str         AS weapon_str,
                        weapons.hit         AS weapon_hit
                    FROM
                        weapon_remodels remodels
                    INNER JOIN
                        weapons weapons
                        ON remodels.weapon_id = weapons.id
                    WHERE
                        weapons.name LIKE $1 || '%'
                    LIMIT $2
                    OFFSET $3
                    "#)
                    .bind(name)
                    .bind(pageable.size)
                    .bind(pageable.offset())
                    .fetch_all(conn)
                    .await
            })
            .await?;

        let mut weapon_remodels = Vec::new();

        for row in remodel_rows {
            let materials = TransactionContext::execute_with(
                &self.pool, async |conn| {
                    query_as::<_, WeaponMaterialRow>(r#"
                        SELECT
                            materials.id       AS material_id,
                            materials.quantity AS material_quantity,
                            items.id           AS item_id,
                            items.code         AS item_code,
                            items.name         AS item_name,
                            items.price        AS item_price,
                            items.sell_price   AS item_sell_price
                        FROM
                            weapon_materials materials
                        INNER JOIN
                            items
                            ON materials.item_id = items.id
                        WHERE
                            materials.weapon_id = $1
                        "#)
                        .bind(row.weapon_id)
                        .fetch_all(conn)
                        .await
                })
                .await?
                .into_iter()
                    .map(|row| WeaponMaterial::of(
                        row.material_id,
                        Item::of(
                            row.item_id,
                            row.item_code,
                            row.item_name,
                            row.item_price,
                            row.item_sell_price,
                        ),
                        row.material_quantity,
                    ))
                    .collect();

            weapon_remodels.push(
                WeaponRemodel::of(
                    row.remodel_id,
                    Weapon::of(
                        row.weapon_id,
                        row.weapon_name,
                        row.weapon_str,
                        row.weapon_hit,
                    ),
                    row.remodel_price,
                    materials,
                )
            );
        }

        Ok(Page::new(weapon_remodels, pageable, count))
    }
}
