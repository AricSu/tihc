use rust_embed::Embed;

#[derive(Embed)]
#[folder = "../frontend/dist/"]
#[prefix = "assets/"]
pub struct Assets;
