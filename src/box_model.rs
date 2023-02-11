// CSS box model. All sizes are in px.

use crate::style::css::StyledNode;

#[derive(Default)]
struct Dimensions {
    /// Position of the content area relative to the document origin
    content: Rect,

    // Surrounding edges
    padding: EdgeSizes,
    border: EdgeSizes,
    margin: EdgeSizes,
}

/// Rect is short for Rectangle :)
/// It is a cartesian shape :)
#[derive(Default)]
struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

#[derive(Default)]
struct EdgeSizes {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

/// Represents a "box" in the dom box model.
struct LayoutBox<'a> {
    dims: Dimensions,
    box_type: BoxType<'a>,
    children: Vec<LayoutBox<'a>>,
}

impl<'a> LayoutBox<'a> {
    fn new(box_type: BoxType<'a>) -> Self {
        Self {
            box_type,
            dims: Default::default(),
            children: Vec::new(),
        }
    }

    /// TODO: Understand this shit.
    ///
    /// This is intentionally simplified in a number of ways from the standard CSS box generation algorithm. For example, it doesn't handle the case where an inline box contains a block-level child. Also, it generates an unnecessary anonymous box if a block-level node has only inline children.
    fn get_inline_container(&mut self) -> &mut LayoutBox<'a> {
        match self.box_type {
            BoxType::InlineNode(_) | BoxType::AnonymousBlock => self,
            BoxType::BlockNode(_) => {
                let child = self.children.last();

                // This function assumes that there is at least one child.
                assert!(self.children.len() > 0);

                match child.unwrap().box_type {
                    BoxType::AnonymousBlock => {}
                    _ => self.children.push(LayoutBox::new(BoxType::AnonymousBlock)),
                }
                self.children.last_mut().unwrap()
            }
        }
    }
}

/// How should the box formatted?
enum BoxType<'a> {
    BlockNode(&'a StyledNode<'a>),
    InlineNode(&'a StyledNode<'a>),
    AnonymousBlock,
}

/// The values for the display property.
pub enum Display {
    Inline,
    Block,
    None,
}

/// Convert a [StyledNode] to its corresponding [BoxType].
fn node_to_box<'a>(n: &'a StyledNode) -> BoxType<'a> {
    match n.display() {
        Display::Inline => BoxType::InlineNode(n),
        Display::Block => BoxType::BlockNode(n),
        Display::None => BoxType::AnonymousBlock,
    }
}

/// Build a tree of [LayoutBox]s. Not performing any calculations yet.
fn build_layout_tree<'a>(style_node: &'a StyledNode) -> LayoutBox<'a> {
    // Create the root box
    let mut root = LayoutBox::new(node_to_box(style_node));

    // Create all children boxes
    for c in &style_node.children {
        match c.display() {
            Display::Block => root.children.push(build_layout_tree(c)),
            Display::Inline => root
                .get_inline_container()
                .children
                .push(build_layout_tree(c)),
            Display::None => {}
        }
    }

    root
}
