#version 430 core

layout(location = 0) in vec3 position; // xyz
layout(location = 1) in vec4 vertexColor; // RGBA
uniform layout(location = 2) float elapsed;
out vec4 fragmentColor; // output to fragment shader

void main()
{
    // identity matrix
    mat4x4 identityMatrix = mat4(1);
    identityMatrix[1][1] = elapsed;

    // give color to fragment shader
    fragmentColor = vertexColor;
    
    // 
    gl_Position = identityMatrix * vec4(position, 1.0f);
}
