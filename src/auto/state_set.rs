// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use StateType;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct StateSet(Object<ffi::AtkStateSet, ffi::AtkStateSetClass>);

    match fn {
        get_type => || ffi::atk_state_set_get_type(),
    }
}

impl StateSet {
    pub fn new() -> StateSet {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::atk_state_set_new())
        }
    }
}

impl Default for StateSet {
    fn default() -> Self {
        Self::new()
    }
}

pub trait StateSetExt: 'static {
    fn add_state(&self, type_: StateType) -> bool;

    //fn add_states(&self, types: /*Unimplemented*/&CArray TypeId { ns_id: 1, id: 67 });

    fn and_sets(&self, compare_set: &StateSet) -> Option<StateSet>;

    fn clear_states(&self);

    fn contains_state(&self, type_: StateType) -> bool;

    //fn contains_states(&self, types: /*Unimplemented*/&CArray TypeId { ns_id: 1, id: 67 }) -> bool;

    fn is_empty(&self) -> bool;

    fn or_sets(&self, compare_set: &StateSet) -> Option<StateSet>;

    fn remove_state(&self, type_: StateType) -> bool;

    fn xor_sets(&self, compare_set: &StateSet) -> Option<StateSet>;
}

impl<O: IsA<StateSet>> StateSetExt for O {
    fn add_state(&self, type_: StateType) -> bool {
        unsafe {
            from_glib(ffi::atk_state_set_add_state(self.to_glib_none().0, type_.to_glib()))
        }
    }

    //fn add_states(&self, types: /*Unimplemented*/&CArray TypeId { ns_id: 1, id: 67 }) {
    //    unsafe { TODO: call ffi::atk_state_set_add_states() }
    //}

    fn and_sets(&self, compare_set: &StateSet) -> Option<StateSet> {
        unsafe {
            from_glib_full(ffi::atk_state_set_and_sets(self.to_glib_none().0, compare_set.to_glib_none().0))
        }
    }

    fn clear_states(&self) {
        unsafe {
            ffi::atk_state_set_clear_states(self.to_glib_none().0);
        }
    }

    fn contains_state(&self, type_: StateType) -> bool {
        unsafe {
            from_glib(ffi::atk_state_set_contains_state(self.to_glib_none().0, type_.to_glib()))
        }
    }

    //fn contains_states(&self, types: /*Unimplemented*/&CArray TypeId { ns_id: 1, id: 67 }) -> bool {
    //    unsafe { TODO: call ffi::atk_state_set_contains_states() }
    //}

    fn is_empty(&self) -> bool {
        unsafe {
            from_glib(ffi::atk_state_set_is_empty(self.to_glib_none().0))
        }
    }

    fn or_sets(&self, compare_set: &StateSet) -> Option<StateSet> {
        unsafe {
            from_glib_full(ffi::atk_state_set_or_sets(self.to_glib_none().0, compare_set.to_glib_none().0))
        }
    }

    fn remove_state(&self, type_: StateType) -> bool {
        unsafe {
            from_glib(ffi::atk_state_set_remove_state(self.to_glib_none().0, type_.to_glib()))
        }
    }

    fn xor_sets(&self, compare_set: &StateSet) -> Option<StateSet> {
        unsafe {
            from_glib_full(ffi::atk_state_set_xor_sets(self.to_glib_none().0, compare_set.to_glib_none().0))
        }
    }
}

impl fmt::Display for StateSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StateSet")
    }
}
