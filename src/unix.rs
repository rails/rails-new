pub fn dockerfile_content() -> &'static [u8] {
    include_bytes!("../Dockerfile.unix")
}

pub fn get_user_id() -> Option<u32> {
    Some(users::get_current_uid())
}

pub fn get_group_id() -> Option<u32> {
    Some(users::get_current_gid())
}
