use bevy::{prelude::*, render::view::NoFrustumCulling};
use bevy_inspector_egui::{Inspectable, RegisterInspectable};

use crate::instanced_cube::{InstanceData, InstanceMaterialData};

pub struct PointGenerationPlugin;

impl Plugin for PointGenerationPlugin {
    fn build(&self, app: &mut App) {
        // app.init_resource::<PointCloud>()
        app.register_inspectable::<PointCloud>()
            .add_startup_stage("point_cloud", SystemStage::parallel())
            .add_startup_system_to_stage("point_cloud", setup_point_cloud);
        // .add_startup_system_to_stage("point_cloud", draw_points.after(generate_points));
    }
}

#[derive(Component, Default, Inspectable)]
pub struct PointCloud {
    pub points: Vec<i32>,
    pub origin: Vec3,
    pub dimensions: UVec3,
}

impl PointCloud {
    pub fn new(origin: Vec3, width: usize, depth: usize, height: usize) -> Self {
        let len = width * depth * height;
        let mut points = Vec::with_capacity(len);

        let rng = fastrand::Rng::with_seed(1);

        for _ in 0..len {
            points.push(rng.i32(-16..=16));
        }

        PointCloud {
            points,
            origin,
            dimensions: UVec3::new(width as u32, height as u32, depth as u32),
        }
    }

    pub fn len(&self) -> usize {
        (self.dimensions.x * self.dimensions.y * self.dimensions.z) as usize
    }
}

fn setup_point_cloud(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let width: usize = 3;
    let depth: usize = 2;
    let height: usize = 1;
    // let len: usize = width * depth * height;

    let point_cloud = PointCloud::new(origin, width, depth, height);
    let point_cloud_render = PointCloudRender::new(origin, width, depth, height);

    commands
        .spawn()
        .insert_bundle((
            point_cloud,
            Visibility::visible(),
            ComputedVisibility::default(),
        ))
        .with_children(|parent| {
            parent
                .spawn_bundle(point_cloud_render)
                .insert(meshes.add(Mesh::from(shape::Cube { size: 1.1 })));
        });
}

#[derive(Bundle)]
pub struct PointCloudRender {
    #[bundle]
    pub transform_bundle: TransformBundle,
    pub instance_material_data: InstanceMaterialData,
    #[bundle]
    pub visibility_bundle: VisibilityBundle,
    pub no_frustum_culling: NoFrustumCulling,
}

impl PointCloudRender {
    pub fn new(origin: Vec3, width: usize, depth: usize, height: usize) -> Self {
        let len = width * depth * height;

        let mut instance_data = Vec::with_capacity(len);
        let spacing_scale = 0.3;

        for idx in 0..len {
            let x: f32 = (idx % width) as f32;
            let y: f32 = ((idx / width / depth) % height) as f32;
            let z: f32 = ((idx / width) % depth) as f32;

            instance_data.push(InstanceData {
                position: Vec3::new(x * spacing_scale, y * spacing_scale, z * spacing_scale),
                scale: 0.1,
                color: Color::hsla(
                    x / width as f32 * 360.,
                    z / depth as f32 * 0.7 + 0.3,
                    0.5,
                    1.0,
                )
                .as_rgba_f32(),
            });
        }

        PointCloudRender {
            transform_bundle: TransformBundle {
                local: Transform::from_xyz(origin.x, origin.y, origin.z),
                global: GlobalTransform::default(),
            },
            instance_material_data: InstanceMaterialData(instance_data),
            visibility_bundle: VisibilityBundle {
                visibility: Visibility::visible(),
                computed: ComputedVisibility::default(),
            },
            no_frustum_culling: NoFrustumCulling,
        }
    }
}

// fn draw_points(
//     point_cloud: Res<PointCloud>,
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
// ) {
//     let mut instance_data = Vec::with_capacity(point_cloud.points.len());
//     let spacing_scale = 0.3;

//     for (idx, _point) in point_cloud.points.iter().enumerate() {
//         let x: f32 = (idx % WIDTH) as f32;
//         let y: f32 = ((idx / WIDTH / DEPTH) % HEIGHT) as f32;
//         let z: f32 = ((idx / WIDTH) % DEPTH) as f32;

//         instance_data.push(InstanceData {
//             position: Vec3::new(x * spacing_scale, y * spacing_scale, z * spacing_scale),
//             scale: 0.1,
//             color: Color::hsla(
//                 x / WIDTH as f32 * 360.,
//                 z / DEPTH as f32 * 0.7 + 0.3,
//                 0.5,
//                 1.0,
//             )
//             .as_rgba_f32(),
//         });
//     }

//     let temp = meshes.add(Mesh::from(shape::Cube { size: 1.1 }));

//     commands.spawn().insert_bundle((
//         meshes.add(Mesh::from(shape::Cube { size: 1.1 })),
//         Transform::from_xyz(0.0, 0.0, 0.0),
//         GlobalTransform::default(),
//         InstanceMaterialData(instance_data),
//         Visibility::visible(),
//         ComputedVisibility::default(),
//         // NOTE: Frustum culling is done based on the Aabb of the Mesh and the GlobalTransform.
//         // As the cube is at the origin, if its Aabb moves outside the view frustum, all the
//         // instanced cubes will be culled.
//         // The InstanceMaterialData contains the 'GlobalTransform' information for this custom
//         // instancing, and that is not taken into account with the built-in frustum culling.
//         // We must disable the built-in frustum culling by adding the `NoFrustumCulling` marker
//         // component to avoid incorrect culling.
//         NoFrustumCulling,
//     ));
// }
