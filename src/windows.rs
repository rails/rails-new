pub fn dockerfile_content() -> &'static [u8] {
  include_bytes!("../Dockerfile.windows")
}

pub fn get_user_id() -> Option<u32> {
  None
}

pub fn get_group_id() -> Option<u32> {
  None
}
