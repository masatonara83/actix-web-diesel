use crate::app::profile::model::Profile as ProfileModel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileResponse {
    pub profile: Profile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}

impl From<ProfileModel> for ProfileResponse {
    fn from(profile_model: ProfileModel) -> Self {
        Self {
            profile: Profile {
                username: profile_model.username,
                bio: profile_model.bio,
                image: profile_model.image,
                following: profile_model.following,
            },
        }
    }
}
