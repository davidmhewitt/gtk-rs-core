// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(unix, feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(unix)))]
use crate::UnixFDList;
use crate::{
    AsyncInitable, AsyncResult, BusType, Cancellable, DBusCallFlags, DBusConnection, DBusInterface,
    DBusInterfaceInfo, DBusProxyFlags, Initable,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute, pin::Pin, ptr};

glib::wrapper! {
    #[doc(alias = "GDBusProxy")]
    pub struct DBusProxy(Object<ffi::GDBusProxy, ffi::GDBusProxyClass>) @implements AsyncInitable, DBusInterface, Initable;

    match fn {
        type_ => || ffi::g_dbus_proxy_get_type(),
    }
}

impl DBusProxy {
    pub const NONE: Option<&'static DBusProxy> = None;

    #[doc(alias = "g_dbus_proxy_new_for_bus_sync")]
    #[doc(alias = "new_for_bus_sync")]
    pub fn for_bus_sync(
        bus_type: BusType,
        flags: DBusProxyFlags,
        info: Option<&DBusInterfaceInfo>,
        name: &str,
        object_path: &str,
        interface_name: &str,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<DBusProxy, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_dbus_proxy_new_for_bus_sync(
                bus_type.into_glib(),
                flags.into_glib(),
                info.to_glib_none().0,
                name.to_glib_none().0,
                object_path.to_glib_none().0,
                interface_name.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_dbus_proxy_new_sync")]
    pub fn new_sync(
        connection: &DBusConnection,
        flags: DBusProxyFlags,
        info: Option<&DBusInterfaceInfo>,
        name: Option<&str>,
        object_path: &str,
        interface_name: &str,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<DBusProxy, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_dbus_proxy_new_sync(
                connection.to_glib_none().0,
                flags.into_glib(),
                info.to_glib_none().0,
                name.to_glib_none().0,
                object_path.to_glib_none().0,
                interface_name.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_dbus_proxy_new")]
    pub fn new<P: FnOnce(Result<DBusProxy, glib::Error>) + 'static>(
        connection: &DBusConnection,
        flags: DBusProxyFlags,
        info: Option<&DBusInterfaceInfo>,
        name: Option<&str>,
        object_path: &str,
        interface_name: &str,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn new_trampoline<P: FnOnce(Result<DBusProxy, glib::Error>) + 'static>(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::g_dbus_proxy_new_finish(res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = new_trampoline::<P>;
        unsafe {
            ffi::g_dbus_proxy_new(
                connection.to_glib_none().0,
                flags.into_glib(),
                info.to_glib_none().0,
                name.to_glib_none().0,
                object_path.to_glib_none().0,
                interface_name.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn new_future(
        connection: &DBusConnection,
        flags: DBusProxyFlags,
        info: Option<&DBusInterfaceInfo>,
        name: Option<&str>,
        object_path: &str,
        interface_name: &str,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<DBusProxy, glib::Error>> + 'static>> {
        let connection = connection.clone();
        let info = info.map(ToOwned::to_owned);
        let name = name.map(ToOwned::to_owned);
        let object_path = String::from(object_path);
        let interface_name = String::from(interface_name);
        Box_::pin(crate::GioFuture::new(
            &(),
            move |_obj, cancellable, send| {
                Self::new(
                    &connection,
                    flags,
                    info.as_ref().map(::std::borrow::Borrow::borrow),
                    name.as_ref().map(::std::borrow::Borrow::borrow),
                    &object_path,
                    &interface_name,
                    Some(cancellable),
                    move |res| {
                        send.resolve(res);
                    },
                );
            },
        ))
    }

    #[doc(alias = "g_dbus_proxy_new_for_bus")]
    #[doc(alias = "new_for_bus")]
    pub fn for_bus<P: FnOnce(Result<DBusProxy, glib::Error>) + 'static>(
        bus_type: BusType,
        flags: DBusProxyFlags,
        info: Option<&DBusInterfaceInfo>,
        name: &str,
        object_path: &str,
        interface_name: &str,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn for_bus_trampoline<
            P: FnOnce(Result<DBusProxy, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::g_dbus_proxy_new_for_bus_finish(res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = for_bus_trampoline::<P>;
        unsafe {
            ffi::g_dbus_proxy_new_for_bus(
                bus_type.into_glib(),
                flags.into_glib(),
                info.to_glib_none().0,
                name.to_glib_none().0,
                object_path.to_glib_none().0,
                interface_name.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn for_bus_future(
        bus_type: BusType,
        flags: DBusProxyFlags,
        info: Option<&DBusInterfaceInfo>,
        name: &str,
        object_path: &str,
        interface_name: &str,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<DBusProxy, glib::Error>> + 'static>> {
        let info = info.map(ToOwned::to_owned);
        let name = String::from(name);
        let object_path = String::from(object_path);
        let interface_name = String::from(interface_name);
        Box_::pin(crate::GioFuture::new(
            &(),
            move |_obj, cancellable, send| {
                Self::for_bus(
                    bus_type,
                    flags,
                    info.as_ref().map(::std::borrow::Borrow::borrow),
                    &name,
                    &object_path,
                    &interface_name,
                    Some(cancellable),
                    move |res| {
                        send.resolve(res);
                    },
                );
            },
        ))
    }
}

unsafe impl Send for DBusProxy {}
unsafe impl Sync for DBusProxy {}

pub trait DBusProxyExt: 'static {
    #[doc(alias = "g_dbus_proxy_call")]
    fn call<P: FnOnce(Result<glib::Variant, glib::Error>) + 'static>(
        &self,
        method_name: &str,
        parameters: Option<&glib::Variant>,
        flags: DBusCallFlags,
        timeout_msec: i32,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    );

    fn call_future(
        &self,
        method_name: &str,
        parameters: Option<&glib::Variant>,
        flags: DBusCallFlags,
        timeout_msec: i32,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<glib::Variant, glib::Error>> + 'static>>;

    #[doc(alias = "g_dbus_proxy_call_sync")]
    fn call_sync(
        &self,
        method_name: &str,
        parameters: Option<&glib::Variant>,
        flags: DBusCallFlags,
        timeout_msec: i32,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<glib::Variant, glib::Error>;

    #[cfg(any(unix, feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(unix)))]
    #[doc(alias = "g_dbus_proxy_call_with_unix_fd_list")]
    fn call_with_unix_fd_list<
        P: FnOnce(Result<(glib::Variant, UnixFDList), glib::Error>) + 'static,
    >(
        &self,
        method_name: &str,
        parameters: Option<&glib::Variant>,
        flags: DBusCallFlags,
        timeout_msec: i32,
        fd_list: Option<&impl IsA<UnixFDList>>,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    );

    #[cfg(any(unix, feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(unix)))]
    fn call_with_unix_fd_list_future(
        &self,
        method_name: &str,
        parameters: Option<&glib::Variant>,
        flags: DBusCallFlags,
        timeout_msec: i32,
        fd_list: Option<&(impl IsA<UnixFDList> + Clone + 'static)>,
    ) -> Pin<
        Box_<
            dyn std::future::Future<Output = Result<(glib::Variant, UnixFDList), glib::Error>>
                + 'static,
        >,
    >;

    #[cfg(any(unix, feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(unix)))]
    #[doc(alias = "g_dbus_proxy_call_with_unix_fd_list_sync")]
    fn call_with_unix_fd_list_sync(
        &self,
        method_name: &str,
        parameters: Option<&glib::Variant>,
        flags: DBusCallFlags,
        timeout_msec: i32,
        fd_list: Option<&impl IsA<UnixFDList>>,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<(glib::Variant, UnixFDList), glib::Error>;

    #[doc(alias = "g_dbus_proxy_get_cached_property")]
    #[doc(alias = "get_cached_property")]
    fn cached_property(&self, property_name: &str) -> Option<glib::Variant>;

    #[doc(alias = "g_dbus_proxy_get_cached_property_names")]
    #[doc(alias = "get_cached_property_names")]
    fn cached_property_names(&self) -> Vec<glib::GString>;

    #[doc(alias = "g_dbus_proxy_get_connection")]
    #[doc(alias = "get_connection")]
    fn connection(&self) -> DBusConnection;

    #[doc(alias = "g_dbus_proxy_get_default_timeout")]
    #[doc(alias = "get_default_timeout")]
    fn default_timeout(&self) -> i32;

    #[doc(alias = "g_dbus_proxy_get_flags")]
    #[doc(alias = "get_flags")]
    fn flags(&self) -> DBusProxyFlags;

    #[doc(alias = "g_dbus_proxy_get_interface_info")]
    #[doc(alias = "get_interface_info")]
    fn interface_info(&self) -> Option<DBusInterfaceInfo>;

    #[doc(alias = "g_dbus_proxy_get_interface_name")]
    #[doc(alias = "get_interface_name")]
    fn interface_name(&self) -> glib::GString;

    #[doc(alias = "g_dbus_proxy_get_name")]
    #[doc(alias = "get_name")]
    fn name(&self) -> Option<glib::GString>;

    #[doc(alias = "g_dbus_proxy_get_name_owner")]
    #[doc(alias = "get_name_owner")]
    fn name_owner(&self) -> Option<glib::GString>;

    #[doc(alias = "g_dbus_proxy_get_object_path")]
    #[doc(alias = "get_object_path")]
    fn object_path(&self) -> glib::GString;

    #[doc(alias = "g_dbus_proxy_set_cached_property")]
    fn set_cached_property(&self, property_name: &str, value: Option<&glib::Variant>);

    #[doc(alias = "g_dbus_proxy_set_default_timeout")]
    fn set_default_timeout(&self, timeout_msec: i32);

    #[doc(alias = "g_dbus_proxy_set_interface_info")]
    fn set_interface_info(&self, info: Option<&DBusInterfaceInfo>);

    #[doc(alias = "g-connection")]
    fn g_connection(&self) -> Option<DBusConnection>;

    #[doc(alias = "g-default-timeout")]
    fn g_default_timeout(&self) -> i32;

    #[doc(alias = "g-default-timeout")]
    fn set_g_default_timeout(&self, g_default_timeout: i32);

    #[doc(alias = "g-flags")]
    fn g_flags(&self) -> DBusProxyFlags;

    #[doc(alias = "g-interface-info")]
    fn g_interface_info(&self) -> Option<DBusInterfaceInfo>;

    #[doc(alias = "g-interface-info")]
    fn set_g_interface_info(&self, g_interface_info: Option<&DBusInterfaceInfo>);

    #[doc(alias = "g-interface-name")]
    fn g_interface_name(&self) -> Option<glib::GString>;

    #[doc(alias = "g-name")]
    fn g_name(&self) -> Option<glib::GString>;

    #[doc(alias = "g-name-owner")]
    fn g_name_owner(&self) -> Option<glib::GString>;

    #[doc(alias = "g-object-path")]
    fn g_object_path(&self) -> Option<glib::GString>;

    #[doc(alias = "g-default-timeout")]
    fn connect_g_default_timeout_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "g-interface-info")]
    fn connect_g_interface_info_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "g-name-owner")]
    fn connect_g_name_owner_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<DBusProxy>> DBusProxyExt for O {
    fn call<P: FnOnce(Result<glib::Variant, glib::Error>) + 'static>(
        &self,
        method_name: &str,
        parameters: Option<&glib::Variant>,
        flags: DBusCallFlags,
        timeout_msec: i32,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn call_trampoline<
            P: FnOnce(Result<glib::Variant, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::g_dbus_proxy_call_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = call_trampoline::<P>;
        unsafe {
            ffi::g_dbus_proxy_call(
                self.as_ref().to_glib_none().0,
                method_name.to_glib_none().0,
                parameters.to_glib_none().0,
                flags.into_glib(),
                timeout_msec,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn call_future(
        &self,
        method_name: &str,
        parameters: Option<&glib::Variant>,
        flags: DBusCallFlags,
        timeout_msec: i32,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<glib::Variant, glib::Error>> + 'static>>
    {
        let method_name = String::from(method_name);
        let parameters = parameters.map(ToOwned::to_owned);
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.call(
                    &method_name,
                    parameters.as_ref().map(::std::borrow::Borrow::borrow),
                    flags,
                    timeout_msec,
                    Some(cancellable),
                    move |res| {
                        send.resolve(res);
                    },
                );
            },
        ))
    }

    fn call_sync(
        &self,
        method_name: &str,
        parameters: Option<&glib::Variant>,
        flags: DBusCallFlags,
        timeout_msec: i32,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<glib::Variant, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_dbus_proxy_call_sync(
                self.as_ref().to_glib_none().0,
                method_name.to_glib_none().0,
                parameters.to_glib_none().0,
                flags.into_glib(),
                timeout_msec,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(unix, feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(unix)))]
    fn call_with_unix_fd_list<
        P: FnOnce(Result<(glib::Variant, UnixFDList), glib::Error>) + 'static,
    >(
        &self,
        method_name: &str,
        parameters: Option<&glib::Variant>,
        flags: DBusCallFlags,
        timeout_msec: i32,
        fd_list: Option<&impl IsA<UnixFDList>>,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn call_with_unix_fd_list_trampoline<
            P: FnOnce(Result<(glib::Variant, UnixFDList), glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let mut out_fd_list = ptr::null_mut();
            let ret = ffi::g_dbus_proxy_call_with_unix_fd_list_finish(
                _source_object as *mut _,
                &mut out_fd_list,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok((from_glib_full(ret), from_glib_full(out_fd_list)))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = call_with_unix_fd_list_trampoline::<P>;
        unsafe {
            ffi::g_dbus_proxy_call_with_unix_fd_list(
                self.as_ref().to_glib_none().0,
                method_name.to_glib_none().0,
                parameters.to_glib_none().0,
                flags.into_glib(),
                timeout_msec,
                fd_list.map(|p| p.as_ref()).to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    #[cfg(any(unix, feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(unix)))]
    fn call_with_unix_fd_list_future(
        &self,
        method_name: &str,
        parameters: Option<&glib::Variant>,
        flags: DBusCallFlags,
        timeout_msec: i32,
        fd_list: Option<&(impl IsA<UnixFDList> + Clone + 'static)>,
    ) -> Pin<
        Box_<
            dyn std::future::Future<Output = Result<(glib::Variant, UnixFDList), glib::Error>>
                + 'static,
        >,
    > {
        let method_name = String::from(method_name);
        let parameters = parameters.map(ToOwned::to_owned);
        let fd_list = fd_list.map(ToOwned::to_owned);
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.call_with_unix_fd_list(
                    &method_name,
                    parameters.as_ref().map(::std::borrow::Borrow::borrow),
                    flags,
                    timeout_msec,
                    fd_list.as_ref().map(::std::borrow::Borrow::borrow),
                    Some(cancellable),
                    move |res| {
                        send.resolve(res);
                    },
                );
            },
        ))
    }

    #[cfg(any(unix, feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(unix)))]
    fn call_with_unix_fd_list_sync(
        &self,
        method_name: &str,
        parameters: Option<&glib::Variant>,
        flags: DBusCallFlags,
        timeout_msec: i32,
        fd_list: Option<&impl IsA<UnixFDList>>,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<(glib::Variant, UnixFDList), glib::Error> {
        unsafe {
            let mut out_fd_list = ptr::null_mut();
            let mut error = ptr::null_mut();
            let ret = ffi::g_dbus_proxy_call_with_unix_fd_list_sync(
                self.as_ref().to_glib_none().0,
                method_name.to_glib_none().0,
                parameters.to_glib_none().0,
                flags.into_glib(),
                timeout_msec,
                fd_list.map(|p| p.as_ref()).to_glib_none().0,
                &mut out_fd_list,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok((from_glib_full(ret), from_glib_full(out_fd_list)))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn cached_property(&self, property_name: &str) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(ffi::g_dbus_proxy_get_cached_property(
                self.as_ref().to_glib_none().0,
                property_name.to_glib_none().0,
            ))
        }
    }

    fn cached_property_names(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_dbus_proxy_get_cached_property_names(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn connection(&self) -> DBusConnection {
        unsafe {
            from_glib_none(ffi::g_dbus_proxy_get_connection(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn default_timeout(&self) -> i32 {
        unsafe { ffi::g_dbus_proxy_get_default_timeout(self.as_ref().to_glib_none().0) }
    }

    fn flags(&self) -> DBusProxyFlags {
        unsafe { from_glib(ffi::g_dbus_proxy_get_flags(self.as_ref().to_glib_none().0)) }
    }

    fn interface_info(&self) -> Option<DBusInterfaceInfo> {
        unsafe {
            from_glib_none(ffi::g_dbus_proxy_get_interface_info(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn interface_name(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::g_dbus_proxy_get_interface_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::g_dbus_proxy_get_name(self.as_ref().to_glib_none().0)) }
    }

    fn name_owner(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::g_dbus_proxy_get_name_owner(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn object_path(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::g_dbus_proxy_get_object_path(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_cached_property(&self, property_name: &str, value: Option<&glib::Variant>) {
        unsafe {
            ffi::g_dbus_proxy_set_cached_property(
                self.as_ref().to_glib_none().0,
                property_name.to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn set_default_timeout(&self, timeout_msec: i32) {
        unsafe {
            ffi::g_dbus_proxy_set_default_timeout(self.as_ref().to_glib_none().0, timeout_msec);
        }
    }

    fn set_interface_info(&self, info: Option<&DBusInterfaceInfo>) {
        unsafe {
            ffi::g_dbus_proxy_set_interface_info(
                self.as_ref().to_glib_none().0,
                info.to_glib_none().0,
            );
        }
    }

    fn g_connection(&self) -> Option<DBusConnection> {
        glib::ObjectExt::property(self.as_ref(), "g-connection")
    }

    fn g_default_timeout(&self) -> i32 {
        glib::ObjectExt::property(self.as_ref(), "g-default-timeout")
    }

    fn set_g_default_timeout(&self, g_default_timeout: i32) {
        glib::ObjectExt::set_property(self.as_ref(), "g-default-timeout", &g_default_timeout)
    }

    fn g_flags(&self) -> DBusProxyFlags {
        glib::ObjectExt::property(self.as_ref(), "g-flags")
    }

    fn g_interface_info(&self) -> Option<DBusInterfaceInfo> {
        glib::ObjectExt::property(self.as_ref(), "g-interface-info")
    }

    fn set_g_interface_info(&self, g_interface_info: Option<&DBusInterfaceInfo>) {
        glib::ObjectExt::set_property(self.as_ref(), "g-interface-info", &g_interface_info)
    }

    fn g_interface_name(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self.as_ref(), "g-interface-name")
    }

    fn g_name(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self.as_ref(), "g-name")
    }

    fn g_name_owner(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self.as_ref(), "g-name-owner")
    }

    fn g_object_path(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self.as_ref(), "g-object-path")
    }

    fn connect_g_default_timeout_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_g_default_timeout_trampoline<
            P: IsA<DBusProxy>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GDBusProxy,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DBusProxy::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::g-default-timeout\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_g_default_timeout_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_g_interface_info_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_g_interface_info_trampoline<
            P: IsA<DBusProxy>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GDBusProxy,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DBusProxy::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::g-interface-info\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_g_interface_info_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_g_name_owner_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_g_name_owner_trampoline<
            P: IsA<DBusProxy>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GDBusProxy,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DBusProxy::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::g-name-owner\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_g_name_owner_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DBusProxy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DBusProxy")
    }
}
