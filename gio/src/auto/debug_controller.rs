// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Initable;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GDebugController")]
    pub struct DebugController(Interface<ffi::GDebugController, ffi::GDebugControllerInterface>) @requires Initable;

    match fn {
        type_ => || ffi::g_debug_controller_get_type(),
    }
}

impl DebugController {
    pub const NONE: Option<&'static DebugController> = None;
}

pub trait DebugControllerExt: 'static {
    #[doc(alias = "g_debug_controller_get_debug_enabled")]
    #[doc(alias = "get_debug_enabled")]
    fn is_debug_enabled(&self) -> bool;

    #[doc(alias = "g_debug_controller_set_debug_enabled")]
    fn set_debug_enabled(&self, debug_enabled: bool);

    #[cfg(any(feature = "v2_72", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_72")))]
    #[doc(alias = "debug-enabled")]
    fn connect_debug_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DebugController>> DebugControllerExt for O {
    fn is_debug_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::g_debug_controller_get_debug_enabled(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_debug_enabled(&self, debug_enabled: bool) {
        unsafe {
            ffi::g_debug_controller_set_debug_enabled(
                self.as_ref().to_glib_none().0,
                debug_enabled.into_glib(),
            );
        }
    }

    #[cfg(any(feature = "v2_72", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_72")))]
    fn connect_debug_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_debug_enabled_trampoline<
            P: IsA<DebugController>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GDebugController,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DebugController::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::debug-enabled\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_debug_enabled_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DebugController {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DebugController")
    }
}