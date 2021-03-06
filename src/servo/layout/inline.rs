#[doc="Inline layout."]

import dom::rcu;
import dom::rcu::ReaderMethods;
import geom::point::Point2D;
import geom::size::Size2D;
import gfx::geometry::au;
import layout::base::*; // FIXME: Can't get around import *; resolve bug.
import layout::style::style::*; // ditto
import util::tree;

#[doc="The main reflow routine for inline layout."]
impl inline_layout_methods for @Box {
    fn reflow_inline(available_width: au) {
        assert self.kind == InlineBox;

        #debug["starting reflow inline"];

        // FIXME: This is clownshoes inline layout and is not even close to
        // correct.
        let y = 0;
        let mut x = 0, inline_available_width = *available_width;
        let mut current_height = 0;
        for tree::each_child(BTree, self) |kid| {
            kid.bounds.origin = Point2D(au(x), au(y));
            kid.reflow(au(inline_available_width));
            inline_available_width -= *kid.bounds.size.width;
            x += *kid.bounds.size.width;
            current_height = int::max(current_height, *kid.bounds.size.height);
        }

        self.bounds.size = Size2D(available_width, au(current_height));

        #debug["reflow_inline size=%?", copy self.bounds];
    }
}

