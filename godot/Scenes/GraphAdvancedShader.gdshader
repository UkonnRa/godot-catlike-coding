shader_type spatial;

// Properties to control the coloring
uniform vec4 color_a : source_color = vec4(1.0, 0.0, 0.0, 1.0);  // Red
uniform vec4 color_b : source_color = vec4(0.0, 1.0, 0.0, 1.0);  // Green
uniform float color_scale = 0.5;

// Properties for animation
uniform float time_scale = 1.0;
uniform int function_type = 0;

// This matches the function we implemented in Rust, but in the shader
float graph_function(vec2 position, float time) {
    float x = position.x;
    float z = position.y;

    if (function_type == 0) {
        // Basic sine wave with time offset (from tutorial)
        return sin(3.14159 * (x + time));
    } else if (function_type == 1) {
        // Multi-dimensional sine wave
        return sin(time + x + z) * 0.5;
    } else if (function_type == 2) {
        // Ripple effect
        float d = sqrt(x*x + z*z);
        return sin(4.0 * (d - time)) / (d * 4.0 + 2.0);
    } else if (function_type == 3) {
        // Combined waves
        float y1 = sin(x * 4.0 + time) * 0.2;
        float y2 = sin(z * 4.0 + time) * 0.2;
        return y1 + y2;
    }

    // Default
    return sin(time + x) * 0.5;
}

void vertex() {
    // Apply the function to modify vertex Y position
    // This way we can have the animation directly in the shader
    float time = TIME * time_scale;
    VERTEX.y = graph_function(VERTEX.xz, time);
}

void fragment() {
    // Get the normalized position (similar to the tutorial's approach)
    vec2 normalized_pos = VERTEX.xz * color_scale + 0.5;

    // Create a color gradient based on position
    // This is similar to the tutorial's approach of using worldPos components for coloring
    ALBEDO = mix(color_a.rgb, color_b.rgb, normalized_pos.x);

    // Use the y-position to influence the color as well
    float height_factor = (VERTEX.y + 1.0) * 0.5; // Normalize from [-1,1] to [0,1]
    ALBEDO = mix(ALBEDO, vec3(height_factor, height_factor, 1.0), 0.3);

    // Add some basic PBR properties
    METALLIC = 0.1;
    ROUGHNESS = 0.7;
}