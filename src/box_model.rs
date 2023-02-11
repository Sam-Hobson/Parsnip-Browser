// CSS box model. All sizes are in px.

use crate::style::css::{StyledNode, Unit};

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
                assert!(!self.children.is_empty());

                match child.unwrap().box_type {
                    BoxType::AnonymousBlock => {}
                    _ => self.children.push(LayoutBox::new(BoxType::AnonymousBlock)),
                }
                self.children.last_mut().unwrap()
            }
        }
    }

    fn layout(&mut self, containing_block: Dimensions) {
        match self.box_type {
            BoxType::BlockNode(_) => self.layout_block(containing_block),
            BoxType::InlineNode(_) => unimplemented!(),
            BoxType::AnonymousBlock => unimplemented!(),
        }
    }

    fn layout_block(&mut self, containing_block: Dimensions) {
        // Child width can depend on parent width, so we need to calculate this box's width before laying out its children.
        self.calculate_block_width(containing_block);

        // Determine where the box is located within its container.
        self.calculate_block_position(containing_block);

        // Recursively lay out the children of this box.
        self.layout_block_children();

        // Parent height can depend on child height, so `calculate_height` must be called *after* the children are laid out.
        self.calculate_block_height();
    }

    fn calculate_block_width(&mut self, containing_block: Dimensions) {
        let style = self.get_style_node();

        // `width` has initial value `auto`.
        let auto = Keyword("auto".to_string());
        let mut width = style.value("width").unwrap_or(auto.clone());

        // margin, border, and padding have initial value 0.
        let zero = Length(0.0, Unit::Px);

        let mut margin_left = style.lookup("margin-left", "margin", &zero);
        let mut margin_right = style.lookup("margin-right", "margin", &zero);

        let border_left = style.lookup("border-left-width", "border-width", &zero);
        let border_right = style.lookup("border-right-width", "border-width", &zero);

        let padding_left = style.lookup("padding-left", "padding", &zero);
        let padding_right = style.lookup("padding-right", "padding", &zero);

        // ...
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
