use bevy::asset::RenderAssetUsages;
use bevy::math::DVec3;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;

use super::scene::ActiveScene;
use crate::math::carmer::CramerSolution;
use crate::math::determinant::equation_to_matrix;
use crate::state::{AppliedEquation, CalculationState};

/// Seconds the play button needs to sweep `t` from 0 to 1.
pub const PLAYBACK_SECONDS: f32 = 5.0;

/// Faces of the parallelepiped in vertex-bit order: bit 0 = a1, 1 = a2, 2 = a3.
const BODY_FACES: [[usize; 4]; 6] = [
    [0, 1, 3, 2],
    [4, 5, 7, 6],
    [0, 1, 5, 4],
    [2, 3, 7, 6],
    [0, 2, 6, 4],
    [1, 3, 7, 5],
];

/// The same twelve edges as [`BODY_FACES`]; neighbours differ in exactly one bit.
const BODY_EDGES: [[usize; 2]; 12] = [
    [0, 1],
    [2, 3],
    [4, 5],
    [6, 7],
    [0, 2],
    [1, 3],
    [4, 6],
    [5, 7],
    [0, 4],
    [1, 5],
    [2, 6],
    [3, 7],
];

#[derive(Resource, Debug, Clone, Copy)]
pub struct VolumeSceneState {
    /// Index of the column that is replaced by `b`.
    pub column: usize,
    /// Animation cursor in `[0, 1]`.
    pub t: f32,
    pub playing: bool,
}

impl Default for VolumeSceneState {
    fn default() -> Self {
        Self {
            column: 0,
            t: 0.0,
            playing: false,
        }
    }
}

impl VolumeSceneState {
    pub fn select_column(&mut self, column: usize) {
        self.column = column.min(2);
        self.t = 0.0;
        self.playing = false;
    }

    pub fn restart(&mut self) {
        self.t = 0.0;
        self.playing = false;
    }

    /// Dragging the slider stops playback so the two inputs never fight.
    pub fn set_t(&mut self, t: f32) {
        self.t = t.clamp(0.0, 1.0);
        self.playing = false;
    }
}

#[derive(Component)]
pub struct VolumeSceneMarker;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum VolumeBody {
    Reference,
    Transformed,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AnimationStage {
    Scale,
    Shear,
}

impl AnimationStage {
    pub fn label(self) -> &'static str {
        match self {
            AnimationStage::Scale => "Scale",
            AnimationStage::Shear => "Shear",
        }
    }
}

pub fn stage_at(t: f32) -> AnimationStage {
    if t < 0.5 {
        AnimationStage::Scale
    } else {
        AnimationStage::Shear
    }
}

pub fn smoothstep(progress: f32) -> f32 {
    let p = progress.clamp(0.0, 1.0);
    p * p * (3.0 - 2.0 * p)
}

pub fn eased_stage_progress(t: f32) -> f32 {
    if t < 0.5 {
        smoothstep(t * 2.0)
    } else {
        smoothstep((t - 0.5) * 2.0)
    }
}

pub fn replaced_vector(column: DVec3, target: DVec3, xi: f64, t: f32) -> DVec3 {
    let eased = eased_stage_progress(t) as f64;
    let scaled = column * xi;
    if t < 0.5 {
        column.lerp(scaled, eased)
    } else {
        scaled.lerp(target, eased)
    }
}

pub fn triple_product(first: DVec3, second: DVec3, third: DVec3) -> f64 {
    first.cross(second).dot(third)
}

pub fn signed_volume(vectors: [DVec3; 3]) -> f64 {
    triple_product(vectors[0], vectors[1], vectors[2])
}

pub fn column_vectors(matrix: [[f64; 3]; 3]) -> [DVec3; 3] {
    std::array::from_fn(|column| {
        DVec3::new(matrix[0][column], matrix[1][column], matrix[2][column])
    })
}

pub fn rhs_vector(applied: &AppliedEquation) -> DVec3 {
    DVec3::new(
        applied.equations[0].d as f64,
        applied.equations[1].d as f64,
        applied.equations[2].d as f64,
    )
}

pub fn solution_vector(result: &CramerSolution) -> [f64; 3] {
    [result.x, result.y, result.z]
}

/// Columns with the animated column substituted.
pub fn current_columns(
    columns: [DVec3; 3],
    target: DVec3,
    x: [f64; 3],
    column: usize,
    t: f32,
) -> [DVec3; 3] {
    let mut current = columns;
    current[column] = replaced_vector(columns[column], target, x[column], t);
    current
}

