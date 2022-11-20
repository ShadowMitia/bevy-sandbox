#version 450

layout(location=0)out vec4 o_Target;

layout(set=2,binding=0)uniform PhongMaterial{
    vec4 color;
};

layout(location=0)in vec3 Normal;
layout(location=1)in vec3 FragPos;

void main(){

    vec3 lightColor=vec3(1.,1.,1.);
    vec3 lightPos=vec3(15.,10.,-5.);

    // camera
    vec3 viewPos=vec3(3.0, 5.0, -20.0);

    // Blinn-Phong lighting
    // Phong : https://learnopengl.com/Lighting/Basic-Lighting
    // Blinn-Phong : https://learnopengl.com/Advanced-Lighting/Advanced-Lighting

    // ambient
    float ambientStrength=.1;
    vec3 ambient=ambientStrength*lightColor;

    // diffuse
    vec3 norm=normalize(Normal);
    vec3 lightDir=normalize(lightPos-FragPos);
    float diff=max(dot(norm,lightDir),0.);
    vec3 diffuse=diff*lightColor;

    // specular
    float specularStrength=.5;
    vec3 viewDir=normalize(viewPos-FragPos);
    vec3 reflectDir=reflect(-lightDir,norm);
    float spec=pow(max(dot(viewDir,reflectDir),0.),32);
    vec3 specular=specularStrength*spec*lightColor;

    vec3 result=(ambient+diffuse+specular)*color.rgb;

    vec4 out_color=vec4(ambient,1.)*color;

    o_Target=vec4(result, 1.0);
}
