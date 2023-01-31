use crate::models::{NewProduct, Product};
use crate::schema::products;
use diesel::query_dsl::methods::OrderDsl;
use diesel::query_dsl::select_dsl::SelectDsl;
use diesel::{
    query_dsl::methods::{FindDsl, LimitDsl},
    ExpressionMethods, QueryResult, RunQueryDsl,
};

pub struct ProductRepository;

impl ProductRepository {
    pub fn get_products(conn: &mut diesel::SqliteConnection) -> QueryResult<Vec<Product>> {
        products::table.limit(100).load::<Product>(conn)
    }

    pub fn get_product(id: i32, conn: &mut diesel::SqliteConnection) -> QueryResult<Product> {
        // products::table.find(id).first::<Product>(conn)
        products::table.find(id).get_result::<Product>(conn)
    }

    pub fn create_product(
        new_product: NewProduct,
        conn: &mut diesel::SqliteConnection,
    ) -> QueryResult<Product> {
        diesel::insert_into(products::table)
            .values(new_product)
            .execute(conn)?;
        let last_id = Self::last_id(conn)?;
        Self::get_product(last_id, conn)
    }

    fn last_id(conn: &mut diesel::SqliteConnection) -> QueryResult<i32> {
        products::table
            .select(products::id)
            .order(products::id.desc())
            .first(conn)
    }

    pub fn update_product(
        product: Product,
        conn: &mut diesel::SqliteConnection,
    ) -> QueryResult<Product> {
        let id = product.id;
        diesel::update(products::table.find(id))
            .set((
                products::name.eq(&product.name),
                products::description.eq(&product.description),
            ))
            .execute(conn)?;
        Self::get_product(id, conn)
    }

    pub fn delete_product(id: i32, conn: &mut diesel::SqliteConnection) -> QueryResult<usize> {
        diesel::delete(products::table.find(id)).execute(conn)
    }
}
