// Sprengui
// Copyright © 2020-2021 Jeron Aldaron Lau.
//
// Licensed under any of:
//  - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
//  - MIT License (https://mit-license.org/)
//  - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
#![allow(clippy::blanket_clippy_restriction_lints)]
#![allow(clippy::missing_inline_in_public_items)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::exhaustive_structs)]
#![allow(clippy::implicit_return)]

use crate::script::{NodeScript, Script};

/// The 2D section of the node tree
pub mod node2d {
    /// All GUI related nodes
    pub mod gui {
        /// Button nodes
        pub mod button {
            use crate::node::{Node, NodeStruct};
            use crate::script::{ButtonScript, NodeScript};
            use winit::event::{Event, WindowEvent};

            /// Implemented by all button nodes
            pub trait Button {
                /// What to do when the button is pressed
                fn on_click(&mut self, event: &WindowEvent);
                /// What to do when the button is released
                fn on_release(&mut self, event: &WindowEvent);
                /// What to do when the button is hovered over
                fn on_hover(&mut self);
            }

            /// A humble button node
            pub struct SimpleButton<'script> {
                /// All the data about the node
                node: NodeStruct<Self, ButtonScript<'script>>,
            }

            impl<'script> Node<NodeScript<Self>> for SimpleButton<'script> {
                fn attach_script(&mut self, script: NodeScript<Self>) {
                    self.node.script = Some(script);
                }

                fn detach_script(&mut self) -> bool {
                    if self.node.script.is_some() {
                        self.node.script = None;
                        true
                    } else {
                        false
                    }
                }
            }

            impl<'script> Node<ButtonScript<'script>> for SimpleButton<'script> {
                fn attach_script(&mut self, script: ButtonScript<'script>) {
                    self.node.special_script = Some(script);
                }

                fn detach_script(&mut self) -> bool {
                    if self.node.special_script.is_some() {
                        self.node.special_script = None;
                        true
                    } else {
                        false
                    }
                }
            }

            impl<'script> Button for SimpleButton<'script> {
                fn on_click(&mut self, event: &WindowEvent) {
                    if let Some(script) = self.node.special_script {
                        (script.on_click)(self, event);
                    }
                }

                fn on_release(&mut self, event: &WindowEvent) {
                    if let Some(script) = self.node.special_script {
                        (script.on_release)(self, event);
                    }
                }

                fn on_hover(&mut self) {
                    if let Some(script) = self.node.special_script {
                        (script.on_hover)(self);
                    }
                }
            }
        }
    }
}

/// The 3D section of the node tree
pub mod node3d {}

/// Implemented by every node
pub trait Node<T: Script> {
    /// Attach a script to the node
    fn attach_script(&mut self, script: T);
    /// Detach a script from the node
    /// returns whether or not there was a node to be detached
    fn detach_script(&mut self) -> bool;
}

pub struct NodeStruct<T, S: Script> {
    script: Option<NodeScript<T>>,
    special_script: Option<S>,
    visible: bool,
}
