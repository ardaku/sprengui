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

use crate::script::{NodeScript, Script, SpecialScript};
use crate::{node, RealEq};
use std::marker::PhantomData;
use std::ops::Deref;

/// The 2D section of the node tree
pub mod node2d {
    use crate::node::{Node, NodeStruct};
    use crate::script::{NoScript, NodeScript};

    /// The blank 2d node
    pub struct Node2D {
        node: NodeStruct<Self, NodeScript<Self>, NoScript>,
    }

    impl Node<NodeScript<Self>> for Node2D {
        fn attach_script(&mut self, script: NodeScript<Self>) {
            self.node.script = Some(script);
        }

        fn detach_script(&mut self) -> bool {
            if let Some(_script) = self.node.script {
                self.node.script = None;
                true
            } else {
                false
            }
        }

        fn node_id(&self) -> u64 {
            self.node.id
        }
    }

    /// All GUI related nodes
    pub mod gui {
        /// Button nodes
        pub mod button {
            use crate::{
                node::{Node, NodeStruct},
                script::{ButtonScript, NodeScript},
            };
            use winit::event::WindowEvent;
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
                /// All the data about the button
                node: NodeStruct<Self, NodeScript<Self>, ButtonScript<'script>>,
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

                fn node_id(&self) -> u64 {
                    self.node.id
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

                fn node_id(&self) -> u64 {
                    self.node.id
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
pub trait Node<S: Script + RealEq> {
    /// Attach a script to the node
    fn attach_script(&mut self, script: S);
    /// Detach a script from the node
    /// returns whether or not there was a node to be detached
    fn detach_script(&mut self) -> bool;
    /// Returns an ID to the node
    fn node_id(&self) -> u64;
}
impl<S: Script + RealEq> RealEq for dyn Node<S> {
    fn eq(&self, other: &Self) -> bool {
        self.node_id() == other.node_id()
    }
}

/// Contained by every Node
pub struct NodeStruct<T: Node<S>, S: Script + RealEq, SS: SpecialScript> {
    /// The standard script, a NodeScript
    script: Option<NodeScript<T>>,
    /// The special script
    special_script: Option<SS>,
    /// Whether or not the node is currently visible
    pub visible: bool,
    /// The position of the node
    pub position: NodePosition,
    /// The ID of the node
    pub id: u64,
    /// Empty filler
    _empty: Option<S>,
}

/// Contained by [`NodeStruct`]
pub struct NodePosition {
    /// The x position of the node
    pub x: f32,
    /// The y position of the node
    pub y: f32,
    /// The z position of the node
    pub z: f32,
    /// The rotation of the node
    pub rotation: f32,
    /// The X-scale of the node
    pub x_scale: f32,
    /// The Y-scale of the node
    pub y_scale: f32,
    /// The Z-scale of the node
    pub z_scale: f32,
}

pub struct NodeTree<R: Node<S> + ?Sized, S: Script + RealEq> {
    pub root: Box<R>,
    pub children: Vec<NodeTree<R, S>>,
    pub is_root: bool,
    phantom: PhantomData<S>,
}

impl<
        N: RealEq
            + Node<
                for<'r, 's, 't0> fn(
                    &'r mut (dyn Node<N> + 'r),
                    &'s winit::event::Event<'t0, ()>,
                ),
            >,
    > RealEq for NodeTree<N, NodeScript<N>>
{
    fn eq(&self, other: &Self) -> bool {
        self.root.eq(&other.root) && self.children.eq(&other.children)
    }
}

impl<S: Script + RealEq> RealEq for NodeTree<dyn Node<S>, S> {
    fn eq(&self, other: &Self) -> bool {
        self.root.eq(other.root.deref()) && self.children.eq(&other.children)
    }
}

impl<N: RealEq> RealEq for Vec<N> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len()
            && self.iter().zip(other.iter()).all(|(a, b)| a.eq(b))
    }
}

impl<N: RealEq> RealEq for Box<N> {
    fn eq(&self, other: &Self) -> bool {
        self.eq(other)
    }
}

impl<S: Script + RealEq> NodeTree<dyn Node<S>, S> {
    pub(crate) fn new(root: Box<dyn Node<S>>) -> Self {
        NodeTree {
            root,
            children: Vec::new(),
            is_root: true,
            phantom: PhantomData,
        }
    }

    pub(crate) fn new_no_root(root: Box<dyn Node<S>>) -> Self {
        NodeTree {
            root,
            children: Vec::new(),
            is_root: false,
            phantom: PhantomData,
        }
    }

    pub(crate) fn add_child(&mut self, child: NodeTree<dyn Node<S>, S>) {
        self.children.push(child);
    }

    pub(crate) fn remove_child(&mut self, child_in: &NodeTree<dyn Node<S>, S>) {
        // check if the child is in the children list
        let mut i: usize = 0;
        for child in self.children.iter() {
            if child.root.eq(&*child_in.root)
                && child.children.iter().zip(child_in.children.iter()).all(
                    |(a, b): (
                        &NodeTree<dyn Node<S>, S>,
                        &NodeTree<dyn Node<S>, S>,
                    )| RealEq::eq(a, b),
                )
            {
                self.children.remove(i);
                return;
            }
            i += 1;
        }
    }
}
