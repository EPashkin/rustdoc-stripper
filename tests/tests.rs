// Copyright 2016 Gomez Guillaume
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate stripper_lib;
extern crate tempdir;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use tempdir::TempDir;

const BASIC : &'static str = r#"/// struct Foo comment
struct Foo {
    /// Foo comment
    /// fn some_func(a: u32,
    ///              b: u32) {}
    A: u32,
}

mod Bar {
    test! {
        /// struct inside macro
        struct SuperFoo;
        sub_test! {
            /// and another one!
            struct FooFoo {
                x: u32,
            }
        }
    }
}
"#;

const BASIC_STRIPPED : &'static str = r#"struct Foo {
    A: u32,
}

mod Bar {
    test! {
        struct SuperFoo;
        sub_test! {
            struct FooFoo {
                x: u32,
            }
        }
    }
}
"#;

fn get_basic_md(file: &str) -> String {
    format!(r#"<!-- file {} -->
<!-- struct Foo -->
struct Foo comment
<!-- struct Foo§variant A -->
Foo comment
fn some_func(a: u32,
             b: u32) {{}}
<!-- mod Bar§macro test!§struct SuperFoo -->
struct inside macro
<!-- mod Bar§macro test!§macro sub_test!§struct FooFoo -->
and another one!
"#, file)
}

const BASIC2 : &'static str = r#"
use Bin;
use Box;
use Buildable;
use Container;
use Widget;
use Window;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    /// Dialog boxes are a convenient way to prompt the user for a small amount
    /// of input, e.g. to display a message, ask a question, or anything else
    /// that does not require extensive effort on the user’s part.
    ///
    /// ```
    /// {
    ///  dialog = gtk_dialog_new_with_buttons ("Message",
    ///                                        parent,
    ///                                        flags,
    ///                                        _("_OK"),
    ///                                        GTK_RESPONSE_NONE,
    ///                                        NULL);
    /// }
    /// ```
    pub struct Dialog(Object<ffi::GtkDialog>): Widget, Container, Bin, Window, Buildable;

    match fn {
        get_type => || ffi::gtk_dialog_get_type(),
    }
}

impl Dialog {
    /// Creates a new dialog box.
    ///
    /// Widgets should not be packed into this `Window`
    /// directly, but into the `vbox` and `action_area`, as described above.
    ///
    /// # Returns
    ///
    /// the new dialog as a `Widget`
    pub fn new() -> Dialog {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_dialog_new()).downcast_unchecked()
        }
    }

    //pub fn new_with_buttons<T: IsA<Window>>(title: Option<&str>, parent: Option<&T>, flags: DialogFlags, first_button_text: Option<&str>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Dialog {
    //    unsafe { TODO: call ffi::gtk_dialog_new_with_buttons() }
    //}
}

/// Trait containing all `Dialog` methods.
pub trait DialogExt {
    /// Adds an activatable widget to the action area of a `Dialog`,
    /// connecting a signal handler that will emit the `Dialog::response`
    /// signal on the dialog when the widget is activated. The widget is
    /// appended to the end of the dialog’s action area. If you want to add a
    /// non-activatable widget, simply pack it into the `action_area` field
    /// of the `Dialog` struct.
    fn add_action_widget<T: IsA<Widget>>(&self, child: &T, response_id: i32);

    /// Adds a button with the given text
    fn add_button(&self, button_text: &str, response_id: i32) -> Widget;
}
"#;

const BASIC2_STRIPPED : &'static str = r#"
use Bin;
use Box;
use Buildable;
use Container;
use Widget;
use Window;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct Dialog(Object<ffi::GtkDialog>): Widget, Container, Bin, Window, Buildable;

    match fn {
        get_type => || ffi::gtk_dialog_get_type(),
    }
}

