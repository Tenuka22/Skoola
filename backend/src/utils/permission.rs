use std::collections::HashSet;
use std::future::{ready, Ready};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web, Error, HttpMessage,
};
use diesel::prelude::*;
use futures_util::future::LocalBoxFuture;

use crate::{
    AppState,
    database::tables::{Permission, Role, RolePermission, Session, StaffRole},
    errors::APIError,
    schema::{permissions, roles},
};
pub struct PermissionVerification {
    pub required_permission: String,
}

impl<S, B> Transform<S, ServiceRequest> for PermissionVerification
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = PermissionVerificationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(PermissionVerificationMiddleware {
            service,
            required_permission: self.required_permission.clone(),
        }))
    }
}

pub struct PermissionVerificationMiddleware<S> {
    service: S,
    required_permission: String,
}

fn get_all_permissions(
    conn: &mut SqliteConnection,
    role_id: &str,
    visited_roles: &mut HashSet<String>,
) -> Result<HashSet<i32>, diesel::result::Error> {
    if !visited_roles.insert(role_id.to_string()) {
        return Ok(HashSet::new());
    }

    let mut all_permissions = HashSet::new();

    let direct_permissions = RolePermission::find_by_role_id(conn, role_id)?;
    for p in direct_permissions {
        all_permissions.insert(p.permission_id);
    }

    let role: Role = roles::table.find(role_id).first(conn)?;
    if let Some(parent_id) = role.parent_id {
        let parent_permissions = get_all_permissions(conn, &parent_id, visited_roles)?;
        all_permissions.extend(parent_permissions);
    }

    Ok(all_permissions)
}



// ... (keep the rest of the file as is until the call function)

impl<S, B> Service<ServiceRequest> for PermissionVerificationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);
        let required_permission_cloned = self.required_permission.clone();

        Box::pin(async move {
            let res = fut.await?;

            let data = res.request()
                .app_data::<web::Data<AppState>>()
                .ok_or_else(|| APIError::internal("Application state not found"))
                ?;

            let session = {
                let extensions = res.request().extensions();
                extensions.get::<Session>().cloned()
            };

            if let Some(session) = session {
                let mut conn = data.db_pool.get().map_err(APIError::from)?;
                
                let required_permission_id: i32 = permissions::table
                    .filter(permissions::name.eq(&required_permission_cloned))
                    .select(permissions::id)
                    .first(&mut conn)
                    .map_err(|_| APIError::not_found("Permission not found"))?;

                let staff_roles = StaffRole::find_by_staff_id(&mut conn, &session.user_id)
                    .map_err(APIError::from)?;

                let mut user_permissions = HashSet::new();
                for role in staff_roles {
                    let mut visited_roles = HashSet::new();
                    let permissions = get_all_permissions(&mut conn, &role.role_id, &mut visited_roles)
                        .map_err(APIError::from)?;
                    user_permissions.extend(permissions);
                }

                if user_permissions.contains(&required_permission_id) {
                    return Ok(res);
                }
            }

            Err(APIError::forbidden("You don't have the required permission").into())
        })
    }
}
