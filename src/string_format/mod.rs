use std::ffi::{c_char, CStr, CString};

const MAP_FILE_NAME_FORMAT: &str = "{}/mmaps/{:03}.mmap";
const MAP_FILE_NAME_FORMAT_TERRAIN_BUILDER: &str  = "{}/{:03}{:02}{:02}.map";
const TILE_FILE_NAME_FORMAT: &str = "{}/mmaps/{:03}{:02}{:02}.mmtile";

#[unsafe(no_mangle)]
pub unsafe extern "C" fn violet_format_mmap_path(
    base_path_ptr: *const c_char,
    map_id: u32
) -> *mut c_char {
    if base_path_ptr.is_null() {
        return std::ptr::null_mut();
    }
    if base_path_ptr.is_null() { return std::ptr::null_mut(); }

    let base = CStr::from_ptr(base_path_ptr).to_string_lossy();

    let result = MAP_FILE_NAME_FORMAT
        .replacen("{}", &base, 1)
        .replacen("{:03}", &format!("{:03}", map_id), 1);

    CString::new(result).expect("Failed to create CString").into_raw()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn violet_format_mmap_path_for_terrain(
    base_path_ptr: *const c_char,
    map_id: u32,
    tile_y: u32,
    tile_x: u32
) -> *mut c_char {
    if base_path_ptr.is_null() {
        return std::ptr::null_mut();
    }
    if base_path_ptr.is_null() { return std::ptr::null_mut(); }

    let base = CStr::from_ptr(base_path_ptr).to_string_lossy();

    let result = MAP_FILE_NAME_FORMAT_TERRAIN_BUILDER
        .replacen("{}", &base, 1)
        .replacen("{:03}", &format!("{:03}", map_id), 1)
        .replacen("{:02}", &format!("{:02}", tile_y), 1)
        .replacen("{:02}", &format!("{:02}", tile_x), 1);

    CString::new(result).expect("Failed to create CString").into_raw()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn violet_format_mmap_path_for_tile(
    base_path_ptr: *const c_char,
    map_id: u32,
    tile_y: u32,
    tile_x: u32
) -> *mut c_char {
    if base_path_ptr.is_null() {
        return std::ptr::null_mut();
    }
    if base_path_ptr.is_null() { return std::ptr::null_mut(); }

    let base = CStr::from_ptr(base_path_ptr).to_string_lossy();
    // "{}/mmaps/{:03}{:02}{:02}.mmtile"
    let result = TILE_FILE_NAME_FORMAT
        .replacen("{}", &base, 1)
        .replacen("{:03}", &format!("{:03}", map_id), 1)
        .replacen("{:02}", &format!("{:02}", tile_y), 1)
        .replacen("{:02}", &format!("{:02}", tile_x), 1);

    CString::new(result).expect("Failed to create CString").into_raw()
}