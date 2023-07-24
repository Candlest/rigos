/* This file is about tackling config.toml
 * 管理网页相关参数
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct config{
    theme: String
}