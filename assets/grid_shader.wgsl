// grid_shader.wgsl
@group(0) @binding(0) var<uniform> time: f32;
@group(1) @binding(0) var<uniform> view_proj: mat4x4<f32>;

@vertex
fn vs_main(@location(0) position: vec3<f32>, @location(1) uv: vec2<f32>) -> @builtin(position) vec4<f32> {
    return view_proj * vec4<f32>(position, 1.0);
}

@fragment
fn fs_main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    // Define the grid
    let grid_size = 0.1; // Size of each grid square
    let line_width = 0.02; // Width of the grid lines
    let grid_x = step(line_width, fract(uv.x / grid_size));
    let grid_y = step(line_width, fract(uv.y / grid_size));

    // Combine both directions to form a grid
    let grid = grid_x * grid_y;

    // Return black for the lines and white for the background
    return vec4<f32>(vec3<f32>(grid), 1.0);
}
