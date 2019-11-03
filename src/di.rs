macro_rules! resolve {
    (FileService) => {
        crate::services::FileService
    };

    (FolderService) => {
        crate::services::FolderService
    };

    (UserService) => {
        crate::services::UserService
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
