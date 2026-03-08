use crate::database::enums::PermissionEnum;
use crate::handlers::students::student;
use crate::utils::jwt::Authenticated;
use crate::utils::permission_verification::PermissionVerification;
use apistos::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/students")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::StudentManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(student::create_student))
            .route("/{id}", web::get().to(student::get_student_by_id))
            .route("", web::get().to(student::get_all_student))
            .route("/{id}", web::put().to(student::update_student))
            .route("/{id}", web::delete().to(student::delete_student))
            .route("/bulk", web::delete().to(student::bulk_delete_student))
            .route("/bulk", web::patch().to(student::bulk_update_student)),
    );
}
