use super::nif_interface;
use super::nif_interface::{ NIF_ENV, NIF_TERM };
use std::mem;

pub fn is_list(env: NIF_ENV, list: NIF_TERM) -> bool {
    (unsafe { nif_interface::enif_is_list(env, list) } == 1)
}
pub fn is_empty_list(env: NIF_ENV, list: NIF_TERM) -> bool {
    (unsafe { nif_interface::enif_is_empty_list(env, list) } == 1)
}

pub fn get_list_cell(env: NIF_ENV, list: NIF_TERM) -> Option<(NIF_TERM, NIF_TERM)> {
    let mut head: NIF_TERM = unsafe { mem::uninitialized() };
    let mut tail: NIF_TERM = unsafe { mem::uninitialized() };
    let success = unsafe { nif_interface::enif_get_list_cell(env, list, &mut head as *mut NIF_TERM, &mut tail as *mut NIF_TERM) };

    if success != 1 {
        return None;
    }
    Some((head, tail))
}
