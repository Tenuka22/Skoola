/// Comprehensive admin handler macros with support for:
/// - Multiple ID types (String, i32, NaiveDate, UUID, etc.)
/// - All CRUD operations (create, read, update, delete)
/// - Bulk operations (bulk_create, bulk_update, bulk_delete)
/// - Advanced querying with filters, sorting, pagination
/// - API documentation via apistos

/// Internal helper macro - handles different action types with proper parameters
/// Must be defined BEFORE create_admin_handlers!
#[macro_export]
macro_rules! __admin_handler_fn {
    (create, $entity_name:ident, $tag:expr, $resp:ty, $_query:ty, $_update_req:ty, $create_req:ty, $service:path, $method:ident, $_id_type:ty, $summary:expr) => {
        $crate::__admin_handler_fn!(@create $entity_name, $tag, $resp, $create_req, $service, $method, $summary);
    };
    (get_by_id, $entity_name:ident, $tag:expr, $resp:ty, $_query:ty, $_update_req:ty, $_create_req:ty, $service:path, $method:ident, $id_type:ty, $summary:expr) => {
        $crate::__admin_handler_fn!(@get_by_id $entity_name, $tag, $resp, $service, $method, $id_type, $summary);
    };
    (get_all, $entity_name:ident, $tag:expr, $resp:ty, $query:ty, $_update_req:ty, $_create_req:ty, $service:path, $method:ident, $id_type:ty, $summary:expr) => {
        $crate::__admin_handler_fn!(@get_all $entity_name, $tag, $resp, $query, $service, $method, $id_type, $summary);
    };
    (update, $entity_name:ident, $tag:expr, $resp:ty, $_query:ty, $update_req:ty, $_create_req:ty, $service:path, $method:ident, $id_type:ty, $summary:expr) => {
        $crate::__admin_handler_fn!(@update $entity_name, $tag, $resp, $update_req, $service, $method, $id_type, $summary);
    };
    (delete, $entity_name:ident, $tag:expr, $resp:ty, $_query:ty, $_update_req:ty, $_create_req:ty, $service:path, $method:ident, $id_type:ty, $summary:expr) => {
        $crate::__admin_handler_fn!(@delete $entity_name, $tag, $service, $method, $id_type, $summary);
    };
    (bulk_delete, $entity_name:ident, $tag:expr, $resp:ty, $_query:ty, $_update_req:ty, $_create_req:ty, $service:path, $method:ident, $id_type:ty, $summary:expr) => {
        $crate::__admin_handler_fn!(@bulk_delete $entity_name, $tag, $service, $method, $id_type, $summary);
    };
    (bulk_update, $entity_name:ident, $tag:expr, $resp:ty, $_query:ty, $update_req:ty, $_create_req:ty, $service:path, $method:ident, $id_type:ty, $summary:expr) => {
        $crate::__admin_handler_fn!(@bulk_update $entity_name, $tag, $resp, $update_req, $service, $method, $id_type, $summary);
    };
    (bulk_create, $entity_name:ident, $tag:expr, $resp:ty, $_query:ty, $_update_req:ty, $create_req:ty, $service:path, $method:ident, $_id_type:ty, $summary:expr) => {
        $crate::__admin_handler_fn!(@bulk_create $entity_name, $tag, $create_req, $service, $method, $summary);
    };
    
    (@create $entity_name:ident, $tag:expr, $resp:ty, $create_req:ty, $service:path, $method:ident, $summary:expr) => {
        ::paste::paste! {
            #[doc = $summary]
            #[apistos::api_operation(tag = $tag)]
            pub async fn [<create_ $entity_name:snake>](
                data: actix_web::web::Data<crate::AppState>,
                body: actix_web::web::Json<$create_req>
            ) -> Result<actix_web::web::Json<$resp>, crate::errors::APIError> {
                let body_inner: $create_req = body.into_inner();
                let res: $resp = $service::$method(data, body_inner).await?;
                Ok(actix_web::web::Json(res))
            }
        }
    };
    (@get_by_id $entity_name:ident, $tag:expr, $resp:ty, $service:path, $method:ident, $id_type:ty, $summary:expr) => {
        ::paste::paste! {
            #[doc = $summary]
            #[apistos::api_operation(tag = $tag)]
            pub async fn [<get_ $entity_name:snake _by_id>](
                data: actix_web::web::Data<crate::AppState>,
                path: actix_web::web::Path<$id_type>
            ) -> Result<actix_web::web::Json<$resp>, crate::errors::APIError> {
                let id: $id_type = path.into_inner();
                let res: $resp = $service::$method(data, id).await?;
                Ok(actix_web::web::Json(res))
            }
        }
    };
    (@get_all $entity_name:ident, $tag:expr, $resp:ty, $query:ty, $service:path, $method:ident, $id_type:ty, $summary:expr) => {
        ::paste::paste! {
            #[doc = $summary]
            #[apistos::api_operation(tag = $tag)]
            pub async fn [<get_all_ $entity_name:snake>](
                data: actix_web::web::Data<crate::AppState>,
                query: actix_web::web::Query<$query>
            ) -> Result<actix_web::web::Json<crate::services::admin_db::PaginatedResponse<$resp>>, crate::errors::APIError> {
                use crate::services::admin_db::AsAdminQuery;
                let query_inner: $query = query.into_inner();
                let (items, total, total_pages, next_last_id): (Vec<$resp>, i64, i64, Option<$id_type>) = $service::$method(data, query_inner.clone()).await?;
                let admin_q = query_inner.as_admin_query();
                let next_last_id_str = next_last_id.as_ref().map(|id| format!("{:?}", id));
                Ok(actix_web::web::Json(crate::services::admin_db::PaginatedResponse {
                    data: items,
                    total,
                    page: admin_q.page.unwrap_or(1),
                    limit: admin_q.limit.unwrap_or(10),
                    total_pages,
                    next_last_id: next_last_id_str,
                    has_next: next_last_id.is_some(),
                    has_prev: admin_q.page.unwrap_or(1) > 1,
                }))
            }
        }
    };
    (@update $entity_name:ident, $tag:expr, $resp:ty, $update_req:ty, $service:path, $method:ident, $id_type:ty, $summary:expr) => {
        ::paste::paste! {
            #[doc = $summary]
            #[apistos::api_operation(tag = $tag)]
            pub async fn [<update_ $entity_name:snake>](
                data: actix_web::web::Data<crate::AppState>,
                path: actix_web::web::Path<$id_type>,
                body: actix_web::web::Json<$update_req>
            ) -> Result<actix_web::web::Json<$resp>, crate::errors::APIError> {
                let id: $id_type = path.into_inner();
                let body_inner: $update_req = body.into_inner();
                let res: $resp = $service::$method(data, id, body_inner).await?;
                Ok(actix_web::web::Json(res))
            }
        }
    };
    (@delete $entity_name:ident, $tag:expr, $service:path, $method:ident, $id_type:ty, $summary:expr) => {
        ::paste::paste! {
            #[doc = $summary]
            #[apistos::api_operation(tag = $tag)]
            pub async fn [<delete_ $entity_name:snake>](
                data: actix_web::web::Data<crate::AppState>,
                path: actix_web::web::Path<$id_type>
            ) -> Result<actix_web::web::Json<crate::models::MessageResponse>, crate::errors::APIError> {
                let id: $id_type = path.into_inner();
                $service::$method(data, id).await?;
                Ok(actix_web::web::Json(crate::models::MessageResponse {
                    message: format!("{} deleted successfully", stringify!($entity_name))
                }))
            }
        }
    };
    (@bulk_delete $entity_name:ident, $tag:expr, $service:path, $method:ident, $id_type:ty, $summary:expr) => {
        ::paste::paste! {
            #[doc = $summary]
            #[apistos::api_operation(tag = $tag)]
            pub async fn [<bulk_delete_ $entity_name:snake>](
                data: actix_web::web::Data<crate::AppState>,
                body: actix_web::web::Json<Vec<$id_type>>
            ) -> Result<actix_web::web::Json<crate::models::MessageResponse>, crate::errors::APIError> {
                let ids: Vec<$id_type> = body.into_inner();
                $service::$method(data, ids).await?;
                Ok(actix_web::web::Json(crate::models::MessageResponse {
                    message: format!("{} entities deleted successfully", stringify!($entity_name))
                }))
            }
        }
    };
    (@bulk_update $entity_name:ident, $tag:expr, $resp:ty, $update_req:ty, $service:path, $method:ident, $id_type:ty, $summary:expr) => {
        ::paste::paste! {
            #[doc = $summary]
            #[apistos::api_operation(tag = $tag)]
            pub async fn [<bulk_update_ $entity_name:snake>](
                data: actix_web::web::Data<crate::AppState>,
                body: actix_web::web::Json<crate::services::admin_db::BulkUpdateRequest<$update_req, $id_type>>
            ) -> Result<actix_web::web::Json<crate::models::MessageResponse>, crate::errors::APIError> {
                let body_inner = body.into_inner();
                $service::$method(data, body_inner).await?;
                Ok(actix_web::web::Json(crate::models::MessageResponse {
                    message: format!("{} entities updated successfully", stringify!($entity_name))
                }))
            }
        }
    };
    (@bulk_create $entity_name:ident, $tag:expr, $create_req:ty, $service:path, $method:ident, $summary:expr) => {
        ::paste::paste! {
            #[doc = $summary]
            #[apistos::api_operation(tag = $tag)]
            pub async fn [<bulk_create_ $entity_name:snake>](
                data: actix_web::web::Data<crate::AppState>,
                body: actix_web::web::Json<crate::services::admin_db::BulkCreateRequest<$create_req>>
            ) -> Result<actix_web::web::Json<crate::models::MessageResponse>, crate::errors::APIError> {
                let body_inner = body.into_inner();
                $service::$method(data, body_inner.items).await?;
                Ok(actix_web::web::Json(crate::models::MessageResponse {
                    message: format!("{} entities created successfully", stringify!($entity_name))
                }))
            }
        }
    };
}

