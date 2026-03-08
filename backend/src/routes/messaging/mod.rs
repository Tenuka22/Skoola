use apistos::web;

pub fn configure(_cfg: &mut web::ServiceConfig) {
    // Messaging routes are not generic CRUD and are removed from the admin scope.
}
