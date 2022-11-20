#version 450

layout(location=0)in vec3 Vertex_Position;
layout(location=1)in vec3 Vertex_Normal;

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

layout(location=0)out vec3 Normal;
layout(location=1)out vec3 FragPos;

void main(){

    Normal=Vertex_Normal;

    FragPos=vec3(Model*vec4(Vertex_Position,1.));

    gl_Position=ViewProj*Model*vec4(Vertex_Position,1.);
}
