initSidebarItems({"attr":[["gflags","Attribute macro for defining flags using the `bitflags` crate. This macro will also define a `GFlags::get_type` function and the [`glib::Value`] traits."]],"constant":[["CLONE_MACRO_LOG_DOMAIN","This is the log domain used by the [`clone!`][crate::clone] macro. If you want to use a custom logger (it prints to stdout by default), you can set your own logger using the corresponding `log` functions."]],"derive":[["GBoxed","Derive macro for defining a [`BoxedType`]`::get_type` function and the [`glib::Value`] traits."],["GEnum",""]],"enum":[["ChecksumType",""],["DateMonth",""],["DateWeekday",""],["FileError",""],["KeyFileError",""],["LogLevel",""],["OptionArg",""],["SeekType",""],["TimeType",""],["UserDirectory",""]],"fn":[["access",""],["assert_warning",""],["assertion_message",""],["assertion_message_cmpstr",""],["base64_decode",""],["base64_encode",""],["bit_nth_lsf",""],["bit_nth_msf",""],["bit_storage",""],["build_filenamev",""],["build_pathv",""],["canonicalize_filename",""],["chdir",""],["check_version",""],["child_watch_future","Create a `Future` that will resolve once the child process with the given pid exits"],["child_watch_future_with_priority","Create a `Future` that will resolve once the child process with the given pid exits"],["clear_error",""],["compute_checksum_for_bytes",""],["compute_checksum_for_data",""],["compute_checksum_for_string",""],["compute_hmac_for_bytes",""],["compute_hmac_for_data",""],["compute_hmac_for_string",""],["dcgettext",""],["dgettext",""],["dngettext",""],["dpgettext",""],["dpgettext2",""],["environ_getenv",""],["file_get_contents",""],["file_open_tmp",""],["file_read_link",""],["file_set_contents",""],["file_test",""],["filename_display_basename",""],["filename_display_name",""],["filename_from_uri",""],["filename_to_uri",""],["find_program_in_path",""],["format_size",""],["format_size_full",""],["get_application_name",""],["get_charset",""],["get_codeset",""],["get_console_charset",""],["get_current_dir",""],["get_current_time",""],["get_environ",""],["get_home_dir",""],["get_host_name",""],["get_language_names",""],["get_language_names_with_category",""],["get_locale_variants",""],["get_monotonic_time",""],["get_num_processors",""],["get_os_info",""],["get_prgname",""],["get_program_name","Same as [`get_prgname()`]."],["get_real_name",""],["get_real_time",""],["get_system_config_dirs",""],["get_system_data_dirs",""],["get_tmp_dir",""],["get_user_cache_dir",""],["get_user_config_dir",""],["get_user_data_dir",""],["get_user_name",""],["get_user_runtime_dir",""],["get_user_special_dir",""],["getenv",""],["hostname_is_ascii_encoded",""],["hostname_is_ip_address",""],["hostname_is_non_ascii",""],["hostname_to_ascii",""],["hostname_to_unicode",""],["interval_stream","Create a `Stream` that will provide a value every given number of milliseconds."],["interval_stream_seconds","Create a `Stream` that will provide a value every given number of seconds."],["interval_stream_seconds_with_priority","Create a `Stream` that will provide a value every given number of seconds."],["interval_stream_with_priority","Create a `Stream` that will provide a value every given number of milliseconds."],["listenv",""],["log_default_handler",""],["log_remove_handler",""],["log_set_always_fatal",""],["log_set_default_handler","To set back the default print handler, use the [`log_unset_default_handler`] function."],["log_set_fatal_mask",""],["log_set_handler",""],["log_unset_default_handler","To set the default print handler, use the [`log_set_default_handler`] function."],["main_current_source",""],["main_depth",""],["markup_escape_text",""],["mem_is_system_malloc",""],["mem_profile",""],["mkdir_with_parents",""],["mkdtemp",""],["mkdtemp_full",""],["mkstemp",""],["mkstemp_full",""],["on_error_query",""],["on_error_stack_trace",""],["path_get_basename",""],["path_get_dirname",""],["path_is_absolute",""],["path_skip_root",""],["pattern_match_simple",""],["random_double",""],["random_double_range",""],["random_int",""],["random_int_range",""],["random_set_seed",""],["reload_user_special_dirs_cache",""],["return_if_fail_warning",""],["rmdir",""],["set_application_name",""],["set_prgname",""],["set_print_handler","To set back the default print handler, use the [`unset_print_handler`] function."],["set_printerr_handler","To set back the default print handler, use the [`unset_printerr_handler`] function."],["set_program_name","Same as [`set_prgname()`]."],["setenv",""],["shell_parse_argv",""],["shell_quote",""],["shell_unquote",""],["spaced_primes_closest",""],["spawn_async",""],["spawn_async_with_fds",""],["spawn_async_with_pipes",""],["spawn_check_exit_status",""],["spawn_command_line_async",""],["stpcpy",""],["timeout_future","Create a `Future` that will resolve after the given number of milliseconds."],["timeout_future_seconds","Create a `Future` that will resolve after the given number of seconds."],["timeout_future_seconds_with_priority","Create a `Future` that will resolve after the given number of seconds."],["timeout_future_with_priority","Create a `Future` that will resolve after the given number of milliseconds."],["unix_signal_future","Create a `Future` that will resolve once the given UNIX signal is raised"],["unix_signal_future_with_priority","Create a `Future` that will resolve once the given UNIX signal is raised"],["unix_signal_stream","Create a `Stream` that will provide a value whenever the given UNIX signal is raised"],["unix_signal_stream_with_priority","Create a `Stream` that will provide a value whenever the given UNIX signal is raised"],["unlink",""],["unset_print_handler","To set the default print handler, use the [`set_print_handler`] function."],["unset_printerr_handler","To set the default print handler, use the [`set_printerr_handler`] function."],["unsetenv",""],["uri_escape_string",""],["uri_list_extract_uris",""],["uri_parse_scheme",""],["uri_unescape_segment",""],["uri_unescape_string",""],["usleep",""],["uuid_string_is_valid",""],["uuid_string_random",""],["warn_message",""]],"macro":[["clone","Macro for passing variables as strong or weak references into a closure."],["g_critical","Macro used to log using GLib logging system. It uses [g_log]."],["g_debug","Macro used to log using GLib logging system. It uses [g_log]."],["g_error","Macro used to log using GLib logging system. It uses [g_log]."],["g_info","Macro used to log using GLib logging system. It uses [g_log]."],["g_log","Macro used to log using GLib logging system. It uses [g_log]."],["g_message","Macro used to log using GLib logging system. It uses [g_log]."],["g_print","Macro used to print messages. It uses [g_print]."],["g_printerr","Macro used to print error messages. It uses [g_printerr]."],["g_warning","Macro used to log using GLib logging system. It uses [g_log]."],["glib_bool_error","Generic error used for functions that fail without any further information"],["glib_boxed_wrapper","Wrapper implementations for Boxed types. See `glib_wrapper!`."],["glib_object_impl","Macro for boilerplate of [`ObjectImpl`] implementations."],["glib_object_interface","Macro for boilerplate of [`ObjectInterface`] implementations."],["glib_object_subclass","Macro for boilerplate of [`ObjectSubclass`] implementations."],["glib_object_wrapper","ObjectType implementations for Object types. See `glib_wrapper!`."],["glib_result_from_gboolean",""],["glib_shared_wrapper","Wrapper implementations for shared types. See `glib_wrapper!`."],["glib_wrapper","Defines a wrapper type and implements the appropriate traits."]],"mod":[["auto",""],["boxed","`IMPL` Boxed wrapper implementation."],["char",""],["clone",""],["closure",""],["error","`Error` binding and helper trait."],["functions",""],["object","`IMPL` Object wrapper implementation and `Object` binding."],["prelude","Traits and essential types intended for blanket imports."],["send_unique",""],["shared","`IMPL` Shared (reference counted) wrapper implementation."],["signal","`IMPL` Low level signal support."],["source",""],["subclass","Module containing infrastructure for subclassing `GObject`s and registering boxed types."],["translate","Translation between GLib/GLib-based FFI types and their Rust counterparts."],["types","Runtime type information."],["value","`Value` binding and helper traits."],["variant","`Variant` binding and helper traits."],["wrapper","`IMPL` The `glib_wrapper!` macro and miscellaneous wrapper traits."]],"static":[["CSET_A_2_Z",""],["CSET_DIGITS",""],["CSET_a_2_z",""],["KEY_FILE_DESKTOP_GROUP",""],["KEY_FILE_DESKTOP_KEY_ACTIONS",""],["KEY_FILE_DESKTOP_KEY_CATEGORIES",""],["KEY_FILE_DESKTOP_KEY_COMMENT",""],["KEY_FILE_DESKTOP_KEY_DBUS_ACTIVATABLE",""],["KEY_FILE_DESKTOP_KEY_EXEC",""],["KEY_FILE_DESKTOP_KEY_GENERIC_NAME",""],["KEY_FILE_DESKTOP_KEY_HIDDEN",""],["KEY_FILE_DESKTOP_KEY_ICON",""],["KEY_FILE_DESKTOP_KEY_MIME_TYPE",""],["KEY_FILE_DESKTOP_KEY_NAME",""],["KEY_FILE_DESKTOP_KEY_NOT_SHOW_IN",""],["KEY_FILE_DESKTOP_KEY_NO_DISPLAY",""],["KEY_FILE_DESKTOP_KEY_ONLY_SHOW_IN",""],["KEY_FILE_DESKTOP_KEY_PATH",""],["KEY_FILE_DESKTOP_KEY_STARTUP_NOTIFY",""],["KEY_FILE_DESKTOP_KEY_STARTUP_WM_CLASS",""],["KEY_FILE_DESKTOP_KEY_TERMINAL",""],["KEY_FILE_DESKTOP_KEY_TRY_EXEC",""],["KEY_FILE_DESKTOP_KEY_TYPE",""],["KEY_FILE_DESKTOP_KEY_URL",""],["KEY_FILE_DESKTOP_KEY_VERSION",""],["KEY_FILE_DESKTOP_TYPE_APPLICATION",""],["KEY_FILE_DESKTOP_TYPE_DIRECTORY",""],["KEY_FILE_DESKTOP_TYPE_LINK",""],["OPTION_REMAINING",""],["STR_DELIMITERS",""],["TEST_OPTION_ISOLATE_DIRS",""],["URI_RESERVED_CHARS_GENERIC_DELIMITERS",""],["URI_RESERVED_CHARS_SUBCOMPONENT_DELIMITERS",""]],"struct":[["Binding",""],["BindingClass",""],["BindingFlags",""],["ByteArray",""],["Bytes","A shared immutable byte slice (the equivalent of `Rc<[u8]>`)."],["Checksum",""],["Date",""],["DateTime",""],["EnumClass","Representation of an `enum` for dynamically, at runtime, querying the values of the enum and using them."],["EnumValue","Representation of a single enum value of an `EnumClass`."],["FileTest",""],["FlagsBuilder","Builder for conveniently setting/unsetting flags and returning a `Value`."],["FlagsClass","Representation of a `flags` for dynamically, at runtime, querying the values of the enum and using them"],["FlagsValue","Representation of a single flags value of a `FlagsClass`."],["FormatSizeFlags",""],["GString",""],["IOCondition",""],["KeyFile",""],["KeyFileFlags",""],["LogHandlerId",""],["LogLevelFlags",""],["LogLevels",""],["MainContext",""],["MainLoop",""],["OptionFlags",""],["ParamFlags",""],["ParamSpec",""],["Quark",""],["Receiver","A `Receiver` that can be attached to a main context to receive items from its corresponding `Sender` or `SyncSender`."],["Sender","A `Sender` that can be used to send items to the corresponding main context receiver."],["SignalFlags",""],["Source",""],["SourceFuture","Represents a `Future` around a `glib::Source`. The future will be resolved once the source has provided a value"],["SourceStream","Represents a `Stream` around a `glib::Source`. The stream will be provide all values that are provided by the source"],["SpawnFlags",""],["String","A mutable text buffer that grows automatically."],["SyncSender","A `SyncSender` that can be used to send items to the corresponding main context receiver."],["ThreadPool",""],["TimeVal",""],["TimeZone",""],["ValueArray",""],["VariantDict","`VariantDict` is a mutable key/value store where the keys are always strings and the values are `Variant`s."],["VariantTy","Describes `Variant` types."],["VariantType","Describes `Variant` types."]],"type":[["DateDay",""],["DateYear",""],["Time",""],["TimeSpan",""]]});