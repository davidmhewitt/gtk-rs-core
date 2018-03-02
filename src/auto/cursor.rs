// This file was generated by gir (https://github.com/gtk-rs/gir @ fbb95f4)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

use CursorType;
use Display;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use cairo;
use ffi;
use gdk_pixbuf;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Cursor(Object<ffi::GdkCursor>);

    match fn {
        get_type => || ffi::gdk_cursor_get_type(),
    }
}

impl Cursor {
    #[cfg_attr(feature = "v3_16", deprecated)]
    pub fn new(cursor_type: CursorType) -> Cursor {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gdk_cursor_new(cursor_type.to_glib()))
        }
    }

    pub fn new_for_display(display: &Display, cursor_type: CursorType) -> Cursor {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gdk_cursor_new_for_display(display.to_glib_none().0, cursor_type.to_glib()))
        }
    }

    pub fn new_from_name(display: &Display, name: &str) -> Cursor {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gdk_cursor_new_from_name(display.to_glib_none().0, name.to_glib_none().0))
        }
    }

    pub fn new_from_pixbuf(display: &Display, pixbuf: &gdk_pixbuf::Pixbuf, x: i32, y: i32) -> Cursor {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gdk_cursor_new_from_pixbuf(display.to_glib_none().0, pixbuf.to_glib_none().0, x, y))
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    pub fn new_from_surface(display: &Display, surface: &cairo::Surface, x: f64, y: f64) -> Cursor {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gdk_cursor_new_from_surface(display.to_glib_none().0, mut_override(surface.to_glib_none().0), x, y))
        }
    }
}

pub trait CursorExt {
    fn get_cursor_type(&self) -> CursorType;

    fn get_display(&self) -> Display;

    fn get_image(&self) -> Option<gdk_pixbuf::Pixbuf>;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_surface(&self) -> (Option<cairo::Surface>, f64, f64);

    fn connect_property_cursor_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_display_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Cursor> + IsA<glib::object::Object>> CursorExt for O {
    fn get_cursor_type(&self) -> CursorType {
        unsafe {
            from_glib(ffi::gdk_cursor_get_cursor_type(self.to_glib_none().0))
        }
    }

    fn get_display(&self) -> Display {
        unsafe {
            from_glib_none(ffi::gdk_cursor_get_display(self.to_glib_none().0))
        }
    }

    fn get_image(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            from_glib_full(ffi::gdk_cursor_get_image(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_surface(&self) -> (Option<cairo::Surface>, f64, f64) {
        unsafe {
            let mut x_hot = mem::uninitialized();
            let mut y_hot = mem::uninitialized();
            let ret = from_glib_full(ffi::gdk_cursor_get_surface(self.to_glib_none().0, &mut x_hot, &mut y_hot));
            (ret, x_hot, y_hot)
        }
    }

    fn connect_property_cursor_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::cursor-type",
                transmute(notify_cursor_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_display_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::display",
                transmute(notify_display_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_cursor_type_trampoline<P>(this: *mut ffi::GdkCursor, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Cursor> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Cursor::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_display_trampoline<P>(this: *mut ffi::GdkCursor, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Cursor> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Cursor::from_glib_borrow(this).downcast_unchecked())
}
