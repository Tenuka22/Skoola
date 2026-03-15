#[macro_export]
macro_rules! create_admin_handlers_i32 {
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
                        let body_inner: $create_req = body.into_inner();
                        let res: $resp = $service::$m(data, body_inner).await?;
                        Ok(actix_web::web::Json(res))
                    }
                };
                (get_by_id, $m:ident) => {
                    #[apistos::api_operation(summary = "Get " $entity_name " by ID", tag = $tag, operation_id = "get_" [<$entity_name:snake>] "_by_id")]
                    pub async fn [<get_ $entity_name:snake _by_id>](data: actix_web::web::Data<crate::AppState>, path: actix_web::web::Path<i32>) -> Result<actix_web::web::Json<$resp>, crate::errors::APIError> {
                        let id: i32 = path.into_inner();
                        let res: $resp = $service::$m(data, id).await?;
                        Ok(actix_web::web::Json(res))
                    }
                };
                (get_all, $m:ident) => {
                    #[apistos::api_operation(summary = "Get All " $entity_name, tag = $tag, operation_id = "get_all_" [<$entity_name:snake>])]
                    pub async fn [<get_all_ $entity_name:snake>](data: actix_web::web::Data<crate::AppState>, query: actix_web::web::Query<$query>) -> Result<actix_web::web::Json<crate::services::admin_db::PaginatedResponse<$resp>>, crate::errors::APIError> {
                        use crate::services::admin_db::AsAdminQuery;
                        let query_inner: $query = query.into_inner();
                        let (items, total, total_pages, next_last_id): (Vec<$resp>, i64, i64, Option<i32>) = $service::$m(data, query_inner.clone()).await?;
                        let admin_q = query_inner.as_admin_query();
                        Ok(actix_web::web::Json(crate::services::admin_db::PaginatedResponse {
                            data: items,
                            total,
                            page: admin_q.page.unwrap_or(1),
                            limit: admin_q.limit.unwrap_or(10),
                            total_pages,
                            next_last_id: next_last_id.map(|id| id.to_string()),
                        }))
                    }
                };
                (update, $m:ident) => {
                    #[apistos::api_operation(summary = "Update " $entity_name, tag = $tag, operation_id = "update_" [<$entity_name:snake>])]
                    pub async fn [<update_ $entity_name:snake>](data: actix_web::web::Data<crate::AppState>, path: actix_web::web::Path<i32>, body: actix_web::web::Json<$update_req>) -> Result<actix_web::web::Json<$resp>, crate::errors::APIError> {
                        let id: i32 = path.into_inner();
                        let body_inner: $update_req = body.into_inner();
                        let res: $resp = $service::$m(data, id, body_inner).await?;
                        Ok(actix_web::web::Json(res))
                    }
                };
                (delete, $m:ident) => {
                    #[apistos::api_operation(summary = "Delete " $entity_name, tag = $tag, operation_id = "delete_" [<$entity_name:snake>])]
                    pub async fn [<delete_ $entity_name:snake>](data: actix_web::web::Data<crate::AppState>, path: actix_web::web::Path<i32>) -> Result<actix_web::web::Json<crate::models::MessageResponse>, crate::errors::APIError> {
                        let id: i32 = path.into_inner();
                        $service::$m(data, id).await?;
                        Ok(actix_web::web::Json(crate::models::MessageResponse { message: format!("{} deleted successfully", stringify!($entity_name)) }))
                    }
                };
                (bulk_delete, $m:ident) => {
                    #[apistos::api_operation(summary = "Bulk Delete " $entity_name, tag = $tag, operation_id = "bulk_delete_" [<$entity_name:snake>])]
                    pub async fn [<bulk_delete_ $entity_name:snake>](data: actix_web::web::Data<crate::AppState>, body: actix_web::web::Json<Vec<i32>>) -> Result<actix_web::web::Json<crate::models::MessageResponse>, crate::errors::APIError> {
                        let ids: Vec<i32> = body.into_inner();
                        $service::$m(data, ids).await?;
                        Ok(actix_web::web::Json(crate::models::MessageResponse { message: format!("{} entities deleted successfully", stringify!($entity_name)) }))
                    }
                };
                (bulk_update, $m:ident) => {
                    #[apistos::api_operation(summary = "Bulk Update " $entity_name, tag = $tag, operation_id = "bulk_update_" [<$entity_name:snake>])]
                    pub async fn [<bulk_update_ $entity_name:snake>](data: actix_web::web::Data<crate::AppState>, body: actix_web::web::Json<crate::services::admin_db::BulkUpdateRequestI32<$update_req>>) -> Result<actix_web::web::Json<crate::models::MessageResponse>, crate::errors::APIError> {
                        let body_inner = body.into_inner();
                        $service::$m(data, body_inner).await?;
                        Ok(actix_web::web::Json(crate::models::MessageResponse { message: format!("{} entities updated successfully", stringify!($entity_name)) }))
                    }
                };
                (bulk_create, $m:ident) => {
                    #[apistos::api_operation(summary = "Bulk Create " $entity_name, tag = $tag, operation_id = "bulk_create_" [<$entity_name:snake>])]
                    pub async fn [<bulk_create_ $entity_name:snake>](data: actix_web::web::Data<crate::AppState>, body: actix_web::web::Json<crate::services::admin_db::BulkCreateRequest<$create_req>>) -> Result<actix_web::web::Json<crate::models::MessageResponse>, crate::errors::APIError> {
                        let body_inner = body.into_inner();
                        $service::$m(data, body_inner.items).await?;
                        Ok(actix_web::web::Json(crate::models::MessageResponse { message: format!("{} entities created successfully", stringify!($entity_name)) }))
                    }
                };
            }

            $( gen_handler!($action, $method); )*
        }
    };
}

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
        id_type => $id_type:ty,
        methods => {
            $( $action:ident => $method:ident ),*
        }
    ) => {
        ::paste::paste! {
            macro_rules! gen_handler {
                (create, $m:ident) => {
                    #[apistos::api_operation(summary = "Create " $entity_name, tag = $tag, operation_id = "create_" [<$entity_name:snake>])]
                    pub async fn [<create_ $entity_name:snake>](data: actix_web::web::Data<crate::AppState>, body: actix_web::web::Json<$create_req>) -> Result<actix_web::web::Json<$resp>, crate::errors::APIError> {
                        let body_inner: $create_req = body.into_inner();
                        let res: $resp = $service::$m(data, body_inner).await?;
                        Ok(actix_web::web::Json(res))
                    }
                };
                (get_by_id, $m:ident) => {
                    #[apistos::api_operation(summary = "Get " $entity_name " by ID", tag = $tag, operation_id = "get_" [<$entity_name:snake>] "_by_id")]
                    pub async fn [<get_ $entity_name:snake _by_id>](data: actix_web::web::Data<crate::AppState>, path: actix_web::web::Path<$id_type>) -> Result<actix_web::web::Json<$resp>, crate::errors::APIError> {
                        let id: $id_type = path.into_inner();
                        let res: $resp = $service::$m(data, id).await?;
                        Ok(actix_web::web::Json(res))
                    }
                };
                (get_all, $m:ident) => {
                    #[apistos::api_operation(summary = "Get All " $entity_name, tag = $tag, operation_id = "get_all_" [<$entity_name:snake>])]
                    pub async fn [<get_all_ $entity_name:snake>](data: actix_web::web::Data<crate::AppState>, query: actix_web::web::Query<$query>) -> Result<actix_web::web::Json<crate::services::admin_db::PaginatedResponse<$resp>>, crate::errors::APIError> {
                        use crate::services::admin_db::AsAdminQuery;
                        let query_inner: $query = query.into_inner();
                        let (items, total, total_pages, next_last_id): (Vec<$resp>, i64, i64, Option<$id_type>) = $service::$m(data, query_inner.clone()).await?;
                        let admin_q = query_inner.as_admin_query();

                        // Handle next_last_id conversion to Option<String>
                        let next_last_id_str = next_last_id.map(|id| format!("{:?}", id)); // Simplistic conversion, might need trait bound for better formatting

                        Ok(actix_web::web::Json(crate::services::admin_db::PaginatedResponse {
                            data: items,
                            total,
                            page: admin_q.page.unwrap_or(1),
                            limit: admin_q.limit.unwrap_or(10),
                            total_pages,
                            next_last_id: next_last_id_str,
                        }))
                    }
                };
                (update, $m:ident) => {
                    #[apistos::api_operation(summary = "Update " $entity_name, tag = $tag, operation_id = "update_" [<$entity_name:snake>])]
                    pub async fn [<update_ $entity_name:snake>](data: actix_web::web::Data<crate::AppState>, path: actix_web::web::Path<$id_type>, body: actix_web::web::Json<$update_req>) -> Result<actix_web::web::Json<$resp>, crate::errors::APIError> {
                        let id: $id_type = path.into_inner();
                        let body_inner: $update_req = body.into_inner();
                        let res: $resp = $service::$m(data, id, body_inner).await?;
                        Ok(actix_web::web::Json(res))
                    }
                };
                (delete, $m:ident) => {
                    #[apistos::api_operation(summary = "Delete " $entity_name, tag = $tag, operation_id = "delete_" [<$entity_name:snake>])]
                    pub async fn [<delete_ $entity_name:snake>](data: actix_web::web::Data<crate::AppState>, path: actix_web::web::Path<$id_type>) -> Result<actix_web::web::Json<crate::models::MessageResponse>, crate::errors::APIError> {
                        let id: $id_type = path.into_inner();
                        $service::$m(data, id).await?;
                        Ok(actix_web::web::Json(crate::models::MessageResponse { message: format!("{} deleted successfully", stringify!($entity_name)) }))
                    }
                };
                (bulk_delete, $m:ident) => {
                    #[apistos::api_operation(summary = "Bulk Delete " $entity_name, tag = $tag, operation_id = "bulk_delete_" [<$entity_name:snake>])]
                    pub async fn [<bulk_delete_ $entity_name:snake>](data: actix_web::web::Data<crate::AppState>, body: actix_web::web::Json<Vec<$id_type>>) -> Result<actix_web::web::Json<crate::models::MessageResponse>, crate::errors::APIError> {
                        let ids: Vec<$id_type> = body.into_inner();
                        $service::$m(data, ids).await?;
                        Ok(actix_web::web::Json(crate::models::MessageResponse { message: format!("{} entities deleted successfully", stringify!($entity_name)) }))
                    }
                };
                (bulk_update, $m:ident) => {
                    #[apistos::api_operation(summary = "Bulk Update " $entity_name, tag = $tag, operation_id = "bulk_update_" [<$entity_name:snake>])]
                    pub async fn [<bulk_update_ $entity_name:snake>](data: actix_web::web::Data<crate::AppState>, body: actix_web::web::Json<crate::services::admin_db::BulkUpdateRequest<$update_req>>) -> Result<actix_web::web::Json<crate::models::MessageResponse>, crate::errors::APIError> {
                        let body_inner = body.into_inner();
                        $service::$m(data, body_inner).await?;
                        Ok(actix_web::web::Json(crate::models::MessageResponse { message: format!("{} entities updated successfully", stringify!($entity_name)) }))
                    }
                };
                (bulk_create, $m:ident) => {
                    #[apistos::api_operation(summary = "Bulk Create " $entity_name, tag = $tag, operation_id = "bulk_create_" [<$entity_name:snake>])]
                    pub async fn [<bulk_create_ $entity_name:snake>](data: actix_web::web::Data<crate::AppState>, body: actix_web::web::Json<crate::services::admin_db::BulkCreateRequest<$create_req>>) -> Result<actix_web::web::Json<crate::models::MessageResponse>, crate::errors::APIError> {
                        let body_inner = body.into_inner();
                        $service::$m(data, body_inner.items).await?;
                        Ok(actix_web::web::Json(crate::models::MessageResponse { message: format!("{} entities created successfully", stringify!($entity_name)) }))
                    }
                };
            }

            $( gen_handler!($action, $method); )*
        }
    };
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
        $crate::create_admin_handlers!(
            tag => $tag,
            entity => $entity_name,
            response => $resp,
            query => $query,
            create => $create_req,
            update => $update_req,
            service => $service,
            id_type => String,
            methods => {
                $( $action => $method ),*
            }
        );
    };
}