pub fn volume_at(columns: [DVec3; 3], target: DVec3, x: [f64; 3], column: usize, t: f32) -> f64 {
    signed_volume(current_columns(columns, target, x, column, t))
}

pub fn body_vertices(vectors: [DVec3; 3]) -> [Vec3; 8] {
    std::array::from_fn(|index| {
        let mut corner = DVec3::ZERO;
        for (axis, vector) in vectors.iter().enumerate() {
            if index & (1 << axis) != 0 {
                corner += *vector;
            }
        }
        corner.as_vec3()
    })
}

fn vector_color(index: usize) -> Color {
    match index {
        0 => Color::srgb(1.0, 0.48, 0.42),
        1 => Color::srgb(0.36, 0.60, 1.0),
        _ => Color::srgb(0.24, 0.86, 0.72),
    }
}

fn faded_vector_color(index: usize) -> Color {
    match index {
        0 => Color::srgba(1.0, 0.48, 0.42, 0.45),
        1 => Color::srgba(0.36, 0.60, 1.0, 0.45),
        _ => Color::srgba(0.24, 0.86, 0.72, 0.45),
    }
}

/// Colors used by the panel legend, kept next to the gizmo palette.
pub fn legend_colors() -> [(Color, &'static str); 4] {
    [
        (vector_color(0), "a1"),
        (vector_color(1), "a2"),
        (vector_color(2), "a3"),
        (target_color(), "b"),
    ]
}

fn target_color() -> Color {
    Color::srgb(1.0, 0.78, 0.28)
}

fn new_body_mesh(vertices: &[Vec3; 8]) -> Mesh {
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    write_body_mesh(&mut mesh, vertices);

    let indices: Vec<u32> = (0..6)
        .flat_map(|face| {
            let base = (face * 4) as u32;
            [base, base + 1, base + 2, base, base + 2, base + 3]
        })
        .collect();
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

fn write_body_mesh(mesh: &mut Mesh, vertices: &[Vec3; 8]) {
    let mut positions = Vec::with_capacity(24);
    let mut normals = Vec::with_capacity(24);
    for face in BODY_FACES {
        let [p0, p1, p2, p3] = face.map(|index| vertices[index]);
        let normal = (p1 - p0).cross(p2 - p0).normalize_or_zero();
        positions.extend([p0, p1, p2, p3]);
        normals.extend([normal; 4]);
    }
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
}

pub fn advance_volume_animation(
    time: Res<Time>,
    active: Res<ActiveScene>,
    mut state: ResMut<VolumeSceneState>,
) {
    if *active != ActiveScene::Volume || !state.playing {
        return;
    }
    state.t = (state.t + time.delta_secs() / PLAYBACK_SECONDS).clamp(0.0, 1.0);
    if state.t >= 1.0 {
        state.playing = false;
    }
}

#[allow(clippy::too_many_arguments)]
pub fn sync_volume_scene(
    mut commands: Commands,
    active: Res<ActiveScene>,
    mut state: ResMut<VolumeSceneState>,
    applied: Res<AppliedEquation>,
    calculation: Res<CalculationState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    existing: Query<(Entity, &Mesh3d, &MeshMaterial3d<StandardMaterial>), With<VolumeSceneMarker>>,
) {
    if *active != ActiveScene::Volume {
        for (entity, mesh, material) in &existing {
            meshes.remove(&mesh.0);
            materials.remove(&material.0);
            commands.entity(entity).despawn();
        }
        return;
    }

    if !existing.is_empty() {
        return;
    }

    state.restart();

    let columns = column_vectors(equation_to_matrix(&applied));
    let target = rhs_vector(&applied);
    let solution = calculation.result.filter(CramerSolution::can_animate);
    let current = match solution {
        Some(result) => current_columns(
            columns,
            target,
            solution_vector(&result),
            state.column,
            state.t,
        ),
        None => columns,
    };
    let visibility = if solution.is_some() {
        Visibility::Inherited
    } else {
        Visibility::Hidden
    };

    let reference_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.55, 0.66, 0.85, 0.16),
        alpha_mode: AlphaMode::Blend,
        cull_mode: None,
        double_sided: true,
        unlit: true,
        ..default()
    });
    let transformed_material = materials.add(StandardMaterial {
        base_color: Color::srgba(1.0, 0.72, 0.24, 0.30),
        alpha_mode: AlphaMode::Blend,
        cull_mode: None,
        double_sided: true,
        unlit: true,
        ..default()
    });

    commands.spawn((
        Name::new("Volume reference body"),
        VolumeSceneMarker,
        VolumeBody::Reference,
        Mesh3d(meshes.add(new_body_mesh(&body_vertices(columns)))),
        MeshMaterial3d(reference_material),
        Transform::default(),
        visibility,
    ));
    commands.spawn((
        Name::new("Volume transformed body"),
        VolumeSceneMarker,
        VolumeBody::Transformed,
        Mesh3d(meshes.add(new_body_mesh(&body_vertices(current)))),
        MeshMaterial3d(transformed_material),
        Transform::default(),
        visibility,
    ));
}

