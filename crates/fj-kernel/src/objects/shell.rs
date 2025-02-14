use crate::{builder::ShellBuilder, storage::Handle};

use super::{face::FaceSet, Face, Objects};

/// A 3-dimensional closed shell
///
/// # Implementation Note
///
/// The faces that make up a shell should be closed ("watertight"). This is not
/// currently validated.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Shell {
    faces: FaceSet,
}

impl Shell {
    /// Build a `Shell` using [`ShellBuilder`]
    pub fn builder(objects: &Objects) -> ShellBuilder {
        ShellBuilder {
            objects,
            faces: FaceSet::new(),
        }
    }

    /// Construct an empty instance of `Shell`
    pub fn new(faces: impl IntoIterator<Item = Handle<Face>>) -> Self {
        Self {
            faces: faces.into_iter().collect(),
        }
    }

    /// Access the shell's faces
    pub fn faces(&self) -> &FaceSet {
        &self.faces
    }

    /// Find the given face in this shell
    pub fn find_face(&self, face: &Handle<Face>) -> Option<Handle<Face>> {
        self.faces().find(face)
    }
}
