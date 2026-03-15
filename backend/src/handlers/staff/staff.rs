use crate::models::staff::staff::{CreateStaffRequest, UpdateStaffRequest, StaffResponse, StaffQuery, StaffContactResponse, StaffContactQuery, CreateStaffContactRequest, UpdateStaffContactRequest, StaffMediaResponse, CreateStaffMediaRequest, UpdateStaffMediaRequest, StaffRewardSnapshotResponse, CreateStaffRewardSnapshotRequest, UpdateStaffRewardSnapshotRequest};
use crate::services::staff::staff::{StaffService, StaffContactService, StaffMediaService, StaffRewardSnapshotService};
use crate::create_admin_handlers;
use crate::services::admin_db::AdminQuery;

create_admin_handlers!(
    tag => "staff",
    entity => Staff,
    response => StaffResponse,
    query => StaffQuery,
    create => CreateStaffRequest,
    update => UpdateStaffRequest,
    service => StaffService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => update_with_logic,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => bulk_update_staff
    }
);

create_admin_handlers!(
    tag => "staff_contacts",
    entity => StaffContact,
    response => StaffContactResponse,
    query => StaffContactQuery,
    create => CreateStaffContactRequest,
    update => UpdateStaffContactRequest,
    service => StaffContactService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "staff_media",
    entity => StaffMedia,
    response => StaffMediaResponse,
    query => AdminQuery,
    create => CreateStaffMediaRequest,
    update => UpdateStaffMediaRequest,
    service => StaffMediaService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

create_admin_handlers!(
    tag => "staff_reward_snapshots",
    entity => StaffRewardSnapshot,
    response => StaffRewardSnapshotResponse,
    query => AdminQuery,
    create => CreateStaffRewardSnapshotRequest,
    update => UpdateStaffRewardSnapshotRequest,
    service => StaffRewardSnapshotService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);

