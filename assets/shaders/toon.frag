#version 450

layout(location=0)out vec4 o_Target;

layout(set=2,binding=0)uniform ToonMaterial {
    vec4 color;
};

layout(location=0)in vec3 normal;
layout(location=1) in vec3 FragPos;

void main(){

    vec3 lightColor=vec3(1.0, 1.0, 1.0);
    vec3 lightPos=vec3(15.,10.,-5.);
    // camera position
    vec3 viewPos=vec3(3.0, 5.0, -20.0);

    // Blinn-Phong lighting
    // Phong : https://learnopengl.com/Lighting/Basic-Lighting
    // Blinn-Phong : https://learnopengl.com/Advanced-Lighting/Advanced-Lighting

    // Toon stuff
    // See https://roystan.net/articles/toon-shader.html
    // TODO: implement with more than two bands (see here : https://roystan.net/articles/toon-shader.html)

    // ambient
    float ambientStrength=.1;
    vec3 ambient=ambientStrength*lightColor;

    // diffuse
    vec3 lightDir=normalize(lightPos-FragPos);
    vec3 viewDir=normalize(viewPos-FragPos);

    float diffuse=max(dot(normalize(normal),lightDir),0.);

    float lightIntensity=smoothstep(0,.01,diffuse);
    vec3 light=lightIntensity*lightColor;

    // Specular
    vec3 halfVector=normalize(lightPos+viewDir);
    float halfwayDiffuse=dot(normalize(normal),halfVector);

    float glossiness=32.;

    float specularIntensity=pow(halfwayDiffuse*lightIntensity,glossiness*glossiness);
    float specularIntensitySmooth=smoothstep(.005,.01,specularIntensity);
    float specular = specularIntensitySmooth;

    // Toon rim
    float rimAmount=.716;
    vec3 rimColor=vec3(0.,1.,0.);

    float rimDot=1.-dot(viewDir,normal);
    float rimIntensity=smoothstep(rimAmount-.01,rimAmount+.01,rimDot*diffuse);
    vec3 rim=rimIntensity*rimColor;

    o_Target=vec4(light + ambient + specular + rim, 1.0) * color;
}