/// Internal helper for create_admin_handlers to avoid match/concat in attributes
#[macro_export]
macro_rules! __dispatch_admin_handler {
    (create, $entity_name:ident, $tag:expr, $resp:ty, $query:ty, $update_req:ty, $create_req:ty, $service:path, $method:ident, $id_type:ty) => {
        $crate::__admin_handler_fn!(create, $entity_name, $tag, $resp, $query, $update_req, $create_req, $service, $method, $id_type, concat!("Create ", stringify!($entity_name)));
    };
    (get_by_id, $entity_name:ident, $tag:expr, $resp:ty, $query:ty, $update_req:ty, $create_req:ty, $service:path, $method:ident, $id_type:ty) => {
        $crate::__admin_handler_fn!(get_by_id, $entity_name, $tag, $resp, $query, $update_req, $create_req, $service, $method, $id_type, concat!("Get ", stringify!($entity_name), " by ID"));
    };
    (get_all, $entity_name:ident, $tag:expr, $resp:ty, $query:ty, $update_req:ty, $create_req:ty, $service:path, $method:ident, $id_type:ty) => {
        $crate::__admin_handler_fn!(get_all, $entity_name, $tag, $resp, $query, $update_req, $create_req, $service, $method, $id_type, concat!("Get All ", stringify!($entity_name)));
    };
    (update, $entity_name:ident, $tag:expr, $resp:ty, $query:ty, $update_req:ty, $create_req:ty, $service:path, $method:ident, $id_type:ty) => {
        $crate::__admin_handler_fn!(update, $entity_name, $tag, $resp, $query, $update_req, $create_req, $service, $method, $id_type, concat!("Update ", stringify!($entity_name)));
    };
    (delete, $entity_name:ident, $tag:expr, $resp:ty, $query:ty, $update_req:ty, $create_req:ty, $service:path, $method:ident, $id_type:ty) => {
        $crate::__admin_handler_fn!(delete, $entity_name, $tag, $resp, $query, $update_req, $create_req, $service, $method, $id_type, concat!("Delete ", stringify!($entity_name)));
    };
    (bulk_delete, $entity_name:ident, $tag:expr, $resp:ty, $query:ty, $update_req:ty, $create_req:ty, $service:path, $method:ident, $id_type:ty) => {
        $crate::__admin_handler_fn!(bulk_delete, $entity_name, $tag, $resp, $query, $update_req, $create_req, $service, $method, $id_type, concat!("Bulk Delete ", stringify!($entity_name)));
    };
    (bulk_update, $entity_name:ident, $tag:expr, $resp:ty, $query:ty, $update_req:ty, $create_req:ty, $service:path, $method:ident, $id_type:ty) => {
        $crate::__admin_handler_fn!(bulk_update, $entity_name, $tag, $resp, $query, $update_req, $create_req, $service, $method, $id_type, concat!("Bulk Update ", stringify!($entity_name)));
    };
    (bulk_create, $entity_name:ident, $tag:expr, $resp:ty, $query:ty, $update_req:ty, $create_req:ty, $service:path, $method:ident, $id_type:ty) => {
        $crate::__admin_handler_fn!(bulk_create, $entity_name, $tag, $resp, $query, $update_req, $create_req, $service, $method, $id_type, concat!("Bulk Create ", stringify!($entity_name)));
    };
}

