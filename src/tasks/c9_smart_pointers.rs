// This chapter is dedicated to the smart pointers: Box, Rc and RefCell.

use std::cell::RefCell;
use std::rc::Rc;

// Box
// ================================================================================================

// ----- 1 --------------------------------------
// Implement a recursive `BinaryTreeNode` which have:
// - fields:
//   - `value: i32`
//   - `left_child: Option<BinaryTreeNode>`
//   - `right_child: Option<BinaryTreeNode>`
// - methods:
//   - `new(value: i32)`, which creates a note with provided value and without any children
//   - `with_children(value: i32, left_child: BinaryTreeNode, right_child: BinaryTreeNode)` which
//     creates a note using the provided values
//   - `sum(&self)` which computes the sum of all values in the tree
//
// Use `Box` if needed

// IMPLEMENT HERE:
pub struct BinaryTreeNode {
    value: i32,
    left_child: Option<Box<BinaryTreeNode>>,
    right_child: Option<Box<BinaryTreeNode>>,
}

impl BinaryTreeNode {
    pub fn new(value: i32) -> BinaryTreeNode {
        BinaryTreeNode {
            value,
            left_child: None,
            right_child: None,
        }
    }

    pub fn with_children(
        value: i32,
        left_child: BinaryTreeNode,
        right_child: BinaryTreeNode,
    ) -> BinaryTreeNode {
        BinaryTreeNode {
            value,
            left_child: Some(Box::new(left_child)),
            right_child: Some(Box::new(right_child)),
        }
    }

    pub fn sum(&self) -> i32 {
        self.value
            + self.left_child.as_ref().map_or(0, |node| node.sum())
            + self.right_child.as_ref().map_or(0, |node| node.sum())
    }
}

// Rc
// ================================================================================================

// ----- 2 --------------------------------------
// Implement a package dependency tree where multiple packages can depend on the same shared
// library.
//
// Implement the `Package` struct with `name: String` and `dependencies: Vec<Package>` fields.
// Implement methods:
// - `new(name: &str) -> Self` which creates a new package with provided name and without any
//   dependencies.
// - `with_dependencies(name: &str, dependencies: Vec<Package>) -> Self` which creates a new package
//   with provided name and dependencies.
// - `list_dependencies(package: Package) -> Vec<String>` which return a vector of all dependencies
//   of this package (including all recursive dependencies).
//
// Write a test which will reuse the created Packages in several other Packages as dependencies.
// Use `Rc` in the `Package` struct where needed to avoid deep clone.

// IMPLEMENT HERE:
pub struct Package {
    name: String,
    pub dependencies: Vec<Rc<Package>>,
}

impl Package {
    pub fn new(name: &str) -> Package {
        Package {
            name: name.to_string(),
            dependencies: Vec::new(),
        }
    }

    pub fn with_dependencies(name: &str, dependencies: Vec<Rc<Package>>) -> Package {
        Package { name: name.to_string(), dependencies }
    }

    pub fn list_dependencies(&self) -> Vec<String> {
        let mut dependencies = Vec::new();
        self.collect_dependencies(&mut dependencies);
        dependencies
    }

    fn collect_dependencies(&self, dependencies: &mut Vec<String>) {
        dependencies.push(self.name.clone());

        for dep in &self.dependencies {
            dep.collect_dependencies(dependencies);
        }
    }
}

#[test]
fn test_list_dependencies() {
    let package1 = Rc::new(Package::new("package1"));
    let package2 = Rc::new(Package::new("package2"));
    let package3 = Rc::new(Package::new("package3"));
    let package4 =
        Rc::new(Package::with_dependencies("package4", vec![package1.clone(), package2.clone()]));
    let package5 =
        Rc::new(Package::with_dependencies("package5", vec![package3.clone(), package4.clone()]));

    let mut dependencies = package5.list_dependencies();
    dependencies.sort();

    let mut expected = Vec::new();
    for i in 1..6 {
        expected.push(format!("package{i}").to_string());
    }

    assert_eq!(dependencies, expected);
}

// RefCell
// ================================================================================================

// ----- 3 --------------------------------------
// Create a simple `SharedCounter` where multiple owners can increment its value without mutable
// reference.
//
// Implement `new() -> Self` constructor, `increment(&self)` and `get(&self) -> i32` methods.
// Use `RefCell` where needed.

// IMPLEMENT HERE:
pub struct SharedCounter {
    value: RefCell<i32>,
}

impl SharedCounter {
    pub fn new() -> Self {
        SharedCounter { value: RefCell::new(0) }
    }

    pub fn increment(&self) {
        *self.value.borrow_mut() += 1;
    }

    pub fn get(&self) -> i32 {
        *self.value.borrow()
    }
}
