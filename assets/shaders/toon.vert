#version 450

layout(location=0)in vec3 Vertex_Position;
layout(location=1)in vec3 Vertex_Normal;

layout(set=0,binding=0)uniform Camera{
    mat4 ViewProj;
};

layout(set=1,binding=0)uniform Transform{
    mat4 Model;
};

layout(location=0)out vec3 normal;
layout(location=1)out vec3 FragPos;

void main(){
    
    normal=Vertex_Normal;
    
    FragPos=vec3(Model*vec4(Vertex_Position,1.));
    
    gl_Position=ViewProj*Model*vec4(Vertex_Position,1.);
}