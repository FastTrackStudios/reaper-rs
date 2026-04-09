use reaper_low::raw;
use std::os::raw::c_char;

/// Owned project import registration with stable memory for callbacks.
///
/// This is used to register a custom file format importer with REAPER via the `"projectimport"`
/// registration key.
//
// Case 2: Internals exposed: yes | vtable: no
// ===========================================
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct OwnedProjectImportRegister {
    inner: raw::project_import_register_t,
}

impl OwnedProjectImportRegister {
    /// Creates a new project import registration.
    ///
    /// # Arguments
    ///
    /// * `want_project_file` - Called by REAPER to check if this importer wants to handle the given
    ///   file.
    /// * `enum_file_extensions` - Called by REAPER to enumerate supported file extensions. Return
    ///   `NULL` when `i` is past the last extension. Write a description to `descptr`.
    /// * `load_project` - Called by REAPER to load/import the project file using the given
    ///   `ProjectStateContext`.
    pub fn new(
        want_project_file: unsafe extern "C" fn(*const c_char) -> bool,
        enum_file_extensions: unsafe extern "C" fn(
            ::std::os::raw::c_int,
            *mut *mut c_char,
        ) -> *const c_char,
        load_project: unsafe extern "C" fn(
            *const c_char,
            *mut raw::ProjectStateContext,
        ) -> ::std::os::raw::c_int,
    ) -> Self {
        Self {
            inner: raw::project_import_register_t {
                WantProjectFile: Some(want_project_file),
                EnumFileExtensions: Some(enum_file_extensions),
                LoadProject: Some(load_project),
            },
        }
    }
}

impl AsRef<raw::project_import_register_t> for OwnedProjectImportRegister {
    fn as_ref(&self) -> &raw::project_import_register_t {
        &self.inner
    }
}
