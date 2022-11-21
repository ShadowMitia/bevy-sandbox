#version 450

layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec3 Vertex_Normal;

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
    mat4 View;
    mat4 InverseView;
    mat4 Projection;
    vec3 WorldPosition;
    float width;
    float height;
};

layout(set = 2, binding = 0) uniform Mesh {
    mat4 Model;
    mat4 InverseTransposeModel;
    uint flags;
};

void main() {
  vec3 pos = Vertex_Position;

  if (pos.y > 0.0) {
    gl_Position = ViewProj * Model * vec4(pos, 1.0);
  } else {
   //pos.x = width - pos.x;
//    pos.y = -pos.y;
    gl_Position = ViewProj * Model * vec4(pos, 1.0);
    }
}
