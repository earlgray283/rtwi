use super::twitter_api::user::UserInfo;
pub fn show_profile(user_info: &UserInfo) {
    println!("name: \n  {} (@{})", user_info.name, user_info.screen_name);
    println!();
    println!("bio: \n   {}", user_info.bio);
}
