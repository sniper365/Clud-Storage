macro_rules! resolve {
    (FileService) => {
        crate::services::file::implementation::Service
    };

    (FolderService) => {
        crate::services::folder::implementation::Service
    };

    (UserService) => {
        crate::services::user::implementation::Service
    };

    (StorageService) => {
        crate::services::StorageService
    };

    (FileController) => {
        crate::controllers::FileController
    };

    (FolderController) => {
        crate::controllers::FolderController
    };

    (UserController) => {
        crate::controllers::UserController
    };
}
