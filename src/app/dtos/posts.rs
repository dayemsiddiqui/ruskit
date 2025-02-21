use crate::framework::prelude::*;
use ts_export_derive::auto_ts_export;

#[auto_ts_export]
pub struct PostsProps {
    pub title: String,
    // TODO: Add your page props here
}