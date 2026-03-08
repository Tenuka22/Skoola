#[macro_export]
macro_rules! create_admin_handlers {
    (
        tag => $tag:expr,
        entity => $entity_name:ident,
        response => $resp:ty,
        query => $query:ty,
        create => $create_req:ty,
        update => $update_req:ty,
        service => $service:path,
        methods => {
            $( $action:ident => $method:ident ),*
        }
    ) => {
        ::paste::paste! {
            macro_rules! gen_handler {
                (create, $m:ident) => {
                    #[apistos::api_operation(summary = "Create " $entity_name, tag = $tag, operation_id = "create_" [<$entity_name:snake>])]
                    pub async fn [<create_ $entity_name:snake>](data: actix_web::web::Data<crate::AppState>, body: actix_web::web::Json<$create_req>) -> Result<actix_web::web::Json<$resp>, crate::errors::APIError> {
                        let res: $resp = $service::$m(data, body.into_inner()).await?;
                        Ok(actix_web::web::Json(res))
                    }
                };
                (get_by_id, $m:ident) => {
                    #[apistos::api_operation(summary = "Get " $entity_name " by ID", tag = $tag, operation_id = "get_" [<$entity_name:snake>] "_by_id")]
                    pub async fn [<get_ $entity_name:snake _by_id>](data: actix_web::web::Data<crate::AppState>, path: actix_web::web::Path<String>) -> Result<actix_web::web::Json<$resp>, crate::errors::APIError> {
                        let res: $resp = $service::$m(data, path.into_inner()).await?;
                        Ok(actix_web::web::Json(res))
                    }
                };
                (get_all, $m:ident) => {
                    #[apistos::api_operation(summary = "Get All " $entity_name, tag = $tag, operation_id = "get_all_" [<$entity_name:snake>])]
                    pub async fn [<get_all_ $entity_name:snake>](data: actix_web::web::Data<crate::AppState>, query: actix_web::web::Query<$query>) -> Result<actix_web::web::Json<crate::services::admin_db::PaginatedResponse<$resp>>, crate::errors::APIError> {
                        use crate::services::admin_db::AsAdminQuery;
                        let query_inner = query.into_inner();
                        let (items, total, total_pages, next_last_id): (Vec<$resp>, i64, i64, Option<String>) = $service::$m(data, query_inner.clone()).await?;
                        let admin_q = query_inner.as_admin_query();
                        Ok(actix_web::web::Json(crate::services::admin_db::PaginatedResponse {
                            data: items,
                            total,
                            page: admin_q.page.unwrap_or(1),
                            limit: admin_q.limit.unwrap_or(10),
                            total_pages,
                            next_last_id,
                        }))
                    }
                };
                (update, $m:ident) => {
                    #[apistos::api_operation(summary = "Update " $entity_name, tag = $tag, operation_id = "update_" [<$entity_name:snake>])]
                    pub async fn [<update_ $entity_name:snake>](data: actix_web::web::Data<crate::AppState>, path: actix_web::web::Path<String>, body: actix_web::web::Json<$update_req>) -> Result<actix_web::web::Json<$resp>, crate::errors::APIError> {
                        let res: $resp = $service::$m(data, path.into_inner(), body.into_inner()).await?;
                        Ok(actix_web::web::Json(res))
                    }
                };
                (delete, $m:ident) => {
                    #[apistos::api_operation(summary = "Delete " $entity_name, tag = $tag, operation_id = "delete_" [<$entity_name:snake>])]
                    pub async fn [<delete_ $entity_name:snake>](data: actix_web::web::Data<crate::AppState>, path: actix_web::web::Path<String>) -> Result<actix_web::web::Json<crate::models::MessageResponse>, crate::errors::APIError> {
                        $service::$m(data, path.into_inner()).await?;
                        Ok(actix_web::web::Json(crate::models::MessageResponse { message: format!("{} deleted successfully", stringify!($entity_name)) }))
                    }
                };
                (bulk_delete, $m:ident) => {
                    #[apistos::api_operation(summary = "Bulk Delete " $entity_name, tag = $tag, operation_id = "bulk_delete_" [<$entity_name:snake>])]
                    pub async fn [<bulk_delete_ $entity_name:snake>](data: actix_web::web::Data<crate::AppState>, body: actix_web::web::Json<crate::services::admin_db::BulkIdRequest>) -> Result<actix_web::web::Json<crate::models::MessageResponse>, crate::errors::APIError> {
                        $service::$m(data, body.into_inner().ids).await?;
                        Ok(actix_web::web::Json(crate::models::MessageResponse { message: format!("{} entities deleted successfully", stringify!($entity_name)) }))
                    }
                };
                (bulk_update, $m:ident) => {
                    #[apistos::api_operation(summary = "Bulk Update " $entity_name, tag = $tag, operation_id = "bulk_update_" [<$entity_name:snake>])]
                    pub async fn [<bulk_update_ $entity_name:snake>](data: actix_web::web::Data<crate::AppState>, body: actix_web::web::Json<crate::services::admin_db::BulkUpdateRequest<$update_req>>) -> Result<actix_web::web::Json<crate::models::MessageResponse>, crate::errors::APIError> {
                        $service::$m(data, body.into_inner()).await?;
                        Ok(actix_web::web::Json(crate::models::MessageResponse { message: format!("{} entities updated successfully", stringify!($entity_name)) }))
                    }
                };
                (bulk_create, $m:ident) => {
                    #[apistos::api_operation(summary = "Bulk Create " $entity_name, tag = $tag, operation_id = "bulk_create_" [<$entity_name:snake>])]
                    pub async fn [<bulk_create_ $entity_name:snake>](data: actix_web::web::Data<crate::AppState>, body: actix_web::web::Json<crate::services::admin_db::BulkCreateRequest<$create_req>>) -> Result<actix_web::web::Json<crate::models::MessageResponse>, crate::errors::APIError> {
                        $service::$m(data, body.into_inner().items).await?;
                        Ok(actix_web::web::Json(crate::models::MessageResponse { message: format!("{} entities created successfully", stringify!($entity_name)) }))
                    }
                };
            }

            $( gen_handler!($action, $method); )*
        }
    };
}
