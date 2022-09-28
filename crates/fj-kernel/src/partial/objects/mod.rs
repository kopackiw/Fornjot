pub mod curve;
pub mod edge;
pub mod vertex;

use crate::{
    objects::{
        Curve, GlobalCurve, GlobalEdge, GlobalVertex, HalfEdge, SurfaceVertex,
        Vertex,
    },
    stores::{Handle, Stores},
};

use super::{
    HasPartial, MaybePartial, PartialCurve, PartialGlobalCurve,
    PartialGlobalEdge, PartialGlobalVertex, PartialHalfEdge,
    PartialSurfaceVertex, PartialVertex,
};

macro_rules! impl_traits {
    ($($full:ty, $partial:ty;)*) => {
        $(
            impl HasPartial for $full {
                type Partial = $partial;

                fn from_partial(partial: Self::Partial, stores: &Stores)
                    -> Self
                {
                    partial.build(stores)
                }
            }

            impl From<$partial> for MaybePartial<$full> {
                fn from(partial: $partial) -> Self {
                    Self::Partial(partial)
                }
            }
        )*
    };
}

impl_traits!(
    Curve, PartialCurve;
    GlobalEdge, PartialGlobalEdge;
    GlobalVertex, PartialGlobalVertex;
    HalfEdge, PartialHalfEdge;
    SurfaceVertex, PartialSurfaceVertex;
    Vertex, PartialVertex;

    Handle<GlobalCurve>, PartialGlobalCurve;
);
