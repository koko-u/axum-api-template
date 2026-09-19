use crate::features::health_check::*;

mod security_addon;

const MODIFIER: security_addon::SecurityAddon = security_addon::SecurityAddon;

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
    ok
    ),
    modifiers(
       &MODIFIER
    )
)]
pub struct ApiDoc;