pub fn update_volume_scene(
    active: Res<ActiveScene>,
    applied: Res<AppliedEquation>,
    calculation: Res<CalculationState>,
    state: Res<VolumeSceneState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut bodies: Query<(&VolumeBody, &Mesh3d, &mut Visibility)>,
    mut gizmos: Gizmos,
) {
    if *active != ActiveScene::Volume || bodies.is_empty() {
        return;
    }

    let Some(result) = calculation.result.filter(CramerSolution::can_animate) else {
        // Singular matrix: keep the scene empty and let the panel explain why.
        for (_, _, mut visibility) in &mut bodies {
            *visibility = Visibility::Hidden;
        }
        return;
    };

    let columns = column_vectors(equation_to_matrix(&applied));
    let target = rhs_vector(&applied);
    let x = solution_vector(&result);
    let current = current_columns(columns, target, x, state.column, state.t);
    let matrix_changed = applied.is_changed();

    for (body, mesh_handle, mut visibility) in &mut bodies {
        *visibility = Visibility::Inherited;
        let vertices = match body {
            VolumeBody::Reference => {
                if matrix_changed {
                    if let Some(mut mesh) = meshes.get_mut(&mesh_handle.0) {
                        write_body_mesh(&mut mesh, &body_vertices(columns));
                    }
                }
                continue;
            }
            VolumeBody::Transformed => body_vertices(current),
        };
        if let Some(mut mesh) = meshes.get_mut(&mesh_handle.0) {
            write_body_mesh(&mut mesh, &vertices);
        }
    }

    draw_overlay(&mut gizmos, columns, current, target, x, state.column);
}

fn draw_overlay(
    gizmos: &mut Gizmos,
    columns: [DVec3; 3],
    current: [DVec3; 3],
    target: DVec3,
    x: [f64; 3],
    column: usize,
) {
    let reference = body_vertices(columns);
    let moving = body_vertices(current);

    for [start, end] in BODY_EDGES {
        gizmos.line(
            reference[start],
            reference[end],
            Color::srgba(0.58, 0.68, 0.88, 0.55),
        );
        gizmos.line(
            moving[start],
            moving[end],
            Color::srgba(1.0, 0.78, 0.28, 0.85),
        );
    }

    for (index, vector) in columns.iter().enumerate() {
        let color = if index == column {
            // Ghost of the column that is being replaced.
            faded_vector_color(index)
        } else {
            vector_color(index)
        };
        gizmos.arrow(Vec3::ZERO, vector.as_vec3(), color);
    }

    gizmos.arrow(Vec3::ZERO, target.as_vec3(), target_color());
    gizmos.arrow(Vec3::ZERO, current[column].as_vec3(), vector_color(column));

    let scaled = (columns[column] * x[column]).as_vec3();
    dashed_line(
        gizmos,
        columns[column].as_vec3(),
        scaled,
        faded_vector_color(column),
    );
    dashed_line(
        gizmos,
        scaled,
        target.as_vec3(),
        Color::srgba(1.0, 0.78, 0.28, 0.45),
    );
}

