use super::Restrict;
use db::models::{File, User};

impl Restrict<User> for File {
    fn viewable_by(&self, user: &User) -> bool {
        let folder = match self.folder() {
            Ok(folder) => folder,
            Err(_) => return false,
        };

        folder.user_id() == user.id()
    }

    fn modifiable_by(&self, user: &User) -> bool {
        let folder = match self.folder() {
            Ok(folder) => folder,
            Err(_) => return false,
        };

        folder.user_id() == user.id()
    }

    fn deletable_by(&self, user: &User) -> bool {
        let folder = match self.folder() {
            Ok(folder) => folder,
            Err(_) => return false,
        };

        folder.user_id() == user.id()
    }
}