/// Main macro for String ID type - generates all handler variants
#[macro_export]
macro_rules! create_admin_handlers {
    // Version with explicit id_type and explicit methods
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
            $( $action:ident => $method:ident ),* $(,)?
        }
    ) => {
        $(
            $crate::__dispatch_admin_handler!($action, $entity_name, $tag, $resp, $query, $update_req, $create_req, $service, $method, $id_type);
        )*
    };
    
    // Version with explicit id_type and DEFAULT methods
    (
        tag => $tag:expr,
        entity => $entity_name:ident,
        response => $resp:ty,
        query => $query:ty,
        create => $create_req:ty,
        update => $update_req:ty,
        service => $service:path,
        id_type => $id_type:ty
    ) => {
        $crate::create_admin_handlers!(
            tag => $tag,
            entity => $entity_name,
            response => $resp,
            query => $query,
            create => $create_req,
            update => $update_req,
            service => $service,
            id_type => $id_type,
            methods => {
                create => generic_create,
                get_by_id => generic_get_by_id,
                get_all => generic_get_all,
                update => generic_update,
                delete => generic_delete,
                bulk_delete => generic_bulk_delete,
                bulk_update => generic_bulk_update,
                bulk_create => generic_bulk_create,
            }
        );
    };

    // Default to String ID type with explicit methods
    (
        tag => $tag:expr,
        entity => $entity_name:ident,
        response => $resp:ty,
        query => $query:ty,
        create => $create_req:ty,
        update => $update_req:ty,
        service => $service:path,
        methods => {
            $( $action:ident => $method:ident ),* $(,)?
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

    // Default to String ID type and DEFAULT methods
    (
        tag => $tag:expr,
        entity => $entity_name:ident,
        response => $resp:ty,
        query => $query:ty,
        create => $create_req:ty,
        update => $update_req:ty,
        service => $service:path
    ) => {
        $crate::create_admin_handlers!(
            tag => $tag,
            entity => $entity_name,
            response => $resp,
            query => $query,
            create => $create_req,
            update => $update_req,
            service => $service,
            id_type => String
        );
    };
}

/// Macro for i32 ID type
#[macro_export]
macro_rules! create_admin_handlers_i32 {
    // Version with explicit methods
    (
        tag => $tag:expr,
        entity => $entity_name:ident,
        response => $resp:ty,
        query => $query:ty,
        create => $create_req:ty,
        update => $update_req:ty,
        service => $service:path,
        methods => {
            $( $action:ident => $method:ident ),* $(,)?
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
            id_type => i32,
            methods => {
                $( $action => $method ),*
            }
        );
    };

    // Version with DEFAULT methods
    (
        tag => $tag:expr,
        entity => $entity_name:ident,
        response => $resp:ty,
        query => $query:ty,
        create => $create_req:ty,
        update => $update_req:ty,
        service => $service:path
    ) => {
        $crate::create_admin_handlers!(
            tag => $tag,
            entity => $entity_name,
            response => $resp,
            query => $query,
            create => $create_req,
            update => $update_req,
            service => $service,
            id_type => i32
        );
    };
}