fn dashed_line(gizmos: &mut Gizmos, start: Vec3, end: Vec3, color: Color) {
    const DASHES: usize = 12;
    let step = (end - start) / (DASHES as f32 * 2.0 - 1.0);
    if !step.is_finite() || step.length() <= f32::EPSILON {
        return;
    }
    for dash in 0..DASHES {
        let from = start + step * (dash * 2) as f32;
        gizmos.line(from, from + step, color);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::carmer::solver;

    fn reference() -> ([DVec3; 3], DVec3, [f64; 3]) {
        let applied = AppliedEquation::default();
        let matrix = equation_to_matrix(&applied);
        let target = rhs_vector(&applied);
        let solution = solver(matrix, [target.x, target.y, target.z]);
        assert!(solution.can_animate());
        (column_vectors(matrix), target, solution_vector(&solution))
    }

    #[test]
    fn volume_starts_at_det_and_ends_at_xi_times_det() {
        let (columns, target, x) = reference();
        let v0 = signed_volume(columns);
        assert!((v0 - 7.0).abs() < 1e-12);

        for column in 0..3 {
            assert!((volume_at(columns, target, x, column, 0.0) - v0).abs() < 1e-12);
            let expected = x[column] * v0;
            assert!((volume_at(columns, target, x, column, 0.5) - expected).abs() < 1e-12);
            assert!((volume_at(columns, target, x, column, 1.0) - expected).abs() < 1e-12);
            let mut replaced = equation_to_matrix(&AppliedEquation::default());
            for row in 0..3 {
                replaced[row][column] = [target.x, target.y, target.z][row];
            }
            let det_replaced = crate::math::determinant::det3(replaced);
            assert!((volume_at(columns, target, x, column, 1.0) - det_replaced).abs() < 1e-12);
        }
    }

    #[test]
    fn shear_stage_keeps_the_volume_constant() {
        let (columns, target, x) = reference();
        for column in 0..3 {
            let expected = x[column] * signed_volume(columns);
            for step in 0..=40 {
                let t = 0.5 + 0.5 * step as f32 / 40.0;
                assert!((volume_at(columns, target, x, column, t) - expected).abs() < 1e-12);
            }
        }
    }

    #[test]
    fn scale_stage_interpolates_volume_linearly_from_one_to_xi() {
        let (columns, target, x) = reference();
        let v0 = signed_volume(columns);
        for column in 0..3 {
            for step in 0..40 {
                let t = 0.5 * step as f32 / 40.0;
                let factor = 1.0 + (x[column] - 1.0) * eased_stage_progress(t) as f64;
                assert!((volume_at(columns, target, x, column, t) - factor * v0).abs() < 1e-12);
            }
        }
    }

    #[test]
    fn smoothstep_hits_its_endpoints() {
        assert_eq!(smoothstep(0.0), 0.0);
        assert_eq!(smoothstep(0.5), 0.5);
        assert_eq!(smoothstep(1.0), 1.0);
        assert_eq!(eased_stage_progress(0.0), 0.0);
        assert_eq!(eased_stage_progress(0.5), 0.0);
        assert_eq!(eased_stage_progress(1.0), 1.0);
    }

    fn scene_entities(app: &mut App) -> Vec<Entity> {
        let mut query = app
            .world_mut()
            .query_filtered::<Entity, With<VolumeSceneMarker>>();
        query.iter(app.world()).collect()
    }

    #[test]
    fn entities_are_spawned_on_entry_and_removed_on_exit() {
        let mut app = App::new();
        app.init_resource::<ActiveScene>()
            .init_resource::<VolumeSceneState>()
            .init_resource::<AppliedEquation>()
            .init_resource::<CalculationState>()
            .insert_resource(Assets::<Mesh>::default())
            .insert_resource(Assets::<StandardMaterial>::default())
            .add_systems(Update, sync_volume_scene);

        app.update();
        assert!(scene_entities(&mut app).is_empty());

        *app.world_mut().resource_mut::<ActiveScene>() = ActiveScene::Volume;
        app.update();
        assert_eq!(scene_entities(&mut app).len(), 2);
        assert_eq!(app.world().resource::<Assets<Mesh>>().len(), 2);

        *app.world_mut().resource_mut::<ActiveScene>() = ActiveScene::Cube;
        app.update();
        assert!(scene_entities(&mut app).is_empty());
        assert_eq!(app.world().resource::<Assets<Mesh>>().len(), 0);
        assert_eq!(app.world().resource::<Assets<StandardMaterial>>().len(), 0);

        *app.world_mut().resource_mut::<ActiveScene>() = ActiveScene::Volume;
        app.update();
        assert_eq!(scene_entities(&mut app).len(), 2);
    }

    #[test]
    fn corners_are_the_eight_signed_sums() {
        let (columns, target, x) = reference();
        let vertices = body_vertices(columns);
        assert!(vertices.iter().all(|vertex| vertex.is_finite()));
        assert!(vertices[0].length() < 1e-6);
        let expected = columns[0] + columns[1] + columns[2];
        assert!((vertices[7].as_dvec3() - expected).abs().max_element() < 1e-6);
        let replaced = body_vertices(current_columns(columns, target, x, 0, 1.0));
        let expected = target + columns[1] + columns[2];
        assert!((replaced[7].as_dvec3() - expected).abs().max_element() < 1e-6);
    }
}
