use std::ffi::c_void;

use pj_sys::*;

use crate::prelude::AutoCreate;


pub struct PJRbTree { pub ctx: Box<*mut pj_rbtree> }
// pub struct PJRbTree

impl From<Box<*mut pj_rbtree>> for PJRbTree {
    fn from(ptr: Box<*mut pj_rbtree> ) -> Self {
        Self { ctx: ptr }
    }
}

impl PJRbTree {

    // void 	pj_rbtree_init (pj_rbtree *tree, pj_rbtree_comp *comp)
    pub fn init(comp: pj_rbtree_comp) -> Self {
        unsafe {
            let tree = Box::new(&mut pj_rbtree::new() as *mut _);
            pj_rbtree_init(*tree, comp);
            Self::from(tree)
        }
    }

    // pj_rbtree_node * 	pj_rbtree_first (pj_rbtree *tree)
    pub fn first(&self) -> Box<*mut pj_rbtree_node> {
        unsafe { Box::new(pj_rbtree_first(*self.ctx)) }
    }

    // pj_rbtree_node * 	pj_rbtree_last (pj_rbtree *tree)
    pub fn last(&self) -> Box<*mut pj_rbtree_node> {
        unsafe { Box::new(pj_rbtree_last(*self.ctx)) }
    }

    // pj_rbtree_node * 	pj_rbtree_next (pj_rbtree *tree, pj_rbtree_node *node)
    pub fn next(&self, node: Box<*mut pj_rbtree_node>) -> Box<*mut pj_rbtree_node> {
        unsafe { Box::new(pj_rbtree_next(*self.ctx, *node)) }
    }

    // pj_rbtree_node * 	pj_rbtree_prev (pj_rbtree *tree, pj_rbtree_node *node)
    pub fn prev(&self, node: Box<*mut pj_rbtree_node>) -> Box<*mut pj_rbtree_node> {
        unsafe { Box::new(pj_rbtree_next(*self.ctx, *node)) }
    }

    // int 	pj_rbtree_insert (pj_rbtree *tree, pj_rbtree_node *node)
    pub fn insert(&self, node: Box<*mut pj_rbtree_node>) -> i32 {
        unsafe { pj_rbtree_insert(*self.ctx, *node) }
    }

    // pj_rbtree_node * 	pj_rbtree_find (pj_rbtree *tree, const void *key)
    pub fn find(&self, key: Box<*const c_void>) -> Box<*mut pj_rbtree_node> {
        unsafe { Box::new(pj_rbtree_find(*self.ctx, *key)) }
    }

    // pj_rbtree_node * 	pj_rbtree_erase (pj_rbtree *tree, pj_rbtree_node *node)
    pub fn erase(&self, node: Box<*mut pj_rbtree_node>) -> Box<*mut pj_rbtree_node> {
        unsafe { Box::new(pj_rbtree_erase(*self.ctx, *node)) }
    }

    // unsigned 	pj_rbtree_max_height (pj_rbtree *tree, pj_rbtree_node *node)
    pub fn max_height(&self, node: Box<*mut pj_rbtree_node>) -> u32 {
        unsafe { pj_rbtree_max_height(*self.ctx, *node) }
    }

    // unsigned 	pj_rbtree_min_height (pj_rbtree *tree, pj_rbtree_node *node)
    pub fn min_height(&self, node: Box<*mut pj_rbtree_node>) -> u32 {
        unsafe { pj_rbtree_min_height(*self.ctx, *node) }
    }

}


