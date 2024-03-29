{{include "rend3-routine/math/sphere.wgsl"}}

struct Object {
    transform: mat4x4<f32>,
    bounding_sphere: Sphere,
    first_index: u32,
    index_count: u32,
    material_index: u32,
    vertex_attribute_start_offsets: array<u32, {{vertex_array_counts}}>,
}

struct ObjectRange {
    invocation_start: u32,
    invocation_end: u32,
    object_id: u32,
}

struct BatchData {
    ranges: array<ObjectRange, 256>,
    total_objects: u32,
    total_invocations: u32,
    base_output_invocation: u32,
}