impl Dialog {
    pub fn new() -> Dialog {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_dialog_new()).downcast_unchecked()
        }
    }

    //pub fn new_with_buttons<T: IsA<Window>>(title: Option<&str>, parent: Option<&T>, flags: DialogFlags, first_button_text: Option<&str>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Dialog {
    //    unsafe { TODO: call ffi::gtk_dialog_new_with_buttons() }
    //}
}

pub trait DialogExt {
    fn add_action_widget<T: IsA<Widget>>(&self, child: &T, response_id: i32);

    fn add_button(&self, button_text: &str, response_id: i32) -> Widget;
}
"#;

fn get_basic2_md(file: &str) -> String {
    format!(r#"<!-- file {} -->
<!-- struct Dialog -->
Dialog boxes are a convenient way to prompt the user for a small amount
of input, e.g. to display a message, ask a question, or anything else
that does not require extensive effort on the user’s part.

```
{{
 dialog = gtk_dialog_new_with_buttons ("Message",
                                       parent,
                                       flags,
                                       _("_OK"),
                                       GTK_RESPONSE_NONE,
                                       NULL);
}}
```
<!-- impl Dialog§fn new -->
Creates a new dialog box.

Widgets should not be packed into this `Window`
directly, but into the `vbox` and `action_area`, as described above.

# Returns

the new dialog as a `Widget`
<!-- trait DialogExt -->
Trait containing all `Dialog` methods.
<!-- trait DialogExt§fn add_action_widget -->
Adds an activatable widget to the action area of a `Dialog`,
connecting a signal handler that will emit the `Dialog::response`
signal on the dialog when the widget is activated. The widget is
appended to the end of the dialog’s action area. If you want to add a
non-activatable widget, simply pack it into the `action_area` field
of the `Dialog` struct.
<!-- trait DialogExt§fn add_button -->
Adds a button with the given text
"#, file)
}

const BASIC2_MD: &'static str = r#"<!-- file * -->
<!-- struct Dialog -->
Dialog boxes are a convenient way to prompt the user for a small amount
of input, e.g. to display a message, ask a question, or anything else
that does not require extensive effort on the user’s part.

```
{
 dialog = gtk_dialog_new_with_buttons ("Message",
                                       parent,
                                       flags,
                                       _("_OK"),
                                       GTK_RESPONSE_NONE,
                                       NULL);
}
```
<!-- impl Dialog§fn new -->
Creates a new dialog box.

Widgets should not be packed into this `Window`
directly, but into the `vbox` and `action_area`, as described above.

# Returns

the new dialog as a `Widget`
<!-- impl Dialog§fn new_with_buttons -->
Creates a new `Dialog` with title `title` (or `None` for the default
title; see `Window::set_title`) and transient parent `parent` (or
`None` for none; see `Window::set_transient_for`).
<!-- trait DialogExt -->
Trait containing all `Dialog` methods.
<!-- trait DialogExt§fn add_action_widget -->
Adds an activatable widget to the action area of a `Dialog`,
connecting a signal handler that will emit the `Dialog::response`
signal on the dialog when the widget is activated. The widget is
appended to the end of the dialog’s action area. If you want to add a
non-activatable widget, simply pack it into the `action_area` field
of the `Dialog` struct.
<!-- trait DialogExt§fn add_button -->
Adds a button with the given text
"#;

const BASIC3 : &'static str = r#"///struct Foo comment
struct Foo;
"#;

const BASIC3_STRIPPED : &'static str = r#"struct Foo;
"#;

const BASIC3_REGEN : &'static str = r#"/// struct Foo comment
struct Foo;
"#;

fn get_basic3_md(file: &str) -> String {
    format!(r#"<!-- file {} -->
<!-- struct Foo -->
struct Foo comment
"#, file)
}

fn gen_file(temp_dir: &TempDir, filename: &str, content: &str) -> File {
    let mut f = File::create(temp_dir.path().join(filename)).expect("gen_file");
    write!(f, "{}", content).unwrap();
    f
}

fn compare_files(expected_content: &str, file: &Path) {
    let mut f = File::open(file).expect("compare_files '{}'");
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    println!("");
    for (l, r) in expected_content.lines().zip(buf.lines()) {
        assert_eq!(l, r);
        println!("{}", l);
    }
    assert!(expected_content == &buf);
}

#[allow(unused_must_use)]
#[test]
fn test_strip() {
    let test_file = "basic.rs";
    let comment_file = "basic.md";
    let temp_dir = TempDir::new("").unwrap();
    gen_file(&temp_dir, test_file, BASIC);
    {
        let mut f = gen_file(&temp_dir, comment_file, "");
        stripper_lib::strip_comments(temp_dir.path(), test_file, &mut f, false);
    }
    compare_files(&get_basic_md(test_file), &temp_dir.path().join(comment_file));
    compare_files(BASIC_STRIPPED, &temp_dir.path().join(test_file));
}

#[allow(unused_must_use)]
#[test]
fn test_regeneration() {
    let test_file = "basic.rs";
    let comment_file = "basic.md";
    let temp_dir = TempDir::new("").unwrap();
    gen_file(&temp_dir, test_file, BASIC_STRIPPED);
    gen_file(&temp_dir, comment_file, &get_basic_md(test_file));
    stripper_lib::regenerate_doc_comments(temp_dir.path().to_str().unwrap(), false,
                                          &temp_dir.path().join(comment_file).to_str().unwrap(),
                                          false);
    compare_files(BASIC, &temp_dir.path().join(test_file));
}

#[allow(unused_must_use)]
#[test]
fn test2_strip() {
    let test_file = "basic.rs";
    let comment_file = "basic.md";
    let temp_dir = TempDir::new("").unwrap();
    gen_file(&temp_dir, test_file, BASIC2);
    {
        let mut f = gen_file(&temp_dir, comment_file, "");
        stripper_lib::strip_comments(temp_dir.path(), test_file, &mut f, true);
    }
    compare_files(&get_basic2_md(test_file), &temp_dir.path().join(comment_file));
    compare_files(BASIC2_STRIPPED, &temp_dir.path().join(test_file));
}

#[allow(unused_must_use)]
#[test]
fn test2_regeneration() {
    let test_file = "basic.rs";
    let comment_file = "basic.md";
    let temp_dir = TempDir::new("").unwrap();
    gen_file(&temp_dir, test_file, BASIC2_STRIPPED);
    gen_file(&temp_dir, comment_file, BASIC2_MD);
    stripper_lib::regenerate_doc_comments(temp_dir.path().to_str().unwrap(), false,
                                          &temp_dir.path().join(comment_file).to_str().unwrap(),
                                          true);
    compare_files(BASIC2, &temp_dir.path().join(test_file));
}

#[allow(unused_must_use)]
#[test]
fn test3_strip() {
    let test_file = "basic.rs";
    let comment_file = "basic.md";
    let temp_dir = TempDir::new("").unwrap();
    gen_file(&temp_dir, test_file, BASIC3);
    {
        let mut f = gen_file(&temp_dir, comment_file, "");
        stripper_lib::strip_comments(temp_dir.path(), test_file, &mut f, false);
    }
    compare_files(&get_basic3_md(test_file), &temp_dir.path().join(comment_file));
    compare_files(BASIC3_STRIPPED, &temp_dir.path().join(test_file));
}

#[allow(unused_must_use)]
#[test]
fn test3_regeneration() {
    let test_file = "basic.rs";
    let comment_file = "basic.md";
    let temp_dir = TempDir::new("").unwrap();
    gen_file(&temp_dir, test_file, BASIC3_STRIPPED);
    gen_file(&temp_dir, comment_file, &get_basic3_md(test_file));
    stripper_lib::regenerate_doc_comments(temp_dir.path().to_str().unwrap(), false,
                                          &temp_dir.path().join(comment_file).to_str().unwrap(),
                                          false);
    compare_files(BASIC3_REGEN, &temp_dir.path().join(test_file));
}
