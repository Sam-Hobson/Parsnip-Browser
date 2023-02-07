// CSS box model. All sizes are in px.

use crate::style::css::StyledNode;

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
struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

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
    None
}

/// Build a tree of [LayoutBox]s. Not performing any calculations yet.
fn build_layout_tree<'a>(style_node: &'a StyledNode) -> LayoutBox<'a> {

}
