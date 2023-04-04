//! Create a 3D distance field by sweeping a 2D distance field
//! around the perimiter of another 2D distance field

use rust_gpu_bridge::glam::{Vec2, Vec3, Vec3Swizzles};
use type_fields::Field;

use crate::prelude::{AttrDistance, Field, FieldOperator, AttrNormal, Operator, AttrUv, items::position::Position, Normal, Uv};

/// Create a 3D distance field by sweeping a 2D distance field
/// around the perimiter of another 2D distance field
#[derive(Debug, Default, Copy, Clone, PartialEq, Field)]
#[cfg_attr(feature = "glam", derive(rust_gpu_bridge::Named))]
#[repr(C)]
pub struct SweepOp;

impl<Core, Shell> FieldOperator<(Core, Shell), AttrDistance<Vec2>> for SweepOp
where
    Core: Field<AttrDistance<f32>>,
    Shell: Field<AttrDistance<f32>>,
{
    fn operator(
        &self,
        (core, shell): &(Core, Shell),
        p: &Position<Vec2>,
    ) -> <AttrDistance<Vec2> as crate::prelude::Attribute>::Output {
        let q = *core.field(&p.x.into());
        shell.field(&q.into())
    }
}

impl<Core, Shell> FieldOperator<(Core, Shell), AttrDistance<Vec3>> for SweepOp
where
    Core: Field<AttrDistance<Vec2>>,
    Shell: Field<AttrDistance<Vec2>>,
{
    fn operator(
        &self,
        (core, shell): &(Core, Shell),
        p: &Position<Vec3>,
    ) -> <AttrDistance<Vec3> as crate::prelude::Attribute>::Output {
        let q = Vec2::new(*core.field(&p.truncate().into()), p.z);
        shell.field(&q.into())
    }
}

impl<Core, Shell> FieldOperator<(Core, Shell), AttrNormal<Vec3>> for SweepOp
where
    Core: Field<AttrDistance<Vec2>>,
    Shell: Field<AttrNormal<Vec2>>,
{
    fn operator(&self, (core, shell): &(Core, Shell), input: &Position<Vec3>) -> Normal<Vec3> {
        let q = Vec2::new(*core.field(&input.truncate().into()), input.z);
        let n = shell.field(&q.into());
        let w = input.xy().normalize() * n.x;
        Vec3::new(w.x, w.y, n.y).into()
    }
}

impl<Core, Shell> FieldOperator<(Core, Shell), AttrUv<Vec3>> for SweepOp
where
    Core: Field<AttrDistance<Vec2>> + Field<AttrUv<Vec2>>,
    Shell: Field<AttrUv<Vec2>>,
{
    fn operator(&self, (core, shell): &(Core, Shell), input: &Position<Vec3>) -> Uv {
        let dist_core = *Field::<AttrDistance<Vec2>>::field(core, &input.truncate().into());
        let uv_core = Field::<AttrUv<Vec2>>::field(core, &input.truncate().into());
        let q = Vec2::new(dist_core, input.z);
        let uv_shell = shell.field(&q.into());
        Vec2::new(uv_core.x, uv_shell.x + uv_shell.y).into()
    }
}

/// Uniformly scale a distance field.
pub type Sweep<Core, Shell> = Operator<SweepOp, (Core, Shell)>;

impl<Core, Shell> Sweep<Core, Shell> {
    pub fn core(&mut self) -> &mut Core {
        &mut self.target().0
    }

    pub fn shell(&mut self) -> &mut Shell {
        &mut self.target().1
    }
}

#[cfg(all(not(feature = "spirv-std"), test))]
pub mod tests {
    use crate::{
        prelude::{BoundTester, Circle, Point, Sweep},
        test_op_attrs_3d,
    };

    #[test]
    fn test_sweep() {
        assert!(BoundTester::<Sweep::<Circle, Circle>>::default().is_field_3d());
    }

    test_op_attrs_3d!(Sweep::<Point, Point>);
}
