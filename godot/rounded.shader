shader_type spatial;
render_mode blend_mix,depth_draw_opaque,cull_back,diffuse_burley,specular_schlick_ggx;
uniform vec4 albedo : hint_color;
uniform sampler2D texture_albedo : hint_albedo;
uniform float specular;
uniform float metallic;
uniform float roughness : hint_range(0,1);
uniform float point_size : hint_range(0,128);
uniform vec3 uv1_scale;
uniform vec3 uv1_offset;
uniform vec3 uv2_scale;
uniform vec3 uv2_offset;
uniform float curve;

void vertex() {
	UV=UV*uv1_scale.xy+uv1_offset.xy;
 	vec4 camera_pos = (CAMERA_MATRIX * vec4(0.0, 0.0, 0.0, 1.0));
	vec4 world_vertex = (WORLD_MATRIX * vec4(VERTEX, 1.0));
	
	world_vertex.xyz -= camera_pos.xyz;
	world_vertex = vec4(0.0f, (world_vertex.z * world_vertex.z) * - curve, 0.0f, 0.0f);
	
	VERTEX += (WORLD_MATRIX * world_vertex).xyz;
	//VERTEX = (inverse(WORLD_MATRIX) * vec4(world_vertex, 1.0)).xyz;
}

void fragment() {
	vec2 base_uv = UV;
	vec4 albedo_tex = texture(texture_albedo,base_uv);
	ALBEDO = albedo.rgb * albedo_tex.rgb;
	METALLIC = metallic;
	ROUGHNESS = roughness;
	SPECULAR = specular;
}
